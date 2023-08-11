//! This module provides extension traits for casting between numerical types. Many of these traits
//! parallel existing mechanisms such as [`From`] or [`TryFrom`], but offer differing semantics
//! tailored to numerical types. A concrete goal of this module is to reduce or eliminate the need
//! for casting numerical types via the `as` keyword by making intent more explicit, since
//! `as`-based casts can be challenging to maintain.

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

/// Provides a mechanism for fallibly casting between numerical types; this is spiritually similar
/// to [`TryFrom`]/[`TryInto`], but offers a few advantages. Specifically, its narrower focus
/// allows it to provide a richer error type and it is implemented for additional conversions, such
/// as to and from floating point numbers.
///
/// This is intended for fallible casting with error detection. If a cast is known to be infallible,
/// consider using [`LosslessCast`] or [`From`]/[`Into`] instead. If a cast is fallible but there
/// is no interest in error detection, consider using [`LossyCast`] or the `as` keyword instead.
pub trait Cast {
    /// Attempts to cast this numerical type to type `T`. Depending on the usage, it may be
    /// necessary to disambiguate the target type. Returns Ok if the cast is lossless and Err if
    /// some amount of data was lost; note that the casted value is retrievable from the error.
    ///
    /// # Examples
    /// ```
    /// use cove_num::cast::Cast;
    ///
    /// // Call a function `foo` via a cast; no type disambiguation required in this case
    /// fn foo(x: u8) -> u8 {x}
    /// assert_eq!(foo(7u32.cast()?), 7u8);
    ///
    /// // Explicit disambiguation via turbofish required in this case
    /// assert_eq!(7u32.cast::<u8>()?, 7u8);
    ///
    /// // Cast a float to an integer losslessly
    /// assert_eq!(6f64.cast::<i8>()?, 6);
    ///
    /// // Cast a float to an integer lossily, extracting the lossy value from the error
    /// assert_eq!(6.3f32.cast::<i32>().unwrap_err().to, 6);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn cast<T>(self) -> Result<T, CastError<Self, T>> where Self: Sized + private::CastImpl<T> {
        self.cast_impl()
    }
}

/// Provides a mechanism for casting between numerical types without error detection. This is
/// spiritually similar to the `as` keyword but offers a few advantages. Foremost among these is to
/// improve self-documentation of code by expressing that the author intended the conversion to be
/// potentially-lossy. This helps a maintainer who might otherwise wonder if the cast were an
/// oversight. In addition, this trait allows for use in generic contexts, and enables
/// implementation of lossy casts on non-primitive types where applicable.
pub trait LossyCast {
    /// Casts this numerical type to type `T`, ignoring any errors. For conversions between
    /// primitive types this is guaranteed to return the same value as using the `as` keyword.
    /// Depending on the usage, it may be necessary to disambiguate the target type. This cast
    /// should have zero runtime cost.
    ///
    /// # Examples
    /// ```
    /// use cove_num::cast::LossyCast;
    ///
    /// // Call a function `foo` via a lossy cast; no type disambiguation required in this case
    /// fn foo(x: u8) -> u8 {x}
    /// assert_eq!(foo(7u32.lossy_cast()), 7u8);
    ///
    /// // Explicit disambiguation via turbofish required in this case
    /// assert_eq!(7u32.lossy_cast::<u8>(), 7u8);
    ///
    /// // Cast an integer to a float; this happens to be lossless
    /// assert_eq!(6i8.lossy_cast::<f64>(), 6f64);
    ///
    /// // Cast a float to an integer lossily without detecting the loss
    /// assert_eq!(6.3f32.lossy_cast::<i32>(), 6);
    /// ```
    fn lossy_cast<T>(self) -> T where Self: Sized + private::LossyCastImpl<T> {
        self.lossy_cast_impl()
    }
}

