use {
    super::EvaluateError,
    crate::{
        data,
        data::value::{TryFromLiteral, Value},
        result::{Error, Result},
    },
    sqlparser::ast::{DataType, Value as Literal},
    std::{
        cmp::Ordering,
        convert::{TryFrom, TryInto},
    },
    Evaluated::*,
};

/// `LiteralRef` and `Literal` are used when it is not possible to specify what kind of `Value`
/// can be applied.
///
/// * `1 + 1` is converted into `LiteralRef + LiteralRef`, `LiteralRef` of `1` can
/// become `Value::I64` but it can be also `Value::F64`.
///
/// * Specifing column `id`, it is converted into `ValueRef` because `id` can be specified from table
/// schema.
///
/// * Evaluated result of `1 + 1` becomes `Literal`, not `LiteralRef` because executor has
/// ownership of `1 + 1`.
///
/// * Similar with `Literal`, `Value` is also generated by any other operation with `ValueRef` or
/// `Value`.
/// e.g. `LiteralRef` + `ValueRef`, `LiteralRef` * `Value`, ...
#[derive(std::fmt::Debug)]
pub enum Evaluated<'a> {
    LiteralRef(&'a Literal),
    Literal(Literal),
    ValueRef(&'a Value),
    Value(Value),
}

impl<'a> PartialEq for Evaluated<'a> {
    fn eq(&self, other: &Evaluated<'a>) -> bool {
        match (self, other) {
            (LiteralRef(l), LiteralRef(r)) => l == r,
            (LiteralRef(l), ValueRef(r)) => r == l,
            (LiteralRef(l), Value(r)) => &r == l,
            (LiteralRef(l), Literal(r)) => *l == r,
            (ValueRef(l), LiteralRef(r)) => l == r,
            (ValueRef(l), Literal(r)) => l == &r,
            (ValueRef(l), ValueRef(r)) => l == r,
            (ValueRef(l), Value(r)) => l == &r,
            (Value(l), LiteralRef(r)) => &l == r,
            (Value(l), ValueRef(r)) => &l == r,
            (Value(l), Value(r)) => l == r,
            (Value(l), Literal(r)) => l == r,
            (Literal(l), Literal(r)) => l == r,
            (Literal(l), LiteralRef(r)) => &l == r,
            (Literal(l), ValueRef(r)) => r == &l,
            (Literal(l), Value(r)) => r == l,
        }
    }
}

impl<'a> PartialOrd for Evaluated<'a> {
    fn partial_cmp(&self, other: &Evaluated<'a>) -> Option<Ordering> {
        match (self, other) {
            (LiteralRef(l), LiteralRef(r)) => literal_partial_cmp(l, r),
            (LiteralRef(l), Literal(r)) => literal_partial_cmp(&l, &r),
            (LiteralRef(l), ValueRef(r)) => r.partial_cmp(l).map(|o| o.reverse()),
            (LiteralRef(l), Value(r)) => r.partial_cmp(*l).map(|o| o.reverse()),
            (Literal(l), LiteralRef(r)) => literal_partial_cmp(&l, &r),
            (Literal(l), ValueRef(r)) => r.partial_cmp(&l).map(|o| o.reverse()),
            (Literal(l), Value(r)) => r.partial_cmp(l).map(|o| o.reverse()),
            (Literal(l), Literal(r)) => literal_partial_cmp(l, r),
            (ValueRef(l), LiteralRef(r)) => l.partial_cmp(r),
            (ValueRef(l), ValueRef(r)) => l.partial_cmp(r),
            (Value(l), Literal(r)) => l.partial_cmp(r),
            (Value(l), Value(r)) => l.partial_cmp(r),
            (ValueRef(l), Literal(r)) => l.partial_cmp(&r),
            (ValueRef(l), Value(r)) => l.partial_cmp(&r),
            (Value(l), LiteralRef(r)) => l.partial_cmp(*r),
            (Value(l), ValueRef(r)) => l.partial_cmp(*r),
        }
    }
}

fn literal_partial_cmp(l: &Literal, r: &Literal) -> Option<Ordering> {
    match (l, r) {
        (Literal::Number(l, false), Literal::Number(r, false)) => {
            match (l.parse::<i64>(), r.parse::<i64>()) {
                (Ok(l), Ok(r)) => Some(l.cmp(&r)),
                (_, Ok(r)) => match l.parse::<f64>() {
                    Ok(l) => l.partial_cmp(&(r as f64)),
                    _ => None,
                },
                (Ok(l), _) => match r.parse::<f64>() {
                    Ok(r) => (l as f64).partial_cmp(&r),
                    _ => None,
                },
                _ => match (l.parse::<f64>(), r.parse::<f64>()) {
                    (Ok(l), Ok(r)) => l.partial_cmp(&r),
                    _ => None,
                },
            }
        }
        (Literal::SingleQuotedString(l), Literal::SingleQuotedString(r)) => Some(l.cmp(r)),
        _ => None,
    }
}

impl TryInto<Value> for Evaluated<'_> {
    type Error = Error;

    fn try_into(self) -> Result<Value> {
        match self {
            Evaluated::LiteralRef(v) => Value::try_from(v),
            Evaluated::Literal(v) => Value::try_from(&v),
            Evaluated::ValueRef(v) => Ok(v.clone()),
            Evaluated::Value(v) => Ok(v),
        }
    }
}

