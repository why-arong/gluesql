use {
    super::TryBinaryOperator,
    crate::{
        data::{NumericBinaryOperator, ValueError},
        prelude::Value,
        result::Result,
    },
    rust_decimal::prelude::Decimal,
    std::cmp::Ordering,
    Value::*,
};

impl PartialEq<Value> for i16 {
    fn eq(&self, other: &Value) -> bool {
        let lhs = *self;

        match other {
            I8(rhs) => lhs == (*rhs as i16),
            I16(rhs) => lhs == *rhs,
            I32(rhs) => (lhs as i32) == *rhs,
            I64(rhs) => (lhs as i64) == *rhs,
            I128(rhs) => (lhs as i128) == *rhs,
            F64(rhs) => ((lhs as f64) - rhs).abs() < f64::EPSILON,
            Decimal(rhs) => Decimal::from(lhs) == *rhs,
            _ => false,
        }
    }
}

impl PartialOrd<Value> for i16 {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        match other {
            I8(rhs) => self.partial_cmp(&(*rhs as i16)),
            I16(rhs) => self.partial_cmp(rhs),
            I32(rhs) => (*self as i32).partial_cmp(rhs),
            I64(rhs) => (*self as i64).partial_cmp(rhs),
            I128(rhs) => (*self as i128).partial_cmp(rhs),
            F64(rhs) => (*self as f64).partial_cmp(rhs),
            Decimal(rhs) => Decimal::from(*self).partial_cmp(rhs),
            _ => None,
        }
    }
}

impl TryBinaryOperator for i16 {
    type Rhs = Value;

    fn try_add(&self, rhs: &Self::Rhs) -> Result<Value> {
        let lhs = *self;

        match *rhs {
            I8(rhs) => lhs
                .checked_add(rhs as i16)
                .ok_or_else(|| {
                    ValueError::BinaryOperationOverflow {
                        lhs: I16(lhs),
                        rhs: I8(rhs),
                        operator: NumericBinaryOperator::Add,
                    }
                    .into()
                })
                .map(I16),
            I16(rhs) => lhs
                .checked_add(rhs)
                .ok_or_else(|| {
                    ValueError::BinaryOperationOverflow {
                        lhs: I16(lhs),
                        rhs: I16(rhs),
                        operator: NumericBinaryOperator::Add,
                    }
                    .into()
                })
                .map(I16),
            I32(rhs) => (lhs as i32)
                .checked_add(rhs)
                .ok_or_else(|| {
                    ValueError::BinaryOperationOverflow {
                        lhs: I16(lhs),
                        rhs: I32(rhs),
                        operator: NumericBinaryOperator::Add,
                    }
                    .into()
                })
                .map(I32),
            I64(rhs) => (lhs as i64)
                .checked_add(rhs)
                .ok_or_else(|| {
                    ValueError::BinaryOperationOverflow {
                        lhs: I16(lhs),
                        rhs: I64(rhs),
                        operator: NumericBinaryOperator::Add,
                    }
                    .into()
                })
                .map(I64),
            I128(rhs) => (lhs as i128)
                .checked_add(rhs)
                .ok_or_else(|| {
                    ValueError::BinaryOperationOverflow {
                        lhs: I16(lhs),
                        rhs: I128(rhs),
                        operator: NumericBinaryOperator::Add,
                    }
                    .into()
                })
                .map(I128),
            F64(rhs) => Ok(F64(lhs as f64 + rhs)),
            Decimal(rhs) => Ok(Decimal(Decimal::from(lhs) + rhs)),
            Null => Ok(Null),
            _ => Err(ValueError::NonNumericMathOperation {
                lhs: I16(lhs),
                operator: NumericBinaryOperator::Add,
                rhs: rhs.clone(),
            }
            .into()),
        }
    }