/// Provides a mechanism for infallibly casting between numerical types. This is spiritually
/// similar to [`From`]/[`Into`] but differs slightly. The main difference is that this works
/// with `usize`/`isize` on a per-platform basis. For example, on a 64-bit platform this may be
/// used to cast a u64 to a usize, while on a 32-bit platform the same cast will not compile. So
/// where [`From`]/[`Into`] are most concerned with cross-platform portability, `LosslessCast` is
/// more interested in providing casts on the target platform. Be aware of this tradeoff when
/// considering which mechanism to use. As a rule of thumb, if you have concrete types of fixed
/// size you should probably favor [`From`/`Into`].
pub trait LosslessCast {
    /// Casts this numerical type to type `T` in a fashion guaranteed to be lossless on the target
    /// platform. Be advised that this may not compile on a different target platform. Depending on
    /// the usage, it may be necessary to disambiguate the target type. This cast should have zero
    /// runtime cost.
    ///
    /// # Examples
    /// ```
    /// use cove_num::cast::LosslessCast;
    ///
    /// // Call a function `foo` via a lossless cast; no type disambiguation required in this case
    /// fn foo(x: u32) -> u32 {x}
    /// assert_eq!(foo(7u8.lossless_cast()), 7u32);
    ///
    /// // Explicit disambiguation via turbofish required in this case
    /// assert_eq!(7u8.lossless_cast::<u32>(), 7u32);
    ///
    /// // Cast an integer to a float losslessly
    /// assert_eq!(6i8.lossless_cast::<f64>(), 6f64);
    ///
    /// // Widen a signed integer losslessly
    /// assert_eq!(-3i16.lossless_cast::<i32>(), -3i32);
    /// ```
    ///
    /// ```compile_fail
    /// use cove_num::cast::LosslessCast;
    ///
    /// // Cast a float to an integer losslessly -- OOPS, won't compile on any platform since this
    /// // cannot be guaranteed to be lossless at compile time
    /// assert_eq!(6.3f32.lossless_cast::<i32>(), 6);
    /// ```
    ///
    #[cfg_attr(target_pointer_width = "64", doc = "```")]
    #[cfg_attr(not(target_pointer_width = "64"), doc = "```compile_fail")]
    /// use cove_num::cast::LosslessCast;
    ///
    /// // Cast a u64 to usize; compiles on platforms where usize is 64 bits, but not 16 or 32
    /// assert_eq!(8u64.lossless_cast::<usize>(), 8usize);
    ///
    /// ```
    ///
    #[cfg_attr(any(target_pointer_width = "16", target_pointer_width = "32"), doc = "```")]
    #[cfg_attr(not(any(target_pointer_width = "16", target_pointer_width = "32")), doc = "```compile_fail")]
    /// use cove_num::cast::LosslessCast;
    ///
    /// // Cast an isize to i32; compiles on platforms where isize is 16 or 32 bits, but not 64
    /// assert_eq!(8isize.lossless_cast::<i32>(), 8usize);
    ///
    /// ```
    fn lossless_cast<T>(self) -> T where Self: Sized + private::LosslessCastImpl<T> {
        self.lossless_cast_impl()
    }
}

/// Provides a mechanism for casting between integer types; the cast is either lossless or saturates
/// to the target type's MIN or MAX, whichever is closest to the source. This is not provided for
/// floating point types due to ambiguity in semantics (e.g., what does it mean to saturate NaN to
/// an integer?); consider using an alternate cast for floats, such as [`LossyCast`].
pub trait SaturatingCast {
    /// Casts this integer type to type `T`, yielding the closest possible value for `T`.
    /// Concretely, if self < T::MIN this will return T::MIN; if self > T::MAX this will return
    /// T::MAX, and otherwise this will return the same value as `self` but as type `T`. Depending
    /// on the usage, it may be necessary to disambiguate the target type.
    ///
    /// # Examples
    /// ```
    /// use cove_num::cast::SaturatingCast;
    ///
    /// // Call a function `foo` via saturating casts; no type disambiguation required in this case
    /// fn foo(x: u8) -> u8 {x}
    /// assert_eq!(foo(7u32.saturating_cast()), 7u8);
    /// assert_eq!(foo(300u32.saturating_cast()), u8::MAX);
    ///
    /// // Explicit disambiguation via turbofish required in this case
    /// assert_eq!(7u32.saturating_cast::<u8>(), 7u8);
    /// assert_eq!(300u32.saturating_cast::<u8>(), u8::MAX);
    ///
    /// // Cast signed to unsigned
    /// assert_eq!(6i8.saturating_cast::<u32>(), 6u32);
    /// assert_eq!((-6i8).saturating_cast::<u32>(), u32::MIN);
    ///
    /// ```
    fn saturating_cast<T>(self) -> T where Self: Sized + private::SaturatingCastImpl<T> {
        self.saturating_cast_impl()
    }
}

/// Represents that a value of type `From` lost data when cast to type `To`
#[derive(Copy, Clone, Debug)]
pub struct CastError<CastFrom, CastTo> {
    /// The original value before the cast
    pub from: CastFrom,

    /// The value after the cast
    pub to: CastTo
}

