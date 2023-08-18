//! This module provides built-in implementation of the casting traits for primitive types

#![allow(clippy::wildcard_imports)]

use crate::cast::{AssumeLossless, Cast, Closest, Lossless, Lossy, LossyCastError, Saturated};
use crate::base::CastImpl;

macro_rules! cast {
    ($($num:ty),+) => {
        $(
            impl Cast for $num {}
        )*

        // All casts can be lossy, so generate the LossyCastImpls in n-squared fashion
        cast!(lossy $($num),* => ($($num),*));
    };

    (@lossy $from:ty => ($($to:ty),+)) => {
        $(
            impl Lossy<$to> for Result<$to, LossyCastError<$from, $to>> {
                #[inline]
                fn lossy(self) -> $to {
                    self.unwrap_or_else(|error| error.to)
                }
            }

            impl AssumeLossless<$to> for Result<$to, LossyCastError<$from, $to>> {
                #[inline]
                fn assume_lossless(self) -> $to {
                    self.unwrap_or_else(|error| {
                        // Should not arrive here; panic in a debug build
                        debug_assert!(
                            false,
                            "Lossy cast was assumed to be lossless [{} ({}) -> {} ({})]",
                            error.from, stringify!($from),
                            error.to, stringify!($to)
                        );

                        error
                    }.to)
                }
            }
        )*
    };

    (lossy $($from:ty),+ => $args:tt) => {
        $(cast!(@lossy $from => $args);)*
    };

    (integer $from:ty => $($to:ty),+) => {
        $(
            impl CastImpl<$to> for $from {
                #[inline]
                fn cast_impl(self) -> Result<$to, LossyCastError<Self, $to>> {
                    self.try_into().map_err(|_| LossyCastError {
                        from: self,
                        to: self as $to
                    })
                }
            }

            impl Saturated<$to> for LossyCastError<$from, $to> {
                #[inline]
                fn saturated(self) -> $to {
                    // Cast failed; if this is less than 0 use the target's MIN, otherwise
                    // use its MAX. This logic cannot be used in general for saturation but
                    // holds for all types actually fed to this macro. Note that the branch
                    // will be optimized away for unsigned source types, at least in release.
                    #[allow(unused_comparisons)]
                    match self.from < 0 {
                        true => <$to>::MIN,
                        false => <$to>::MAX
                    }
                }
            }

            impl Saturated<$to> for Result<$to, LossyCastError<$from, $to>> {
                #[inline]
                fn saturated(self) -> $to {
                    self.unwrap_or_else(|error| error.saturated())
                }
            }

            impl Closest<$to> for LossyCastError<$from, $to> {
                #[inline]
                fn closest(self) -> $to {
                    // For int-to-int the closest is the saturated
                    self.saturated()
                }
            }

            impl Closest<$to> for Result<$to, LossyCastError<$from, $to>> {
                #[inline]
                fn closest(self) -> $to {
                    self.unwrap_or_else(|error| error.closest())
                }
            }
        )*
    };

    (floating $from:ty => $($to:ty),+) => {
        $(
            impl CastImpl<$to> for $from {
                #[inline]
                fn cast_impl(self) -> Result<$to, LossyCastError<Self, $to>> {
                    // Because TryFrom/TryInto is not implemented for floating point, we test
                    // for lossy conversions by casting to the target and back, then checking
                    // whether any data was lost.
                    #[allow(clippy::float_cmp)]
                    match self == (self as $to) as $from {
                        true => Ok(self as $to),
                        false => Err(LossyCastError {
                            from: self,
                            to: self as $to
                        })
                    }
                }
            }

            impl Closest<$to> for LossyCastError<$from, $to> {
                #[inline]
                fn closest(self) -> $to {
                    // For float-to-int and int-to-float the raw cast is the closest
                    self.to
                }
            }

            impl Closest<$to> for Result<$to, LossyCastError<$from, $to>> {
                #[inline]
                fn closest(self) -> $to {
                    self.unwrap_or_else(|error| error.closest())
                }
            }
        )*
    };

    (floating $first:ty, $($from:ty),+ => $to:ty) => {
        cast!(floating $first => $to);
        $(cast!(floating $from => $to));*;
    };

    (lossless $from:ty => $($to:ty),+) => {
        $(
            impl Lossless<$to> for Result<$to, LossyCastError<$from, $to>> {
                #[inline]
                fn lossless(self) -> $to {
                    debug_assert!(
                        self.is_ok(),
                        "Implementation error: implemented Lossless for invalid types [{} -> {}]",
                        stringify!($from),
                        stringify!($to)
                    );

                    unsafe {self.unwrap_unchecked()}
                }
            }
        )*
    };

    (lossless $first:ty, $($from:ty),+ => $to:ty) => {
        cast!(lossless $first => $to);
        $(cast!(lossless $from => $to));*;
    };
}