    fn try_subtract(&self, rhs: &Self::Rhs) -> Result<Value> {
        let lhs = *self;

        match *rhs {
            I8(rhs) => lhs
                .checked_sub(rhs as i16)
                .ok_or_else(|| {
                    ValueError::BinaryOperationOverflow {
                        lhs: I16(lhs),
                        rhs: I8(rhs),
                        operator: NumericBinaryOperator::Subtract,
                    }
                    .into()
                })
                .map(I16),
            I16(rhs) => lhs
                .checked_sub(rhs)
                .ok_or_else(|| {
                    ValueError::BinaryOperationOverflow {
                        lhs: I16(lhs),
                        rhs: I16(rhs),
                        operator: NumericBinaryOperator::Subtract,
                    }
                    .into()
                })
                .map(I16),
            I32(rhs) => (lhs as i32)
                .checked_sub(rhs)
                .ok_or_else(|| {
                    ValueError::BinaryOperationOverflow {
                        lhs: I16(lhs),
                        rhs: I32(rhs),
                        operator: NumericBinaryOperator::Subtract,
                    }
                    .into()
                })
                .map(I32),
            I64(rhs) => (lhs as i64)
                .checked_sub(rhs)
                .ok_or_else(|| {
                    ValueError::BinaryOperationOverflow {
                        lhs: I16(lhs),
                        rhs: I64(rhs),
                        operator: NumericBinaryOperator::Subtract,
                    }
                    .into()
                })
                .map(I64),
            I128(rhs) => (lhs as i128)
                .checked_sub(rhs)
                .ok_or_else(|| {
                    ValueError::BinaryOperationOverflow {
                        lhs: I16(lhs),
                        rhs: I128(rhs),
                        operator: NumericBinaryOperator::Subtract,
                    }
                    .into()
                })
                .map(I128),
            F64(rhs) => Ok(F64(lhs as f64 - rhs)),
            Decimal(rhs) => Decimal::from(lhs)
                .checked_sub(rhs)
                .ok_or_else(|| {
                    ValueError::BinaryOperationOverflow {
                        lhs: I16(lhs),
                        rhs: Decimal(rhs),
                        operator: NumericBinaryOperator::Subtract,
                    }
                    .into()
                })
                .map(Decimal),
            Null => Ok(Null),
            _ => Err(ValueError::NonNumericMathOperation {
                lhs: I16(lhs),
                operator: NumericBinaryOperator::Subtract,
                rhs: rhs.clone(),
            }
            .into()),
        }
    }

    fn try_multiply(&self, rhs: &Self::Rhs) -> Result<Value> {
        let lhs = *self;

        match *rhs {
            I8(rhs) => lhs
                .checked_mul(rhs as i16)
                .ok_or_else(|| {
                    ValueError::BinaryOperationOverflow {
                        lhs: I16(lhs),
                        rhs: I8(rhs),
                        operator: NumericBinaryOperator::Multiply,
                    }
                    .into()
                })
                .map(I16),
            I16(rhs) => lhs
                .checked_mul(rhs)
                .ok_or_else(|| {
                    ValueError::BinaryOperationOverflow {
                        lhs: I16(lhs),
                        rhs: I16(rhs),
                        operator: NumericBinaryOperator::Multiply,
                    }
                    .into()
                })
                .map(I16),
            I32(rhs) => (lhs as i32)
                .checked_mul(rhs)
                .ok_or_else(|| {
                    ValueError::BinaryOperationOverflow {
                        lhs: I16(lhs),
                        rhs: I32(rhs),
                        operator: NumericBinaryOperator::Multiply,
                    }
                    .into()
                })
                .map(I32),
            I64(rhs) => (lhs as i64)
                .checked_mul(rhs)
                .ok_or_else(|| {
                    ValueError::BinaryOperationOverflow {
                        lhs: I16(lhs),
                        rhs: I64(rhs),
                        operator: NumericBinaryOperator::Multiply,
                    }
                    .into()
                })
                .map(I64),
            I128(rhs) => (lhs as i128)
                .checked_mul(rhs)
                .ok_or_else(|| {
                    ValueError::BinaryOperationOverflow {
                        lhs: I16(lhs),
                        rhs: I128(rhs),
                        operator: NumericBinaryOperator::Multiply,
                    }
                    .into()
                })
                .map(I128),
            F64(rhs) => Ok(F64(lhs as f64 * rhs)),
            Decimal(rhs) => Decimal::from(lhs)
                .checked_mul(rhs)
                .ok_or_else(|| {
                    ValueError::BinaryOperationOverflow {
                        lhs: I16(lhs),
                        rhs: Decimal(rhs),
                        operator: NumericBinaryOperator::Multiply,
                    }
                    .into()
                })
                .map(Decimal),
            Interval(rhs) => Ok(Interval(lhs * rhs)),
            Null => Ok(Null),
            _ => Err(ValueError::NonNumericMathOperation {
                lhs: I16(lhs),
                operator: NumericBinaryOperator::Multiply,
                rhs: rhs.clone(),
            }
            .into()),
        }
    }

