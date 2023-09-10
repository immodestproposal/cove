//! This module provides built-in implementation of the casting traits for primitive types

#![allow(clippy::wildcard_imports)]

use crate::casts::{Cast, Closest, Saturated};
use crate::errors::LossyCastError;
use crate::base::CastImpl;
use super::LosslessCast;

macro_rules! cast {
    ($($num:ty),+) => {
        $(
            impl Cast for $num {}
        )*
    };

    (integer $from:ty => $($to:ty),+) => {
        $(
            impl CastImpl<$to> for $from {
                type Error = LossyCastError<Self, $to>;

                #[inline]
                fn cast_impl(self) -> Result<$to, Self::Error> {
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

            impl Closest<$to> for LossyCastError<$from, $to> {
                #[inline]
                fn closest(self) -> $to {
                    // For int-to-int the closest is the saturated
                    self.saturated()
                }
            }
        )*
    };

    (floating $from:ty => $to:ty) => {
        impl CastImpl<$to> for $from {
            type Error = LossyCastError<Self, $to>;

            #[inline]
            fn cast_impl(self) -> Result<$to, Self::Error> {
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
    };

    (int_to_float $from:ty => $($to:ty),+) => {
        $(
            cast!(floating $from => $to);

            impl Closest<$to> for LossyCastError<$from, $to> {
                #[inline]
                fn closest(self) -> $to {
                    // For int-to-float the raw cast is the closest
                    self.to
                }
            }
        )*
    };

    (float_to_int $from:ty => $($to:ty),+) => {
        $(
            cast!(floating $from => $to);

            impl Closest<$to> for LossyCastError<$from, $to> {
                #[inline]
                fn closest(self) -> $to {
                    // For float-to-int we must first round the number, then use the raw cast. If we
                    // don't round first the default will round towards zero.
                    #[cfg(feature = "std")] {
                        // We have access to std so use the built-in round() function (which uses a
                        // compiler intrinsic in its turn)
                        self.from.rounded() as $to
                    }

                    #[cfg(not(feature = "std"))] {
                        // We lack access to std so we must implement our own (almost certainly
                        // slower) round() function
                        todo!()
                        //self.from as $to
                        //(self.from - 0.5) as $to
                    }
                }
            }
        )*
    };

    (int_to_float $first:ty, $($from:ty),+ => $to:ty) => {
        cast!(int_to_float $first => $to);
        $(cast!(int_to_float $from => $to));*;
    };

    (float_to_int $first:ty, $($from:ty),+ => $to:ty) => {
        cast!(float_to_int $first => $to);
        $(cast!(float_to_int $from => $to));*;
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

cast!(int_to_float u32, u64, u128, usize, i32, i64, i128, isize => f32);
cast!(int_to_float u64, u128, usize, i64, i128, isize => f64);
cast!(float_to_int f32 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
cast!(float_to_int f64 => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

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