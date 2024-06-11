//! This module provides built-in implementation of the casting traits for core non-zero types

#![allow(clippy::wildcard_imports)]

use crate::casts::{Cast, Closest, Lossless};
use crate::errors::{FailedCastError, LosslessCastError, LossyCastError};
use crate::base::CastImpl;

use core::num::{
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize
};

macro_rules! cast {
    // Implements Cast for each `$num`
    ($($num:ty),+) => {
        $(
            impl Cast for $num {}
        )*
    };

    // Implements Cast and Closest for each pair via LossyCastError:
    // * `$from` -> `$nonzero` where `$from` is NonZero* and `$nonzero` is also NonZero*
    // * `$from` -> `$primitive` where `$from` is NonZero* and `$primitive` is a primitive
    // Closest is implemented in terms of the underlying primitive implementation of Closest.
    (from_nonzero $from:ty => $($nonzero:ty),+; $($primitive:ty),+) => {
        $(
            impl CastImpl<$nonzero> for $from {
                type Error = LossyCastError<Self, $nonzero>;

                #[inline]
                fn cast_impl(self) -> Result<$nonzero, Self::Error> {
                    // Safe to use `new_unchecked` because the value cannot be zero
                    match self.get().cast() {
                        Ok(value) => Ok(unsafe {<$nonzero>::new_unchecked(value)}),
                        Err(error) => Err(LossyCastError {
                            from: self,
                            to: unsafe {<$nonzero>::new_unchecked(error.to)}
                        })
                    }
                }
            }

            impl Closest<$nonzero> for LossyCastError<$from, $nonzero> {
                #[inline]
                fn closest(self) -> $nonzero {
                    unsafe {
                        // Safe to use `new_unchecked` because the value cannot be zero
                        <$nonzero>::new_unchecked(
                            LossyCastError {
                                from: self.from.get(),
                                to: self.to.get()
                            }.closest()
                        )
                    }
                }
            }
        )*

        $(
            impl CastImpl<$primitive> for $from {
                type Error = LossyCastError<Self, $primitive>;

                #[inline]
                fn cast_impl(self) -> Result<$primitive, Self::Error> {
                    // Cast via the associated primitive
                    self.get().cast::<$primitive>().map_err(|error| Self::Error {
                        from: self,
                        to: error.to
                    })
                }
            }

            impl Closest<$primitive> for LossyCastError<$from, $primitive> {
                #[inline]
                fn closest(self) -> $primitive {
                    // Cast via the associated primitive
                    LossyCastError {
                        from: self.from.get(),
                        to: self.to
                    }.closest()
                }
            }
        )*
    };

    (from_nonzero $first:ty, $($from:ty),+ => $nonzero:tt; $primitive:tt) => {
        cast!(from_nonzero $first => $nonzero; $primitive);
        $(cast!(from_nonzero $from => $nonzero; $primitive);)*
    };

    // $from must be NonZero* because there is no Lossless available from primitive -> NonZero*
    (lossless $from:ty => $($nonzero:ty),*; $($primitive:ty),*) => {
        $(
            impl CastImpl<$nonzero> for $from {
                type Error = LosslessCastError<Self, $nonzero>;

                #[inline]
                fn cast_impl(self) -> Result<$nonzero, Self::Error> {
                    // Safe to use `new_unchecked` because the value cannot be zero
                    Ok(unsafe {<$nonzero>::new_unchecked(self.get().cast().lossless())})
                }
            }
        )*

        $(
            impl CastImpl<$primitive> for $from {
                type Error = LosslessCastError<Self, $primitive>;

                #[inline]
                fn cast_impl(self) -> Result<$primitive, Self::Error> {
                    Ok(self.get().cast().lossless())
                }
            }
        )*
    };

    (lossless $first:ty, $($from:ty),+ => $nonzero:tt; $primitive:tt) => {
        cast!(lossless $first => $nonzero; $primitive);
        $(cast!(lossless $from => $nonzero; $primitive);)*
    };

    // Implements Cast and Closest for each pair via FailedCastError:
    // `$primitive` -> `$nonzero` where `$primitive` is a primitive and `$nonzero` is a NonZero*
    // Closest is implemented in terms of the underlying primitive implementation of Closest, but
    // mapping 0 -> 1.
    (from_primitive $primitive:ty => ($($nonzero:ty),+)) => {
        $(
            impl CastImpl<$nonzero> for $primitive {
                type Error = FailedCastError<Self, $nonzero>;

                #[inline]
                fn cast_impl(self) -> Result<$nonzero, Self::Error> {
                    // Cast to the root primitive of the nonzero before creating the nonzero
                    let primitive = self.cast().map_err(|_error| FailedCastError::new(self))?;
                    <$nonzero>::new(primitive).ok_or_else(|| FailedCastError::new(self))
                }
            }

            impl Closest<$nonzero> for FailedCastError<$primitive, $nonzero> {
                #[inline]
                fn closest(self) -> $nonzero {
                    // Create the NonZero from the closest primitive, using a value of 1 if 0
                    <$nonzero>::new(self.from.cast().closest())
                        .unwrap_or_else(|| unsafe {<$nonzero>::new_unchecked(1)})
                }
            }
        )*
    };

    (from_primitive $first:ty, $($primitive:ty),+ => $nonzero:tt) => {
        cast!(from_primitive $first => $nonzero);
        $(cast!(from_primitive $primitive => $nonzero);)*
    };

    // Implements Cast and Closest for each pair via FailedCastError:
    // `$float` -> `$signed_nonzero` where `$float` is a float point primitive and `$signed_nonzero`
    // is a NonZeroI* (that is, a signed NonZero* type)
    // Closest is implemented in terms of the underlying primitive implementation of Closest, but
    // mapping +0 -> 1 and -0 to -1.
    (from_float_to_signed $float:ty => ($($signed_nonzero:ty),+)) => {
        $(
            impl CastImpl<$signed_nonzero> for $float {
                type Error = FailedCastError<Self, $signed_nonzero>;

                #[inline]
                fn cast_impl(self) -> Result<$signed_nonzero, Self::Error> {
                    // Cast to the root primitive of the nonzero before creating the nonzero
                    let primitive = self.cast().map_err(|_error| FailedCastError::new(self))?;
                    <$signed_nonzero>::new(primitive).ok_or_else(|| FailedCastError::new(self))
                }
            }

            impl Closest<$signed_nonzero> for FailedCastError<$float, $signed_nonzero> {
                #[inline]
                fn closest(self) -> $signed_nonzero {
                    // Create the NonZero from the closest primitive
                    <$signed_nonzero>::new(self.from.cast().closest())
                        .unwrap_or_else(|| unsafe {<$signed_nonzero>::new_unchecked(
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

    (from_float_to_signed $first:ty, $($float:ty),+ => $signed_nonzero:tt) => {
        cast!(from_float_to_signed $first => $signed_nonzero);
        $(cast!(from_float_to_signed $float => $signed_nonzero);)*
    }
}

// -- Macro-Generated Bulk Implementations: Portable -- //
cast!(
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize
);

cast!(
    lossless NonZeroU8 =>
    NonZeroU8,  NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128,
    NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128;
    u8, u16, u32, u64, u128,
    i16, i32, i64, i128,
    f32, f64
);

cast!(from_nonzero NonZeroU8 => NonZeroI8; i8);

cast!(
    lossless NonZeroU16 =>
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroI32, NonZeroI64, NonZeroI128;
    u16, u32, u64, u128,
    i32, i64, i128,
    f32, f64
);

cast!(from_nonzero NonZeroU16 => NonZeroU8, NonZeroI8, NonZeroI16; u8, i8, i16);

cast!(
    lossless NonZeroU32 =>
    NonZeroU32, NonZeroU64, NonZeroU128, NonZeroI64, NonZeroI128;
    u32, u64, u128, i64, i128, f64
);

cast!(
    from_nonzero NonZeroU32 =>
    NonZeroU8, NonZeroU16, NonZeroI8, NonZeroI16, NonZeroI32;
    u8, u16, i8, i16, i32, f32
);

cast!(lossless NonZeroU64 => NonZeroU64, NonZeroU128, NonZeroI128; u64, u128, i128);

cast!(
    from_nonzero NonZeroU64 =>
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64;
    u8, u16, u32, i8, i16, i32, i64, f32, f64
);

cast!(lossless NonZeroU128 => NonZeroU128; u128);

cast!(
    from_nonzero NonZeroU128 =>
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64,
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128;
    u8, u16, u32, u64, i8, i16, i32, i64, i128, f32, f64
);

cast!(
    lossless NonZeroI8 =>
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128;
    i8, i16, i32, i64, i128, f32, f64
);

cast!(
    from_nonzero NonZeroI8 =>
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128;
    u8, u16, u32, u64, u128
);

cast!(
    lossless NonZeroI16 =>
    NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128;
    i16, i32, i64, i128, f32, f64
);

cast!(
    from_nonzero NonZeroI16 =>
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128,
    NonZeroI8;
    u8, u16, u32, u64, u128, i8
);

cast!(lossless NonZeroI32 => NonZeroI32, NonZeroI64, NonZeroI128; i32, i64, i128, f64);

cast!(
    from_nonzero NonZeroI32 =>
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128,
    NonZeroI8, NonZeroI16;
    u8, u16, u32, u64, u128, i8, i16, f32
);

cast!(lossless NonZeroI64 => NonZeroI64, NonZeroI128; i64, i128);

cast!(
    from_nonzero NonZeroI64 =>
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128,
    NonZeroI8, NonZeroI16, NonZeroI32;
    u8, u16, u32, u64, u128, i8, i16, i32, f32, f64
);

cast!(lossless NonZeroI128 => NonZeroI128; i128);

cast!(
    from_nonzero NonZeroI128 =>
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128,
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64;
    u8, u16, u32, u64, u128, i8, i16, i32, i64, f32, f64
);

cast!(
    from_primitive 
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize => (
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize
));

cast!(from_primitive f32, f64 => (
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize
));

cast!(from_float_to_signed f32, f64 => (
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize
));

cast!(lossless NonZeroUsize => NonZeroUsize; usize);
cast!(from_nonzero NonZeroUsize => NonZeroIsize; isize);

cast!(lossless NonZeroIsize => NonZeroIsize; isize);
cast!(from_nonzero NonZeroIsize => NonZeroUsize; usize);

// -- Macro-Generated Bulk Implementations: Non-Portable -- //
#[cfg(target_pointer_width = "16")]
mod platform_dependent {
    use super::*;

    cast!(
        lossless NonZeroUsize =>
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroI32, NonZeroI64, NonZeroI128;
        u16, u32, u64, u128,
        i32, i64, i128,
        f32, f64
    );

    cast!(from_nonzero NonZeroUsize => NonZeroU8, NonZeroI8, NonZeroI16; u8, i8, i16);

    cast!(
        lossless NonZeroIsize =>
        NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128;
        i16, i32, i64, i128, f32, f64
    );

    cast!(
        from_nonzero NonZeroIsize =>
        NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128,
        NonZeroI8;
        u8, u16, u32, u64, u128, i8
    );

    cast!(lossless NonZeroU8, NonZeroU16 => NonZeroUsize; usize);

    cast!(
        from_nonzero
        NonZeroU32, NonZeroU64, NonZeroU128,
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128
        => NonZeroUsize; usize
    );

    cast!(lossless NonZeroU8, NonZeroI8, NonZeroI16 => NonZeroIsize; isize);

    cast!(
        from_nonzero
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128,
        NonZeroI32, NonZeroI64, NonZeroI128
        => NonZeroIsize; isize
    );
}

#[cfg(target_pointer_width = "32")]
mod platform_dependent {
    use super::*;

    cast!(
        lossless NonZeroUsize =>
        NonZeroU32, NonZeroU64, NonZeroU128, NonZeroI64, NonZeroI128;
        u32, u64, u128, i64, i128, f64
    );

    cast!(
        from_nonzero NonZeroUsize =>
        NonZeroU8, NonZeroU16, NonZeroI8, NonZeroI16, NonZeroI32;
        u8, u16, i8, i16, i32, f32
    );

    cast!(lossless NonZeroIsize => NonZeroI32, NonZeroI64, NonZeroI128; i32, i64, i128, f64);

    cast!(
        from_nonzero NonZeroIsize =>
        NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128,
        NonZeroI8, NonZeroI16;
        u8, u16, u32, u64, u128, i8, i16, f32
    );

    cast!(lossless NonZeroU8, NonZeroU16, NonZeroU32 => NonZeroUsize; usize);

    cast!(
        from_nonzero
        NonZeroU64, NonZeroU128,
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128
        => NonZeroUsize; usize
    );

    cast!(
        lossless
        NonZeroU8, NonZeroU16,
        NonZeroI8, NonZeroI16, NonZeroI32 =>
        NonZeroIsize; isize
    );

    cast!(
        from_nonzero
        NonZeroU32, NonZeroU64, NonZeroU128,
        NonZeroI64, NonZeroI128
        => NonZeroIsize; isize
    );
}

#[cfg(target_pointer_width = "64")]
mod platform_dependent {
    use super::*;

    cast!(lossless NonZeroUsize => NonZeroU64, NonZeroU128, NonZeroI128; u64, u128, i128);

    cast!(
        from_nonzero NonZeroUsize =>
        NonZeroU8, NonZeroU16, NonZeroU32, NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64;
        u8, u16, u32, i8, i16, i32, i64, f32, f64
    );

    cast!(lossless NonZeroIsize => NonZeroI64, NonZeroI128; i64, i128);

    cast!(
        from_nonzero NonZeroIsize =>
        NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128,
        NonZeroI8, NonZeroI16, NonZeroI32;
        u8, u16, u32, u64, u128, i8, i16, i32, f32, f64
    );

    cast!(lossless NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64 => NonZeroUsize; usize);

    cast!(
        from_nonzero
        NonZeroU128,
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128
        => NonZeroUsize; usize
    );

    cast!(
        lossless
        NonZeroU8, NonZeroU16, NonZeroU32,
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64
        => NonZeroIsize; isize
    );

    cast!(from_nonzero NonZeroU64, NonZeroU128, NonZeroI128 => NonZeroIsize; isize);
}

#[cfg(target_pointer_width = "128")]
mod platform_dependent {
    use super::*;

    cast!(lossless NonZeroUsize => NonZeroU128; u128);

    cast!(
        from_nonzero NonZeroUsize =>
        NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, 
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128;
        u8, u16, u32, u64, i8, i16, i32, i64, i128, f32, f64
    );

    cast!(lossless NonZeroIsize => NonZeroI128; i128);

    cast!(
        from_nonzero NonZeroIsize =>
        NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128,
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64;
        u8, u16, u32, u64, u128, i8, i16, i32, i64, f32, f64
    );

    cast!(
        lossless 
        NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128 => 
        NonZeroUsize; usize
    );

    cast!(
        from_nonzero
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128
        => NonZeroUsize; usize
    );

    cast!(
        lossless
        NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64,
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128
        => NonZeroIsize; isize
    );

    cast!(from_nonzero NonZeroU128 => NonZeroIsize; isize);
}