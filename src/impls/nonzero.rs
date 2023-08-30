//! This module provides built-in implementation of the casting traits for core non-zero types

#![allow(clippy::wildcard_imports)]

use crate::cast::{Cast, Closest, FailedCastError, LossyCastError, Saturated};
use crate::base::CastImpl;
use super::LosslessCast;

use core::num::{
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize
};

// trait NonZeroMapping {
//     type Primitive;
// }
//
// macro_rules! assign {
//     ($($nonzero:ty => $primitive:ty),+) => {
//         $(
//             impl NonZeroMapping for $nonzero {
//                 type Primitive = $primitive;
//             }
//         )*
//     }
// }

// assign!(
//     NonZeroU8 => u8, NonZeroU16 => u16, NonZeroU32 => u32,
//     NonZeroU64 => u64, NonZeroU128 => u128, NonZeroUsize => usize,
//     NonZeroI8 => i8, NonZeroI16 => i16, NonZeroI32 => i32,
//     NonZeroI64 => i64, NonZeroI128 => i128, NonZeroIsize => isize
// );

macro_rules! cast {
    ($($nonzero:ty),+; $($primitive:ty),+) => {
        $(
            impl Cast for $nonzero {}
        )*

        // Generate the nonzero to nonzero implementations in n-squared fashion
        cast!(to_nonzero $($nonzero),* => ($($nonzero),*));

        // Generate the nonzero to primitive implementations in n*m fashion
        cast!(to_primitive $($nonzero),* => ($($primitive), *));

        // Generate the primitive to nonzero implementations in n*m fashion
        cast!(from_primitive $($primitive),* => ($($nonzero), *));
    };

    (to_nonzero $($from:ty),+ => $args:tt) => {
        $(cast!(@to_nonzero $from => $args);)*
    };

    (@to_nonzero $from:ty => ($($to:ty),+)) => {
        $(
            impl CastImpl<$to> for $from {
                type Error = LossyCastError<Self, $to>;

                #[inline]
                fn cast_impl(self) -> Result<$to, Self::Error> {
                    // Safe to use `new_unchecked` because the value cannot be zero
                    // it cannot be zero itself.
                    match self.get().cast() {
                        Ok(value) => Ok(unsafe {<$to>::new_unchecked(value)}),
                        Err(error) => Err(LossyCastError {
                            from: self,
                            to: unsafe {<$to>::new_unchecked(error.to)}
                        })
                    }
                }
            }

            impl Saturated<$to> for LossyCastError<$from, $to> {
                #[inline]
                fn saturated(self) -> $to {
                    unsafe {
                        // Safe to use `new_unchecked` because the value cannot be zero
                        <$to>::new_unchecked(
                            LossyCastError {
                                from: self.from.get(),
                                to: self.to.get()
                            }.saturated()
                        )
                    }
                }
            }

            impl Closest<$to> for LossyCastError<$from, $to> {
                #[inline]
                fn closest(self) -> $to {
                    // For int-to-int the closest is the saturated
                    self.saturated()
                }
            }
        )*
    };

    (to_primitive $($from:ty),+ => $args:tt) => {
        $(cast!(@to_primitive $from => $args);)*
    };

    (@to_primitive $from:ty => ($($to:ty),+)) => {
        $(
            impl CastImpl<$to> for $from {
                type Error = LossyCastError<Self, $to>;

                #[inline]
                fn cast_impl(self) -> Result<$to, Self::Error> {
                    self.get().cast::<$to>().map_err(|error| LossyCastError {
                        from: self,
                        to: error.to
                    })
                }
            }

            impl Saturated<$to> for LossyCastError<$from, $to> {
                #[inline]
                fn saturated(self) -> $to {
                    LossyCastError {
                        from: self.from.get(),
                        to: self.to
                    }.saturated()
                }
            }

            impl Closest<$to> for LossyCastError<$from, $to> {
                #[inline]
                fn closest(self) -> $to {
                    // For int-to-int the closest is the saturated
                    self.saturated()
                }
            }
        )*
    };

    (from_primitive $($from:ty),+ => $args:tt) => {
        $(cast!(@from_primitive $from => $args);)*
    };

    (@from_primitive $from:ty => ($($to:ty),+)) => {
        $(
            impl CastImpl<$to> for $from {
                type Error = FailedCastError<Self, $to>;

                #[inline]
                fn cast_impl(self) -> Result<$to, Self::Error> {
                    // Cast to the root primitive of the nonzero before creating the nonzero
                    let casted_primitive = self.cast().map_err(|error| FailedCastError::new(self))?;
                    <$to>::new(casted_primitive).ok_or_else(|| FailedCastError::new(self))
                }
            }

            impl Closest<$to> for FailedCastError<$from, $to> {
                #[inline]
                fn closest(self) -> $to {
                    // Create the NonZero from the closest primitive, using a value of 1 if 0
                    <$to>::new(self.from.cast().closest())
                        .unwrap_or_else(|| unsafe {<$to>::new_unchecked(1)})
                }
            }
        )*
    };

    (lossless $from:ty => $($to:ty),+) => {
        $(impl LosslessCast for Result<$to, LossyCastError<$from, $to>> {})*
    };

    (lossless $first:ty, $($from:ty),+ => $to:ty) => {
        cast!(lossless $first => $to);
        $(cast!(lossless $from => $to));*;
    };
}

