//! This module provides built-in implementation of the casting traits for core non-zero types

#![allow(clippy::wildcard_imports)]

use crate::cast::{Cast, Closest, LossyCastError, Saturated};
use crate::base::CastImpl;
use super::LosslessCast;

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

    (to_nonzero $from:ty => $($to:ty),+) => {
        $(
            impl CastImpl<$to> for $from {
                #[inline]
                fn cast_impl(self) -> Result<$to, LossyCastError<Self, $to>> {
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

    (lossless $from:ty => $($to:ty),+) => {
        $(impl LosslessCast for Result<$to, LossyCastError<$from, $to>> {})*
    };

    (lossless $first:ty, $($from:ty),+ => $to:ty) => {
        cast!(lossless $first => $to);
        $(cast!(lossless $from => $to));*;
    };
}

cast!(
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize
);

cast!(
    to_nonzero NonZeroU8 =>
        NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize
);

cast!(
    to_nonzero NonZeroU16 =>
        NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize
);

cast!(
    to_nonzero NonZeroU32 =>
        NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize
);

cast!(
    to_nonzero NonZeroU64 =>
        NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize
);

cast!(
    to_nonzero NonZeroU128 =>
        NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize
);

cast!(
    to_nonzero NonZeroUsize =>
        NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize
);

cast!(
    to_nonzero NonZeroI8 =>
        NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize
);

cast!(
    to_nonzero NonZeroI16 =>
        NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize
);

cast!(
    to_nonzero NonZeroI32 =>
        NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize
);

cast!(
    to_nonzero NonZeroI64 =>
        NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize
);

cast!(
    to_nonzero NonZeroI128 =>
        NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize
);

cast!(
    to_nonzero NonZeroIsize =>
        NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize
);


cast!(
    lossless NonZeroU8 =>
        NonZeroU8,  NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128,
        NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128
);

cast!(
    lossless NonZeroU16 =>
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroI32, NonZeroI64, NonZeroI128
);

cast!(lossless NonZeroU32 => NonZeroU32, NonZeroU64, NonZeroU128, NonZeroI64, NonZeroI128);
cast!(lossless NonZeroU64 => NonZeroU64, NonZeroU128, NonZeroI128);
cast!(lossless NonZeroU128 => NonZeroU128);
cast!(lossless NonZeroUsize => NonZeroUsize);

cast!(lossless NonZeroI8 => NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128);
cast!(lossless NonZeroI16 => NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128);
cast!(lossless NonZeroI32 => NonZeroI32, NonZeroI64, NonZeroI128);
cast!(lossless NonZeroI64 => NonZeroI64, NonZeroI128);
cast!(lossless NonZeroI128 => NonZeroI128);
cast!(lossless NonZeroIsize => NonZeroIsize);

// -- Macro-Generated Bulk Implementations: Non-Portable -- //
#[cfg(target_pointer_width = "16")]
mod platform_dependent {
    use super::*;

    cast!(
        lossless NonZeroUsize =>
            NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroI32, NonZeroI64, NonZeroI128
    );

    cast!(lossless NonZeroIsize => NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128);

    cast!(lossless NonZeroU8, NonZeroU16 => NonZeroUsize);
    cast!(lossless NonZeroU8, NonZeroI8, NonZeroI16 => NonZeroIsize);
}

#[cfg(target_pointer_width = "32")]
mod platform_dependent {
    use super::*;

    cast!(lossless NonZeroUsize => NonZeroU32, NonZeroU64, NonZeroU128, NonZeroI64, NonZeroI128);
    cast!(lossless NonZeroIsize => NonZeroI32, NonZeroI64, NonZeroI128);

    cast!(lossless NonZeroU8, NonZeroU16, NonZeroU32 => NonZeroUsize);
    cast!(lossless NonZeroU8, NonZeroU16, NonZeroI8, NonZeroI16, NonZeroI32 => NonZeroIsize);
}

#[cfg(target_pointer_width = "64")]
mod platform_dependent {
    use super::*;

    cast!(lossless NonZeroUsize => NonZeroU64, NonZeroU128, NonZeroI128);
    cast!(lossless NonZeroIsize => NonZeroI64, NonZeroI128);

    cast!(lossless NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64 => NonZeroUsize);

    cast!(
        lossless NonZeroU8, NonZeroU16, NonZeroU32, NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64
        => NonZeroIsize
    );
}