    fn try_divide(&self, rhs: &Self::Rhs) -> Result<Value> {
        let lhs = *self;

        match *rhs {
            I8(rhs) => lhs
                .checked_div(rhs as i16)
                .ok_or_else(|| {
                    ValueError::BinaryOperationOverflow {
                        lhs: I16(lhs),
                        rhs: I8(rhs),
                        operator: NumericBinaryOperator::Divide,
                    }
                    .into()
                })
                .map(I16),
            I16(rhs) => lhs
                .checked_div(rhs)
                .ok_or_else(|| {
                    ValueError::BinaryOperationOverflow {
                        lhs: I16(lhs),
                        rhs: I16(rhs),
                        operator: NumericBinaryOperator::Divide,
                    }
                    .into()
                })
                .map(I16),
            I32(rhs) => (lhs as i32)
                .checked_div(rhs)
                .ok_or_else(|| {
                    ValueError::BinaryOperationOverflow {
                        lhs: I16(lhs),
                        rhs: I32(rhs),
                        operator: NumericBinaryOperator::Divide,
                    }
                    .into()
                })
                .map(I32),
            I64(rhs) => (lhs as i64)
                .checked_div(rhs)
                .ok_or_else(|| {
                    ValueError::BinaryOperationOverflow {
                        lhs: I16(lhs),
                        rhs: I64(rhs),
                        operator: NumericBinaryOperator::Divide,
                    }
                    .into()
                })
                .map(I64),
            I128(rhs) => (lhs as i128)
                .checked_div(rhs)
                .ok_or_else(|| {
                    ValueError::BinaryOperationOverflow {
                        lhs: I16(lhs),
                        rhs: I128(rhs),
                        operator: NumericBinaryOperator::Divide,
                    }
                    .into()
                })
                .map(I128),
            F64(rhs) => Ok(F64(lhs as f64 / rhs)),
            Decimal(rhs) => Decimal::from(lhs)
                .checked_div(rhs)
                .ok_or_else(|| {
                    ValueError::BinaryOperationOverflow {
                        lhs: I16(lhs),
                        rhs: Decimal(rhs),
                        operator: NumericBinaryOperator::Divide,
                    }
                    .into()
                })
                .map(Decimal),
            Null => Ok(Null),
            _ => Err(ValueError::NonNumericMathOperation {
                lhs: I16(lhs),
                operator: NumericBinaryOperator::Divide,
                rhs: rhs.clone(),
            }
            .into()),
        }
    }