impl<CastFrom: Display, CastTo: Display> Display for CastError<CastFrom, CastTo> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "Numerical cast was lossy [{} ({}) -> {} ({})]",
            self.from, stringify!(FromType),
            self.to, stringify!(ToType)
        )
    }
}

impl<CastFrom: Debug + Display, CastTo: Debug + Display> Error for CastError<CastFrom, CastTo> {}

// -- Implementation -- //
mod private {
    use super::CastError;

    /// This is a helper trait for Cast; it provides the underlying implementation of Cast, but is
    /// more annoying to use directly due to needing more type inference syntax. This is only leaked
    /// to the public interface to satisfy the compiler and should not be treated as stable.
    #[doc(hidden)]
    pub trait CastImpl<T> {
        fn cast_impl(self) -> Result<T, CastError<Self, T>> where Self: Sized;
    }

    /// This is a helper trait for LossyCast; it provides the underlying implementation of
    /// LossyCast, but is more annoying to use directly due to needing more type inference syntax.
    /// This is only leaked to the public interface to satisfy the compiler and should not be
    /// treated as stable.
    #[doc(hidden)]
    pub trait LossyCastImpl<T> {
        fn lossy_cast_impl(self) -> T;
    }

    /// This is a helper trait for LossyCast; it provides the underlying implementation of
    /// LosslessCast, but is more annoying to use directly due to needing more type inference
    /// syntax. This is only leaked to the public interface to satisfy the compiler and should
    /// not be treated as stable.
    #[doc(hidden)]
    pub trait LosslessCastImpl<T> {
        fn lossless_cast_impl(self) -> T;
    }

    /// This is a helper trait for SaturatingCast; it provides the underlying implementation of
    /// SaturatingCast, but is more annoying to use directly due to needing more type inference
    /// syntax. This is only leaked to the public interface to satisfy the compiler and should
    /// not be treated as stable.
    #[doc(hidden)]
    pub trait SaturatingCastImpl<T> {
        fn saturating_cast_impl(self) -> T;
    }
}

macro_rules! cast {
    ($($num:ty),+) => {
        $(
            impl Cast for $num {}
            impl LossyCast for $num {}
            impl LosslessCast for $num {}
            impl SaturatingCast for $num {}
        )*

        // All casts can be lossy, so generate the LossyCastImpls in n-squared fashion
        cast!(lossy $($num),* => ($($num),*));
    };

    (@lossy $from:ty => ($($to:ty),+)) => {
        $(
            impl private::LossyCastImpl<$to> for $from {
                fn lossy_cast_impl(self) -> $to {
                    self as $to
                }
            }
        )*
    };

    (lossy $($from:ty),+ => $args:tt) => {
        $(cast!(@lossy $from => $args);)*
    };

    (integer $from:ty => $($to:ty),+) => {
        $(
            impl private::CastImpl<$to> for $from {
                fn cast_impl(self) -> Result<$to, CastError<Self, $to>> {
                    self.try_into().map_err(|_| CastError {
                        from: self,
                        to: self as $to
                    })
                }
            }

            impl private::SaturatingCastImpl<$to> for $from {
                fn saturating_cast_impl(self) -> $to {
                    // Attempt the cast
                    self.try_into().unwrap_or_else(|_| {
                        // Cast failed; if this is less than 0 use the target's MIN, otherwise
                        // use its MAX. This logic cannot be used in general for saturation but
                        // holds for all types actually fed to this macro. Note that the branch
                        // will be optimized away for unsigned source types, at least in release.
                        #[allow(unused_comparisons)]
                        match self < 0 {
                            true => <$to>::MIN,
                            false => <$to>::MAX
                        }
                    })
                }
            }
        )*
    };

    (floating $from:ty => $($to:ty),+) => {
        $(
            impl private::CastImpl<$to> for $from {
                fn cast_impl(self) -> Result<$to, CastError<Self, $to>> {
                    // Because TryFrom/TryInto is not implemented for floating point, we test
                    // for lossy conversions by casting to the target and back, then checking
                    // whether any data was lost.
                    match self == (self as $to as $from) {
                        true => Ok(self as $to),
                        false => Err(CastError {
                            from: self,
                            to: self as $to
                        })
                    }
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
            impl private::LosslessCastImpl<$to> for $from {
                fn lossless_cast_impl(self) -> $to {
                    self as $to
                }
            }
        )*
    };

    (lossless $first:ty, $($from:ty),+ => $to:ty) => {
        cast!(lossless $first => $to);
        $(cast!(lossless $from => $to));*;
    };
}

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