// -- Macro-Generated Bulk Implementations: Portable -- //
cast!(
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize;
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

cast!(
    lossless NonZeroU8 =>
        NonZeroU8,  NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128,
        NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128,
        u8, u16, u32, u64, u128,
        i16, i32, i64, i128
);

cast!(
    lossless NonZeroU16 =>
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroI32, NonZeroI64, NonZeroI128,
        u16, u32, u64, u128, i32, i64, i128
);

cast!(
    lossless NonZeroU32 =>
        NonZeroU32, NonZeroU64, NonZeroU128, NonZeroI64, NonZeroI128,
        u32, u64, u128, i64, i128
);

cast!(lossless NonZeroU64 => NonZeroU64, NonZeroU128, NonZeroI128, u64, u128, i128);
cast!(lossless NonZeroU128 => NonZeroU128, u128);
cast!(lossless NonZeroUsize => NonZeroUsize, usize);

cast!(
    lossless NonZeroI8 =>
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128,
        i8, i16, i32, i64, i128
);

cast!(lossless NonZeroI16 => NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, i16, i32, i64, i128);
cast!(lossless NonZeroI32 => NonZeroI32, NonZeroI64, NonZeroI128, i32, i64, i128);
cast!(lossless NonZeroI64 => NonZeroI64, NonZeroI128, i64, i128);
cast!(lossless NonZeroI128 => NonZeroI128, i128);
cast!(lossless NonZeroIsize => NonZeroIsize, isize);

// -- Macro-Generated Bulk Implementations: Non-Portable -- //
#[cfg(target_pointer_width = "16")]
mod platform_dependent {
    use super::*;

    cast!(
        lossless NonZeroUsize =>
            NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroI32, NonZeroI64, NonZeroI128,
            u16, u32, u64, u128, i32, i64, i128
    );

    cast!(
        lossless NonZeroIsize =>
            NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128,
            i16, i32, i64, i128
    );

    cast!(lossless NonZeroU8, NonZeroU16 => NonZeroUsize);
    cast!(lossless NonZeroU8, NonZeroI8, NonZeroI16 => NonZeroIsize);
}

#[cfg(target_pointer_width = "32")]
mod platform_dependent {
    use super::*;

    cast!(
        lossless NonZeroUsize =>
            NonZeroU32, NonZeroU64, NonZeroU128, NonZeroI64, NonZeroI128,
            u32, u64, u128, i64, i128
    );

    cast!(lossless NonZeroIsize => NonZeroI32, NonZeroI64, NonZeroI128, i32, i64, i128);

    cast!(lossless NonZeroU8, NonZeroU16, NonZeroU32 => NonZeroUsize);
    cast!(lossless NonZeroU8, NonZeroU16, NonZeroI8, NonZeroI16, NonZeroI32 => NonZeroIsize);
}

#[cfg(target_pointer_width = "64")]
mod platform_dependent {
    use super::*;

    cast!(lossless NonZeroUsize => NonZeroU64, NonZeroU128, NonZeroI128, u64, u128, i128);
    cast!(lossless NonZeroIsize => NonZeroI64, NonZeroI128, i64, i128);

    cast!(lossless NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64 => NonZeroUsize);

    cast!(
        lossless NonZeroU8, NonZeroU16, NonZeroU32, NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64
        => NonZeroIsize
    );
}