// -- Macro-Generated Bulk Implementations: Portable -- //
cast!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

cast!(integer u8 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
cast!(integer u16 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
cast!(integer u32 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f64);
cast!(integer u64 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
cast!(integer u128 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
cast!(integer usize => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

cast!(integer i8 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
cast!(integer i16 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
cast!(integer i32 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f64);
cast!(integer i64 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
cast!(integer i128 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
cast!(integer isize => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

cast!(floating u32, u64, u128, usize, i32, i64, i128, isize => f32);
cast!(floating u64, u128, usize, i64, i128, isize => f64);
cast!(floating f32 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
cast!(floating f64 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

cast!(lossless u8 => u8, u16, u32, u64, u128, i16, i32, i64, i128, f32, f64);
cast!(lossless u16 => u16, u32, u64, u128, i32, i64, i128, f32, f64);
cast!(lossless u32 => u32, u64, u128, i64, i128, f64);
cast!(lossless u64 => u64, u128, i128);
cast!(lossless u128 => u128);
cast!(lossless usize => usize);

cast!(lossless i8 => i8, i16, i32, i64, i128, f32, f64);
cast!(lossless i16 => i16, i32, i64, i128, f32, f64);
cast!(lossless i32 => i32, i64, i128, f64);
cast!(lossless i64 => i64, i128);
cast!(lossless i128 => i128);
cast!(lossless isize => isize);

cast!(lossless f32 => f32, f64);
cast!(lossless f64 => f64);

// -- Macro-Generated Bulk Implementations: Non-Portable -- //
#[cfg(target_pointer_width = "16")]
mod platform_dependent {
    use super::*;

    cast!(lossless usize => u16, u32, u64, u128, i32, i64, i128, f32, f64);
    cast!(lossless isize => i16, i32, i64, i128, f32, f64);

    cast!(lossless u8, u16 => usize);
    cast!(lossless u8, i8, i16 => isize);
}

#[cfg(target_pointer_width = "32")]
mod platform_dependent {
    use super::*;

    cast!(lossless usize => u32, u64, u128, i64, i128, f64);
    cast!(lossless isize => i32, i64, i128, f64);

    cast!(lossless u8, u16, u32 => usize);
    cast!(lossless u8, u16, i8, i16, i32 => isize);
}

#[cfg(target_pointer_width = "64")]
mod platform_dependent {
    use super::*;

    cast!(lossless usize => u64, u128, i128);
    cast!(lossless isize => i64, i128);

    cast!(lossless u8, u16, u32, u64 => usize);
    cast!(lossless u8, u16, u32, i8, i16, i32, i64 => isize);
}

// -- Manual Implementations -- //
impl Saturated<f64> for f32 {
    #[inline]
    fn saturated(self) -> f64 {
        self.into()
    }
}

impl Closest<f64> for f32 {
    #[inline]
    fn closest(self) -> f64 {
        self.into()
    }
}

impl Closest<f32> for f64 {
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    fn closest(self) -> f32 {
        self as f32
    }
}