    fn try_modulo(&self, rhs: &Self::Rhs) -> Result<Value> {
        let lhs = *self;

        match *rhs {
            I8(rhs) => lhs
                .checked_rem(rhs as i16)
                .ok_or_else(|| {
                    ValueError::BinaryOperationOverflow {
                        lhs: I16(lhs),
                        rhs: I8(rhs),
                        operator: NumericBinaryOperator::Modulo,
                    }
                    .into()
                })
                .map(I16),
            I16(rhs) => lhs
                .checked_rem(rhs)
                .ok_or_else(|| {
                    ValueError::BinaryOperationOverflow {
                        lhs: I16(lhs),
                        rhs: I16(rhs),
                        operator: NumericBinaryOperator::Modulo,
                    }
                    .into()
                })
                .map(I16),
            I32(rhs) => (lhs as i32)
                .checked_rem(rhs)
                .ok_or_else(|| {
                    ValueError::BinaryOperationOverflow {
                        lhs: I16(lhs),
                        rhs: I32(rhs),
                        operator: NumericBinaryOperator::Modulo,
                    }
                    .into()
                })
                .map(I32),
            I64(rhs) => (lhs as i64)
                .checked_rem(rhs)
                .ok_or_else(|| {
                    ValueError::BinaryOperationOverflow {
                        lhs: I16(lhs),
                        rhs: I64(rhs),
                        operator: NumericBinaryOperator::Modulo,
                    }
                    .into()
                })
                .map(I64),
            I128(rhs) => (lhs as i128)
                .checked_rem(rhs)
                .ok_or_else(|| {
                    ValueError::BinaryOperationOverflow {
                        lhs: I16(lhs),
                        rhs: I128(rhs),
                        operator: NumericBinaryOperator::Modulo,
                    }
                    .into()
                })
                .map(I128),
            F64(rhs) => Ok(F64(lhs as f64 % rhs)),
            Decimal(rhs) => Decimal::from(lhs)
                .checked_rem(rhs)
                .ok_or_else(|| {
                    ValueError::BinaryOperationOverflow {
                        lhs: I16(lhs),
                        rhs: Decimal(rhs),
                        operator: NumericBinaryOperator::Modulo,
                    }
                    .into()
                })
                .map(Decimal),
            Null => Ok(Null),
            _ => Err(ValueError::NonNumericMathOperation {
                lhs: I16(lhs),
                operator: NumericBinaryOperator::Modulo,
                rhs: rhs.clone(),
            }
            .into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::{TryBinaryOperator, Value::*},
        crate::data::{NumericBinaryOperator, ValueError},
        rust_decimal::prelude::Decimal,
        std::cmp::Ordering,
    };

    #[test]
    fn test_extremes() {
        assert_eq!(-1i16, I16(-1));
        assert_eq!(0i16, I16(0));
        assert_eq!(1i16, I16(1));
        assert_eq!(i16::MAX, I16(i16::MAX));
        assert_eq!(i16::MIN, I16(i16::MIN));

        assert_eq!(
            i16::MAX.try_add(&I16(1)),
            Err(ValueError::BinaryOperationOverflow {
                lhs: I16(i16::MAX),
                rhs: I16(1),
                operator: (NumericBinaryOperator::Add)
            }
            .into())
        );

        assert_eq!(i16::MAX.try_add(&I32(1)), Ok(I32(i16::MAX as i32 + 1)));
        assert_eq!(i16::MAX.try_add(&I64(1)), Ok(I64(i16::MAX as i64 + 1)));
        assert_eq!(i16::MAX.try_add(&I128(1)), Ok(I128(i16::MAX as i128 + 1)));

        assert_eq!(
            i16::MAX.try_add(&I16(i16::MAX)),
            Err(ValueError::BinaryOperationOverflow {
                lhs: I16(i16::MAX),
                rhs: I16(i16::MAX),
                operator: (NumericBinaryOperator::Add)
            }
            .into())
        );

        assert_eq!(
            i16::MAX.try_add(&I32(i32::MAX)),
            Err(ValueError::BinaryOperationOverflow {
                lhs: I16(i16::MAX),
                rhs: I32(i32::MAX),
                operator: (NumericBinaryOperator::Add)
            }
            .into())
        );

        assert_eq!(
            i16::MAX.try_add(&I64(i64::MAX)),
            Err(ValueError::BinaryOperationOverflow {
                lhs: I16(i16::MAX),
                rhs: I64(i64::MAX),
                operator: (NumericBinaryOperator::Add)
            }
            .into())
        );

        assert_eq!(
            i16::MAX.try_add(&I128(i128::MAX)),
            Err(ValueError::BinaryOperationOverflow {
                lhs: I16(i16::MAX),
                rhs: I128(i128::MAX),
                operator: (NumericBinaryOperator::Add)
            }
            .into())
        );

        assert_eq!(
            i16::MIN.try_subtract(&I8(1)),
            Err(ValueError::BinaryOperationOverflow {
                lhs: I16(i16::MIN),
                rhs: I8(1),
                operator: (NumericBinaryOperator::Subtract)
            }
            .into())
        );

        assert_eq!(
            i16::MIN.try_subtract(&I16(1)),
            Err(ValueError::BinaryOperationOverflow {
                lhs: I16(i16::MIN),
                rhs: I16(1),
                operator: (NumericBinaryOperator::Subtract)
            }
            .into())
        );

        // these dont overflow since they are not i8 (ie, i32, i64, i128)
        assert_eq!(i16::MIN.try_subtract(&I32(1)), Ok(I32(i16::MIN as i32 - 1)));
        assert_eq!(i16::MIN.try_subtract(&I64(1)), Ok(I64(i16::MIN as i64 - 1)));
        assert_eq!(
            i16::MIN.try_subtract(&I128(1)),
            Ok(I128(i16::MIN as i128 - 1))
        );

        assert_eq!(
            i16::MIN.try_subtract(&I16(i16::MAX)),
            Err(ValueError::BinaryOperationOverflow {
                lhs: I16(i16::MIN),
                rhs: I16(i16::MAX),
                operator: (NumericBinaryOperator::Subtract)
            }
            .into())
        );
        assert_eq!(
            i16::MIN.try_subtract(&I32(i32::MAX)),
            Err(ValueError::BinaryOperationOverflow {
                lhs: I16(i16::MIN),
                rhs: I32(i32::MAX),
                operator: (NumericBinaryOperator::Subtract)
            }
            .into())
        );
        assert_eq!(
            i16::MIN.try_subtract(&I64(i64::MAX)),
            Err(ValueError::BinaryOperationOverflow {
                lhs: I16(i16::MIN),
                rhs: I64(i64::MAX),
                operator: (NumericBinaryOperator::Subtract)
            }
            .into())
        );
        assert_eq!(
            i16::MIN.try_subtract(&I128(i128::MAX)),
            Err(ValueError::BinaryOperationOverflow {
                lhs: I16(i16::MIN),
                rhs: I128(i128::MAX),
                operator: (NumericBinaryOperator::Subtract)
            }
            .into())
        );

        assert_eq!(i16::MAX.try_multiply(&I8(1)), Ok(I16(i16::MAX)));
        assert_eq!(i16::MAX.try_multiply(&I16(1)), Ok(I16(i16::MAX)));
        assert_eq!(i16::MAX.try_multiply(&I32(1)), Ok(I32(i16::MAX as i32)));
        assert_eq!(
            i16::MIN.try_subtract(&I128(i128::MAX)),
            Err(ValueError::BinaryOperationOverflow {
                lhs: I16(i16::MIN),
                rhs: I128(i128::MAX),
                operator: (NumericBinaryOperator::Subtract)
            }
            .into())
        );

        assert_eq!(i16::MAX.try_multiply(&I8(1)), Ok(I16(i16::MAX)));
        assert_eq!(i16::MAX.try_multiply(&I16(1)), Ok(I16(i16::MAX)));
        assert_eq!(i16::MAX.try_multiply(&I32(1)), Ok(I32(i16::MAX as i32)));
        assert_eq!(i16::MAX.try_multiply(&I64(1)), Ok(I64(i16::MAX as i64)));
        assert_eq!(i16::MAX.try_multiply(&I128(1)), Ok(I128(i16::MAX as i128)));

        assert_eq!(
            i16::MAX.try_multiply(&I8(2)),
            Err(ValueError::BinaryOperationOverflow {
                lhs: I16(i16::MAX),
                rhs: I8(2),
                operator: (NumericBinaryOperator::Multiply)
            }
            .into())
        );
        assert_eq!(
            i16::MAX.try_multiply(&I16(2)),
            Err(ValueError::BinaryOperationOverflow {
                lhs: I16(i16::MAX),
                rhs: I16(2),
                operator: (NumericBinaryOperator::Multiply)
            }
            .into())
        );
        assert_eq!(
            2i8.try_multiply(&I32(i32::MAX)),
            Err(ValueError::BinaryOperationOverflow {
                lhs: I16(2),
                rhs: I32(i32::MAX),
                operator: (NumericBinaryOperator::Multiply)
            }
            .into())
        );
        assert_eq!(
            2i8.try_multiply(&I64(i64::MAX)),
            Err(ValueError::BinaryOperationOverflow {
                lhs: I16(2),
                rhs: I64(i64::MAX),
                operator: (NumericBinaryOperator::Multiply)
            }
            .into())
        );
        assert_eq!(
            2i8.try_multiply(&I128(i128::MAX)),
            Err(ValueError::BinaryOperationOverflow {
                lhs: I16(2),
                rhs: I128(i128::MAX),
                operator: (NumericBinaryOperator::Multiply)
            }
            .into())
        );

        assert_eq!(
            2i8.try_multiply(&I128(i128::MAX)),
            Err(ValueError::BinaryOperationOverflow {
                lhs: I16(2),
                rhs: I128(i128::MAX),
                operator: (NumericBinaryOperator::Multiply)
            }
            .into())
        );

        //try_divide
        assert_eq!(
            i16::MAX.try_divide(&I8(0)),
            Err(ValueError::BinaryOperationOverflow {
                lhs: I16(i16::MAX),
                rhs: I8(0),
                operator: (NumericBinaryOperator::Divide)
            }
            .into())
        );
        assert_eq!(
            i16::MAX.try_divide(&I16(0)),
            Err(ValueError::BinaryOperationOverflow {
                lhs: I16(i16::MAX),
                rhs: I16(0),
                operator: (NumericBinaryOperator::Divide)
            }
            .into())
        );
        assert_eq!(
            i16::MAX.try_divide(&I32(0)),
            Err(ValueError::BinaryOperationOverflow {
                lhs: I16(i16::MAX),
                rhs: I32(0),
                operator: (NumericBinaryOperator::Divide)
            }
            .into())
        );
        assert_eq!(
            i16::MAX.try_divide(&I64(0)),
            Err(ValueError::BinaryOperationOverflow {
                lhs: I16(i16::MAX),
                rhs: I64(0),
                operator: (NumericBinaryOperator::Divide)
            }
            .into())
        );
        assert_eq!(
            i16::MAX.try_divide(&I128(0)),
            Err(ValueError::BinaryOperationOverflow {
                lhs: I16(i16::MAX),
                rhs: I128(0),
                operator: (NumericBinaryOperator::Divide)
            }
            .into())
        );

        assert_eq!(
            i16::MAX.try_modulo(&I8(0)),
            Err(ValueError::BinaryOperationOverflow {
                lhs: I16(i16::MAX),
                rhs: I8(0),
                operator: (NumericBinaryOperator::Modulo)
            }
            .into())
        );
        assert_eq!(
            i16::MAX.try_modulo(&I16(0)),
            Err(ValueError::BinaryOperationOverflow {
                lhs: I16(i16::MAX),
                rhs: I16(0),
                operator: (NumericBinaryOperator::Modulo)
            }
            .into())
        );
        assert_eq!(
            i16::MAX.try_modulo(&I32(0)),
            Err(ValueError::BinaryOperationOverflow {
                lhs: I16(i16::MAX),
                rhs: I32(0),
                operator: (NumericBinaryOperator::Modulo)
            }
            .into())
        );
        assert_eq!(
            i16::MAX.try_modulo(&I64(0)),
            Err(ValueError::BinaryOperationOverflow {
                lhs: I16(i16::MAX),
                rhs: I64(0),
                operator: (NumericBinaryOperator::Modulo)
            }
            .into())
        );
        assert_eq!(
            i16::MAX.try_modulo(&I128(0)),
            Err(ValueError::BinaryOperationOverflow {
                lhs: I16(i16::MAX),
                rhs: I128(0),
                operator: (NumericBinaryOperator::Modulo)
            }
            .into())
        );
    }

    #[test]
    fn eq() {
        let base = 1_i16;

        assert_eq!(base, I8(1));
        assert_eq!(base, I16(1));
        assert_eq!(base, I32(1));
        assert_eq!(base, I64(1));
        assert_eq!(base, I128(1));
        assert_eq!(base, F64(1.0));
        assert_eq!(base, Decimal(Decimal::ONE));

        assert_ne!(base, Bool(true));
    }

    #[test]
    fn partial_cmp() {
        let base = 1_i16;

        assert_eq!(base.partial_cmp(&I8(0)), Some(Ordering::Greater));
        assert_eq!(base.partial_cmp(&I16(0)), Some(Ordering::Greater));
        assert_eq!(base.partial_cmp(&I32(0)), Some(Ordering::Greater));
        assert_eq!(base.partial_cmp(&I64(0)), Some(Ordering::Greater));
        assert_eq!(base.partial_cmp(&I128(0)), Some(Ordering::Greater));
        assert_eq!(base.partial_cmp(&F64(0.0)), Some(Ordering::Greater));

        assert_eq!(base.partial_cmp(&I8(1)), Some(Ordering::Equal));
        assert_eq!(base.partial_cmp(&I16(1)), Some(Ordering::Equal));
        assert_eq!(base.partial_cmp(&I32(1)), Some(Ordering::Equal));
        assert_eq!(base.partial_cmp(&I64(1)), Some(Ordering::Equal));
        assert_eq!(base.partial_cmp(&I128(1)), Some(Ordering::Equal));
        assert_eq!(base.partial_cmp(&F64(1.0)), Some(Ordering::Equal));

        assert_eq!(base.partial_cmp(&I8(2)), Some(Ordering::Less));
        assert_eq!(base.partial_cmp(&I16(2)), Some(Ordering::Less));
        assert_eq!(base.partial_cmp(&I32(2)), Some(Ordering::Less));
        assert_eq!(base.partial_cmp(&I64(2)), Some(Ordering::Less));
        assert_eq!(base.partial_cmp(&I128(2)), Some(Ordering::Less));
        assert_eq!(base.partial_cmp(&F64(2.0)), Some(Ordering::Less));

        assert_eq!(
            base.partial_cmp(&Decimal(Decimal::ONE)),
            Some(Ordering::Equal)
        );

        assert_eq!(base.partial_cmp(&Bool(true)), None);
    }

    #[test]
    fn try_add() {
        let base = 1_i16;

        assert_eq!(base.try_add(&I8(1)), Ok(I16(2)));
        assert_eq!(base.try_add(&I16(1)), Ok(I16(2)));
        assert_eq!(base.try_add(&I32(1)), Ok(I32(2)));
        assert_eq!(base.try_add(&I64(1)), Ok(I64(2)));
        assert_eq!(base.try_add(&I128(1)), Ok(I128(2)));

        assert!(matches!(base.try_add(&F64(1.0)), Ok(F64(x)) if (x - 2.0).abs() < f64::EPSILON));
        assert_eq!(
            base.try_add(&Decimal(Decimal::ONE)),
            Ok(Decimal(Decimal::TWO))
        );

        assert_eq!(
            base.try_add(&Bool(true)),
            Err(ValueError::NonNumericMathOperation {
                lhs: I16(base),
                operator: NumericBinaryOperator::Add,
                rhs: Bool(true)
            }
            .into())
        );
    }

    #[test]
    fn try_subtract() {
        let base = 1_i16;

        assert_eq!(base.try_subtract(&I8(1)), Ok(I16(0)));
        assert_eq!(base.try_subtract(&I16(1)), Ok(I16(0)));
        assert_eq!(base.try_subtract(&I32(1)), Ok(I32(0)));
        assert_eq!(base.try_subtract(&I64(1)), Ok(I64(0)));
        assert_eq!(base.try_subtract(&I128(1)), Ok(I128(0)));

        assert!(
            matches!(base.try_subtract(&F64(1.0)), Ok(F64(x)) if (x - 0.0).abs() < f64::EPSILON )
        );

        assert_eq!(
            base.try_subtract(&Decimal(Decimal::ONE)),
            Ok(Decimal(Decimal::ZERO))
        );

        assert_eq!(
            base.try_subtract(&Bool(true)),
            Err(ValueError::NonNumericMathOperation {
                lhs: I16(base),
                operator: NumericBinaryOperator::Subtract,
                rhs: Bool(true)
            }
            .into())
        );
    }

    #[test]
    fn try_multiply() {
        let base = 3_i16;

        // 3 * 2 = 6
        assert_eq!(base.try_multiply(&I8(2)), Ok(I16(6)));
        assert_eq!(base.try_multiply(&I16(2)), Ok(I16(6)));
        assert_eq!(base.try_multiply(&I32(2)), Ok(I32(6)));
        assert_eq!(base.try_multiply(&I64(2)), Ok(I64(6)));
        assert_eq!(base.try_multiply(&I128(2)), Ok(I128(6)));

        assert_eq!(base.try_multiply(&I8(-1)), Ok(I8(-3)));
        assert_eq!(base.try_multiply(&I32(-1)), Ok(I32(-3)));
        assert_eq!(base.try_multiply(&I64(-1)), Ok(I64(-3)));
        assert_eq!(base.try_multiply(&I128(-1)), Ok(I128(-3)));

        assert!(
            matches!(base.try_multiply(&F64(1.0)), Ok(F64(x)) if (x - 3.0).abs() < f64::EPSILON )
        );

        let _result: Decimal = Decimal::from(3);
        assert_eq!(
            base.try_multiply(&Decimal(Decimal::ONE)),
            Ok(Decimal(_result))
        );

        assert_eq!(
            base.try_multiply(&Bool(true)),
            Err(ValueError::NonNumericMathOperation {
                lhs: I16(base),
                operator: NumericBinaryOperator::Multiply,
                rhs: Bool(true)
            }
            .into())
        );
    }

    #[test]
    fn try_divide() {
        let base = 6_i16;

        assert_eq!(base.try_divide(&I8(2)), Ok(I16(3)));
        assert_eq!(base.try_divide(&I16(2)), Ok(I16(3)));
        assert_eq!(base.try_divide(&I32(2)), Ok(I32(3)));
        assert_eq!(base.try_divide(&I64(2)), Ok(I64(3)));
        assert_eq!(base.try_divide(&I128(2)), Ok(I128(3)));

        assert_eq!(base.try_divide(&I8(-6)), Ok(I8(-1)));
        assert_eq!(base.try_divide(&I32(-6)), Ok(I32(-1)));
        assert_eq!(base.try_divide(&I64(-6)), Ok(I64(-1)));
        assert_eq!(base.try_divide(&I128(-6)), Ok(I128(-1)));

        assert!(
            matches!(base.try_divide(&F64(1.0)), Ok(F64(x)) if (x - 6.0).abs() < f64::EPSILON )
        );

        let _decimal_result = Decimal::from(base);
        assert_eq!(
            base.try_divide(&Decimal(Decimal::ONE)),
            Ok(Decimal(_decimal_result))
        );

        assert_eq!(
            base.try_divide(&Bool(true)),
            Err(ValueError::NonNumericMathOperation {
                lhs: I16(base),
                operator: NumericBinaryOperator::Divide,
                rhs: Bool(true)
            }
            .into())
        );
    }

    #[test]
    fn try_modulo() {
        let base = 9_i16;

        assert_eq!(base.try_modulo(&I8(1)), Ok(I16(0)));
        assert_eq!(base.try_modulo(&I16(1)), Ok(I16(0)));
        assert_eq!(base.try_modulo(&I32(1)), Ok(I32(0)));
        assert_eq!(base.try_modulo(&I64(1)), Ok(I64(0)));
        assert_eq!(base.try_modulo(&I128(1)), Ok(I128(0)));

        assert_eq!(base.try_modulo(&I8(2)), Ok(I16(1)));
        assert_eq!(base.try_modulo(&I16(2)), Ok(I16(1)));
        assert_eq!(base.try_modulo(&I32(2)), Ok(I32(1)));
        assert_eq!(base.try_modulo(&I64(2)), Ok(I64(1)));
        assert_eq!(base.try_modulo(&I128(2)), Ok(I128(1)));

        assert!(matches!(base.try_modulo(&F64(1.0)), Ok(F64(x)) if (x).abs() < f64::EPSILON ));
        assert_eq!(
            base.try_modulo(&Decimal(Decimal::ONE)),
            Ok(Decimal(Decimal::ZERO))
        );

        assert_eq!(
            base.try_modulo(&Bool(true)),
            Err(ValueError::NonNumericMathOperation {
                lhs: I16(base),
                operator: NumericBinaryOperator::Modulo,
                rhs: Bool(true)
            }
            .into())
        );
    }
}