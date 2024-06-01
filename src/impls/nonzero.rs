//! This module provides built-in implementation of the casting traits for core non-zero types

#![allow(clippy::wildcard_imports)]

use crate::casts::{Cast, Closest};
use crate::errors::{FailedCastError, LossyCastError};
use crate::base::CastTo;
use super::LosslessCast;

use core::num::{
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize
};

macro_rules! cast {
    ($($nonzero_unsigned:ty),+; $($nonzero_signed:ty),+; $($integer:ty),+; $($floating:ty),+) => {
        $(
            impl Cast for $nonzero_unsigned {}
            impl Cast for $nonzero_signed {}
        )*

        // Generate the nonzero to nonzero implementations in n-squared fashion
        cast!(
            nonzero $($nonzero_unsigned),*, $($nonzero_signed),* =>
            ($($nonzero_unsigned),*, $($nonzero_signed),*)
        );

        // Generate the nonzero to primitive implementations in n*m fashion
        cast!(
            to_primitive $($nonzero_unsigned),*, $($nonzero_signed),* =>
            ($($integer),*, $($floating),*)
        );

        cast!(to_integer $($nonzero_unsigned),*, $($nonzero_signed),* => ($($integer),*));
        cast!(to_floating $($nonzero_unsigned),*, $($nonzero_signed),* => ($($floating),*));

        // Generate the primitive to nonzero implementations in n*m fashion
        cast!(
            from_primitive $($integer),*, $($floating),* =>
            ($($nonzero_unsigned),*, $($nonzero_signed),*)
        );
        
        // The closest value for 0 is 1 when coming from an integer to any nonzero
        cast!(
            from_primitive_positive_estimate $($integer),* =>
            ($($nonzero_unsigned),*, $($nonzero_signed),*)
        );

        // The closest value for 0 is 1 when coming from a float to an unsigned nonzero
        cast!(from_primitive_positive_estimate $($floating),* => ($($nonzero_unsigned),*));

        // The closest value for 0 could be 1 or -1 when coming from a float to a signed nonzero
        cast!(from_floating_to_signed $($floating),* => ($($nonzero_signed),*));
    };

    // -- Adapters for calling sub-macro permutations -- //
    (nonzero $($from:ty),+ => $args:tt) => {
        $(cast!(@nonzero $from => $args);)*
    };

    (to_primitive $($from:ty),+ => $args:tt) => {
        $(cast!(@to_primitive $from => $args);)*
    };

    (to_integer $($from:ty),+ => $args:tt) => {
        $(cast!(@to_integer $from => $args);)*
    };

    (to_floating $($from:ty),+ => $args:tt) => {
        $(cast!(@to_floating $from => $args);)*
    };

    (from_primitive $($from:ty),+ => $args:tt) => {
        $(cast!(@from_primitive $from => $args);)*
    };

    (from_primitive_positive_estimate $($from:ty),+ => $args:tt) => {
        $(cast!(@from_primitive_positive_estimate $from => $args);)*
    };

    (from_floating_to_signed $($from:ty),+ => $args:tt) => {
        $(cast!(@from_floating_to_signed $from => $args);)*
    };

    // -- Sub-macros -- //
    (@nonzero $from:ty => ($($to:ty),+)) => {
        $(
            impl CastTo<$to> for $from {
                type Error = LossyCastError<Self, $to>;

                #[inline]
                fn cast_to(self) -> Result<$to, Self::Error> {
                    // Safe to use `new_unchecked` because the value cannot be zero
                    match self.get().cast() {
                        Ok(value) => Ok(unsafe {<$to>::new_unchecked(value)}),
                        Err(error) => Err(LossyCastError {
                            from: self,
                            to: unsafe {<$to>::new_unchecked(error.to)}
                        })
                    }
                }
            }

            impl Closest<$to> for LossyCastError<$from, $to> {
                #[inline]
                fn closest(self) -> $to {
                    unsafe {
                        // Safe to use `new_unchecked` because the value cannot be zero
                        <$to>::new_unchecked(
                            LossyCastError {
                                from: self.from.get(),
                                to: self.to.get()
                            }.closest()
                        )
                    }
                }
            }
        )*
    };

    (@to_primitive $from:ty => ($($to:ty),+)) => {
        $(
            impl CastTo<$to> for $from {
                type Error = LossyCastError<Self, $to>;

                #[inline]
                fn cast_to(self) -> Result<$to, Self::Error> {
                    self.get().cast::<$to>().map_err(|error| LossyCastError {
                        from: self,
                        to: error.to
                    })
                }
            }
        )*
    };

    (@to_integer $from:ty => ($($to:ty),+)) => {
        $(
            impl Closest<$to> for LossyCastError<$from, $to> {
                #[inline]
                fn closest(self) -> $to {
                    // Delegate to the primitive's implementation of closest
                    LossyCastError {
                        from: self.from.get(),
                        to: self.to
                    }.closest()
                }
            }
        )*
    };

    (@to_floating $from:ty => ($($to:ty),+)) => {
        $(
            impl Closest<$to> for LossyCastError<$from, $to> {
                #[inline]
                fn closest(self) -> $to {
                    // For int-to-float the raw cast is the closest
                    self.to
                }
            }
        )*
    };

    (@from_primitive $from:ty => ($($to:ty),+)) => {
        $(
            impl CastTo<$to> for $from {
                type Error = FailedCastError<Self, $to>;

                #[inline]
                fn cast_to(self) -> Result<$to, Self::Error> {
                    // Cast to the root primitive of the nonzero before creating the nonzero
                    let primitive = self.cast().map_err(|_error| FailedCastError::new(self))?;
                    <$to>::new(primitive).ok_or_else(|| FailedCastError::new(self))
                }
            }
        )*
    };

    (@from_primitive_positive_estimate $from:ty => ($($to:ty),+)) => {
        $(
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

    (@from_floating_to_signed $from:ty => ($($to:ty),+)) => {
        $(
            impl Closest<$to> for FailedCastError<$from, $to> {
                #[inline]
                fn closest(self) -> $to {
                    // Create the NonZero from the closest primitive
                    <$to>::new(self.from.cast().closest())
                        .unwrap_or_else(|| unsafe {<$to>::new_unchecked(
                            // Use a value of 1 if positive or -1 otherwise
                            match self.from.is_sign_positive() {
                                true => 1,
                                false => -1
                            }
                        )})
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
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize;
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize;
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize;
    f32, f64
);

cast!(
    lossless NonZeroU8 =>
        NonZeroU8,  NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128,
        NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128,
        u8, u16, u32, u64, u128, usize,
        i16, i32, i64, i128, isize
);

cast!(
    lossless NonZeroU16 =>
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroI32, NonZeroI64, NonZeroI128,
        u16, u32, u64, u128, usize, i32, i64, i128
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
        i8, i16, i32, i64, i128, isize
);

cast!(
    lossless NonZeroI16 =>
    NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128,
    i16, i32, i64, i128, isize
);

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
    cast!(lossless NonZeroU32 => usize);

    cast!(lossless NonZeroU8, NonZeroU16, NonZeroI8, NonZeroI16, NonZeroI32 => NonZeroIsize);
    cast!(lossless NonZeroU16, NonZeroI32 => isize);
}

#[cfg(target_pointer_width = "64")]
mod platform_dependent {
    use super::*;

    cast!(lossless NonZeroUsize => NonZeroU64, NonZeroU128, NonZeroI128, u64, u128, i128);
    cast!(lossless NonZeroIsize => NonZeroI64, NonZeroI128, i64, i128);

    cast!(lossless NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64 => NonZeroUsize);
    cast!(lossless NonZeroU32, NonZeroU64 => usize);

    cast!(
        lossless NonZeroU8, NonZeroU16, NonZeroU32, NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64
        => NonZeroIsize
    );

    cast!(lossless NonZeroU16, NonZeroU32, NonZeroI32, NonZeroI64 => isize);
}