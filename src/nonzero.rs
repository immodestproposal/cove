//! This module provides built-in implementation of the casting traits for core non-zero types

use crate::cast::{AssumeLossless, Cast, Closest, Lossless, Lossy, LossyCastError, Saturated};
use crate::base::CastImpl;

use core::num::{
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize
};

macro_rules! cast {
    ($($num:ty),+) => {
        $(
            impl Cast for $num {}
        )*
    };

    (to_nonzero $from:ty => $(($to:ty, $to_prim:ty)),+) => {
        $(
            impl CastImpl<$to> for $from {
                #[inline]
                fn cast_impl(self) -> Result<$to, LossyCastError<Self, $to>> {
                    // Safe to use `new_unchecked` because the value is coming from a non-zero so
                    // it cannot be zero itself.
                    self.try_into().map_err(|_| LossyCastError {
                        from: self,
                        to: unsafe {<$to>::new_unchecked(self.get() as $to_prim)}
                    })
                }
            }

            // impl Saturated<$to> for LossyCastError<$from, $to> {
            //     #[inline]
            //     fn saturated(self) -> $to {
            //         // Cast failed; if this is less than 0 use the target's MIN, otherwise
            //         // use its MAX. This logic cannot be used in general for saturation but
            //         // holds for all types actually fed to this macro. Note that the branch
            //         // will be optimized away for unsigned source types, at least in release.
            //         #[allow(unused_comparisons)]
            //         match self.from.get() < 0 {
            //             true => <$to>::MIN,
            //             false => <$to>::MAX
            //         }
            //     }
            // }
            //
            // impl Saturated<$to> for Result<$to, LossyCastError<$from, $to>> {
            //     #[inline]
            //     fn saturated(self) -> $to {
            //         self.unwrap_or_else(|error| error.saturated())
            //     }
            // }
            //
            // impl Closest<$to> for LossyCastError<$from, $to> {
            //     #[inline]
            //     fn closest(self) -> $to {
            //         // For int-to-int the closest is the saturated
            //         self.saturated()
            //     }
            // }
            //
            // impl Closest<$to> for Result<$to, LossyCastError<$from, $to>> {
            //     #[inline]
            //     fn closest(self) -> $to {
            //         self.unwrap_or_else(|error| error.closest())
            //     }
            // }
        )*
    };
}

cast!(
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize
);

cast!(
    to_nonzero NonZeroU8 =>
        (NonZeroU8, u8),    (NonZeroU16, u16),      (NonZeroU32, u32),
        (NonZeroU64, u64),  (NonZeroU128, u128),    (NonZeroUsize, usize),
        (NonZeroI8, i8),    (NonZeroI16, i16),      (NonZeroI32, i32),
        (NonZeroI64, i64),   (NonZeroI128, i128),    (NonZeroIsize, isize)
);

// cast!(to_nonzero u16 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
// cast!(to_nonzero u32 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f64);
// cast!(to_nonzero u64 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
// cast!(to_nonzero u128 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
// cast!(to_nonzero usize => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
//
// cast!(to_nonzero i8 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
// cast!(to_nonzero i16 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
// cast!(to_nonzero i32 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f64);
// cast!(to_nonzero i64 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
// cast!(to_nonzero i128 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
// cast!(to_nonzero isize => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);