macro_rules! binary_op {
    ($name:ident, $op:tt) => {
        pub fn $name(&self, other: &Evaluated<'a>) -> Result<Evaluated<'a>> {
            let literal_binary_op = |l: &Literal, r: &Literal| match (l, r) {
                (Literal::Number(l, false), Literal::Number(r, false)) => match (l.parse::<i64>(), r.parse::<i64>()) {
                    (Ok(l), Ok(r)) => Ok(Literal::Number((l $op r).to_string(), false)),
                    (Ok(l), _) => match r.parse::<f64>() {
                        Ok(r) => Ok(Literal::Number(((l as f64) $op r).to_string(), false)),
                        _ => Err(EvaluateError::UnreachableLiteralArithmetic.into()),
                    },
                    (_, Ok(r)) => match l.parse::<f64>() {
                        Ok(l) => Ok(Literal::Number((l $op (r as f64)).to_string(), false)),
                        _ => Err(EvaluateError::UnreachableLiteralArithmetic.into()),
                    },
                    (_, _) => match (l.parse::<f64>(), r.parse::<f64>()) {
                        (Ok(l), Ok(r)) => Ok(Literal::Number((l $op r).to_string(), false)),
                        _ => Err(EvaluateError::UnreachableLiteralArithmetic.into()),
                    },
                }.map(Evaluated::Literal),
                (Literal::Null, Literal::Number(_, false))
                | (Literal::Number(_, false), Literal::Null)
                | (Literal::Null, Literal::Null) => {
                    Ok(Evaluated::Literal(Literal::Null))
                }
                _ => Err(
                    EvaluateError::UnsupportedLiteralBinaryArithmetic(
                        l.to_string(),
                        r.to_string()
                    ).into()
                ),
            };

            let value_binary_op = |l: &data::Value, r: &data::Value| {
                l.$name(r).map(Evaluated::Value)
            };

            match (self, other) {
                (LiteralRef(l), LiteralRef(r)) => literal_binary_op(l, r),
                (LiteralRef(l), Literal(r))    => literal_binary_op(l, &r),
                (LiteralRef(l), ValueRef(r))   => value_binary_op(&data::Value::try_from(*l)?, r),
                (LiteralRef(l), Value(r))      => value_binary_op(&data::Value::try_from(*l)?, r),
                (Literal(l),    LiteralRef(r)) => literal_binary_op(&l, r),
                (Literal(l),    Literal(r))    => literal_binary_op(&l, &r),
                (Literal(l),    ValueRef(r))   => value_binary_op(&data::Value::try_from(l)?, r),
                (Literal(l),    Value(r))      => value_binary_op(&data::Value::try_from(l)?, r),
                (ValueRef(l),   LiteralRef(r)) => value_binary_op(l, &data::Value::try_from(*r)?),
                (ValueRef(l),   Literal(r))    => value_binary_op(l, &data::Value::try_from(r)?),
                (ValueRef(l),   ValueRef(r))   => value_binary_op(l, r),
                (ValueRef(l),   Value(r))      => value_binary_op(l, r),
                (Value(l),      LiteralRef(r)) => value_binary_op(l, &data::Value::try_from(*r)?),
                (Value(l),      Literal(r))    => value_binary_op(l, &data::Value::try_from(r)?),
                (Value(l),      ValueRef(r))   => value_binary_op(l, r),
                (Value(l),      Value(r))      => value_binary_op(l, r),
            }
        }
    };
}

macro_rules! unary_op {
    ($name:ident, $op:tt) => {
        pub fn $name(&self) -> Result<Evaluated<'a>> {
            let literal_unary_op = |v: &&Literal| match v {
                Literal::Number(v, false) => v
                    .parse::<i64>()
                    .map_or_else(
                        |_| v.parse::<f64>().map(|v| Literal::Number((0.0 $op v).to_string(), false)),
                        |v| Ok(Literal::Number((0 $op v).to_string(), false)),
                    )
                    .map_err(|_| EvaluateError::LiteralUnaryOperationOnNonNumeric.into()),
                Literal::Null => Ok(Literal::Null),
                _ => Err(EvaluateError::LiteralUnaryOperationOnNonNumeric.into()),
            };

            match self {
                LiteralRef(v) => literal_unary_op(v).map(Evaluated::Literal),
                Literal(v) => literal_unary_op(&v).map(Evaluated::Literal),
                ValueRef(v) => v.$name().map(Evaluated::Value),
                Value(v) => (&v).$name().map(Evaluated::Value),
            }
        }
    };
}

impl<'a> Evaluated<'a> {
    binary_op!(add, +);
    binary_op!(subtract, -);
    binary_op!(multiply, *);
    binary_op!(divide, /);
    unary_op!(unary_plus, +);
    unary_op!(unary_minus, -);

    pub fn cast(self, data_type: &DataType) -> Result<Evaluated<'a>> {
        let cast_literal = |literal: &Literal| Value::try_from_literal(data_type, literal);
        let cast_value = |value: &data::Value| value.cast(data_type);

        match self {
            LiteralRef(value) => cast_literal(value),
            Literal(value) => cast_literal(&value),
            ValueRef(value) => cast_value(value),
            Value(value) => cast_value(&value),
        }
        .map(Evaluated::Value)
    }

    pub fn is_some(&self) -> bool {
        match self {
            Evaluated::ValueRef(v) => v.is_some(),
            Evaluated::Value(v) => v.is_some(),
            Evaluated::Literal(v) => v != &Literal::Null,
            Evaluated::LiteralRef(v) => v != &&Literal::Null,
        }
    }
}
