#![allow(clippy::module_name_repetitions)]

mod sealed {
    pub trait Sealed {}

    macro_rules! impl_sealed {
        ($($ty:ty),* $(,)?) => {
            $(impl Sealed for $ty {})*
        };
    }

    impl_sealed!(
        u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
    );
}

/// Primitive integer types supported by the checked arithmetic wrappers.
pub trait CheckedArithmetic: sealed::Sealed + Sized {
    /// Returns `self + rhs`, or `None` on overflow.
    fn checked_add(self, rhs: Self) -> Option<Self>;

    /// Returns `self - rhs`, or `None` on overflow.
    fn checked_sub(self, rhs: Self) -> Option<Self>;

    /// Returns `self * rhs`, or `None` on overflow.
    fn checked_mul(self, rhs: Self) -> Option<Self>;
}

macro_rules! impl_checked_arithmetic {
    ($($ty:ty),* $(,)?) => {
        $(impl CheckedArithmetic for $ty {
            fn checked_add(self, rhs: Self) -> Option<Self> {
                <$ty>::checked_add(self, rhs)
            }

            fn checked_sub(self, rhs: Self) -> Option<Self> {
                <$ty>::checked_sub(self, rhs)
            }

            fn checked_mul(self, rhs: Self) -> Option<Self> {
                <$ty>::checked_mul(self, rhs)
            }
        })*
    };
}

impl_checked_arithmetic!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

/// Returns `left + right`, or `None` on overflow.
#[must_use]
pub fn checked_add<T: CheckedArithmetic>(left: T, right: T) -> Option<T> {
    <T as CheckedArithmetic>::checked_add(left, right)
}

/// Returns `left - right`, or `None` on overflow.
#[must_use]
pub fn checked_sub<T: CheckedArithmetic>(left: T, right: T) -> Option<T> {
    <T as CheckedArithmetic>::checked_sub(left, right)
}

/// Returns `left * right`, or `None` on overflow.
#[must_use]
pub fn checked_mul<T: CheckedArithmetic>(left: T, right: T) -> Option<T> {
    <T as CheckedArithmetic>::checked_mul(left, right)
}

#[cfg(test)]
mod tests {
    use super::{checked_add, checked_mul, checked_sub};

    #[test]
    fn reports_overflow_for_unsigned_values() {
        assert_eq!(checked_add(u8::MAX, 1), None);
        assert_eq!(checked_sub(0_u8, 1), None);
        assert_eq!(checked_mul(200_u8, 2), None);
    }

    #[test]
    fn reports_overflow_for_signed_values() {
        assert_eq!(checked_add(i8::MAX, 1), None);
        assert_eq!(checked_sub(i8::MIN, 1), None);
        assert_eq!(checked_mul(i8::MAX, 2), None);
    }
}
