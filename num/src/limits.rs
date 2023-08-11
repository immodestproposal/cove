/// Provides the maximum value of a type; for primitive types and std::num::NonZero* this is
/// guaranteed to match the associated standard MAX constant. This can be useful in a generic
/// context.
///
/// # Examples
/// ```
/// use cove_num::limits::Maximum;
/// use std::num::NonZeroIsize;
///
/// assert_eq!(u8::MAXIMUM, u8::MAX);
/// assert_eq!(f32::MAXIMUM, f32::MAX);
/// assert_eq!(NonZeroIsize::MAXIMUM, NonZeroIsize::MAX);
/// ```
pub trait Maximum {
    /// The largest value possible for this type
    const MAXIMUM: Self;
}

/// Provides the minimum value of a type; for primitive types and std::num::NonZero* this is
/// guaranteed to match the associated standard MIN constant. This can be useful in a generic
/// context.
///
/// # Examples
/// ```
/// use cove_num::limits::Minimum;
/// use std::num::NonZeroIsize;
///
/// assert_eq!(u8::MINIMUM, u8::MIN);
/// assert_eq!(f32::MINIMUM, f32::MIN);
/// assert_eq!(NonZeroIsize::MINIMUM, NonZeroIsize::MINIMUM);
/// ```
pub trait Minimum {
    /// The smallest value possible for this type
    const MINIMUM: Self;
}

// -- Implementation -- //
use std::num::*;

macro_rules! use_constants {
    ($($num:ty),+) => {
        $(
            impl Maximum for $num {
                const MAXIMUM: Self = <$num>::MAX;
            }

            impl Minimum for $num {
                const MINIMUM: Self = <$num>::MIN;
            }
        )*
    }
}

use_constants!(
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64,
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize
);