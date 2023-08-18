
use crate::base::{CastImpl, LossyCastImpl, LosslessCastImpl};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

/// Extension trait for fallibly casting between numerical types with error detection
///
/// This is spiritually similar to [`TryFrom`]/[`TryInto`], but offers a few advantages.
/// Specifically, its narrower focus allows it to provide a richer error type and it is implemented
/// for additional conversions, such as to and from floating point numbers.
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
    /// use cove::Cast;
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
    #[inline]
    fn cast<T>(self) -> Result<T, CastError<Self, T>> where Self: Sized + CastImpl<T> {
        self.cast_impl()
    }
}

/// Extension trait for fallibly casting between numerical types without error detection
///
/// This is spiritually similar to the `as` keyword but offers a few advantages. Foremost among
/// these is to improve self-documentation of code by expressing that the author intended the
/// conversion to be potentially-lossy. This helps a maintainer who might otherwise wonder if the
/// cast were an oversight. In addition, this trait allows for use in generic contexts, and enables
/// implementation of lossy casts on non-primitive types where applicable.
pub trait LossyCast {
    /// Casts this numerical type to type `T`, ignoring any errors. For conversions between
    /// primitive types this is guaranteed to return the same value as using the `as` keyword.
    /// Depending on the usage, it may be necessary to disambiguate the target type. This cast
    /// has zero runtime cost for primitives.
    ///
    /// # Examples
    /// ```
    /// use cove::LossyCast;
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
    fn lossy_cast<T>(self) -> T where Self: Sized + LossyCastImpl<T> {
        self.lossy_cast_impl()
    }
}

/// Extension trait for infallibly casting between numerical types
///
/// This is spiritually similar to [`From`]/[`Into`] but differs slightly. The main difference is
/// that this works with `usize`/`isize` on a per-platform basis. For example, on a 64-bit platform
/// this may be used to cast a u64 to a usize, while on a 32-bit platform the same cast will not
/// compile. So where [`From`]/[`Into`] are most concerned with cross-platform portability,
/// `LosslessCast` is more interested in providing casts on the target platform. Be aware of this
/// tradeoff when considering which mechanism to use. As a rule of thumb, if you have concrete types
/// of fixed size you should probably favor [`From`]/[`Into`].
pub trait LosslessCast {
    /// Casts this numerical type to type `T` in a fashion guaranteed to be lossless on the target
    /// platform. Be advised that this may not compile on a different target platform. Depending on
    /// the usage, it may be necessary to disambiguate the target type. This cast has zero runtime
    /// cost for primitives.
    ///
    /// # Examples
    /// ```
    /// use cove::LosslessCast;
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
    /// use cove::LosslessCast;
    ///
    /// // Cast a float to an integer losslessly -- OOPS, won't compile on any platform since this
    /// // cannot be guaranteed to be lossless at compile time
    /// assert_eq!(6.3f32.lossless_cast::<i32>(), 6);
    /// ```
    ///
    #[cfg_attr(target_pointer_width = "64", doc = "```")]
    #[cfg_attr(not(target_pointer_width = "64"), doc = "```compile_fail")]
    /// use cove::LosslessCast;
    ///
    /// // Cast a u64 to usize; compiles on platforms where usize is 64 bits, but not 16 or 32
    /// assert_eq!(8u64.lossless_cast::<usize>(), 8usize);
    ///
    /// ```
    ///
    #[cfg_attr(any(target_pointer_width = "16", target_pointer_width = "32"), doc = "```")]
    #[cfg_attr(not(any(target_pointer_width = "16", target_pointer_width = "32")), doc = "```compile_fail")]
    /// use cove::LosslessCast;
    ///
    /// // Cast an isize to i32; compiles on platforms where isize is 16 or 32 bits, but not 64
    /// assert_eq!(8isize.lossless_cast::<i32>(), 8usize);
    ///
    /// ```
    fn lossless_cast<T>(self) -> T where Self: Sized + LosslessCastImpl<T> {
        self.lossless_cast_impl()
    }
}

/// Extension trait for saturating an integer to a target type, applied after a [`Cast::cast`]
///
/// When applied to a cast that was lossless this will simply return the casted value. If lossy,
/// it will return the target type's MIN or MAX, whichever is closest to the source value. This
/// is provided for CastError<F, T> and Result<T, CastError<F, T>> for integer types. It is not
/// provided for floating point types due to ambiguity in semantics (e.g., what does it mean to
/// saturate NaN to an integer?); consider using an alternate cast for floats, such as
/// [`LossyCast`].
pub trait Saturate<T> {
    /// Called on a CastError<F, T> or Result<T, CastError<F, T>> to yield the closest possible
    /// value of type `T` to the original source value. Concretely, if source < T::MIN this will
    /// return T::MIN; if source > T::MAX this will return T::MAX, and otherwise this will return
    /// the source value but as type `T`.
    ///
    /// # Examples
    /// ```
    /// use cove::{Cast, Saturate};
    ///
    /// // Call a function `foo` via a cast; no type disambiguation required in this case
    /// fn foo(x: u8) -> u8 {x}
    /// assert_eq!(foo(7u32.cast().saturate()), 7u8);
    ///
    /// // Saturating after a lossless cast just yields the original value
    /// assert_eq!((-3i32).cast::<i8>().saturate(), -3);
    ///
    /// // Saturating after a lossy cast yields the MIN or MAX, as appropriate
    /// assert_eq!((-3i32).cast::<u8>().saturate(), u8::MIN);
    /// assert_eq!(300u16.cast::<u8>().saturate(), u8::MAX);
    /// ```
    ///
    /// ```compile_fail
    /// use cove::{Cast, Saturate};
    ///
    /// // Attempting to saturate a floating point cast is a compile error; not defined
    /// let _fail = f32::NAN.cast::<u16>().saturate();
    /// ```
    fn saturate(self) -> T;
}

pub trait Lossless<T> {
    /// # Examples
    /// ```
    /// use cove::{Cast, Lossless};
    ///
    /// // Call a function `foo` via a lossless cast; no type disambiguation required in this case
    /// fn foo(x: u32) -> u32 {x}
    /// assert_eq!(foo(7u8.cast().lossless()), 7u32);
    ///
    /// // Explicit disambiguation via turbofish required in this case
    /// assert_eq!(7u8.cast::<u32>().lossless(), 7u32);
    ///
    /// // Cast an integer to a float losslessly
    /// assert_eq!(6i8.cast::<f64>().lossless(), 6f64);
    ///
    /// // Widen a signed integer losslessly
    /// assert_eq!(-3i16.cast::<i32>().lossless(), -3i32);
    /// ```
    ///
    /// ```compile_fail
    /// use cove::{Cast, Lossless};
    ///
    /// // Cast a float to an integer losslessly -- OOPS, won't compile on any platform since this
    /// // cannot be guaranteed to be lossless at compile time
    /// assert_eq!(6.3f32.cast::<i32>().lossless(), 6);
    /// ```
    ///
    #[cfg_attr(target_pointer_width = "64", doc = "```")]
    #[cfg_attr(not(target_pointer_width = "64"), doc = "```compile_fail")]
    /// use cove::{Cast, Lossless};
    ///
    /// // Cast a u64 to usize; compiles on platforms where usize is 64 bits, but not 16 or 32
    /// assert_eq!(8u64.cast::<usize>().lossless(), 8usize);
    ///
    /// ```
    ///
    #[cfg_attr(any(target_pointer_width = "16", target_pointer_width = "32"), doc = "```")]
    #[cfg_attr(not(any(target_pointer_width = "16", target_pointer_width = "32")), doc = "```compile_fail")]
    /// use cove::{Cast, Lossless};
    ///
    /// // Cast an isize to i32; compiles on platforms where isize is 16 or 32 bits, but not 64
    /// assert_eq!(8isize.cast::<i32>().lossless(), 8usize);
    ///
    /// ```
    fn lossless(self) -> T;
}

/// Extension trait for accepting the result of a [`Cast::cast`], even if it was lossy
///
/// This is spiritually similar to the `as` keyword but offers a few advantages. Foremost among
/// these is to improve self-documentation of code by expressing that the author intended the
/// conversion to be potentially-lossy. This helps a maintainer who might otherwise wonder if the
/// cast were an oversight. In addition, this trait allows for use in generic contexts, and enables
/// implementation of lossy casts on non-primitive types where applicable.
pub trait CastResult<T> {
    /// Called on a Result<T, CastError<F, T>> to accept the result of the cast, even if it was
    /// lossy. This is essentially a convenience wrapper around unwrapping in the success case or
    /// extracting the `to` field of the CastError in the fail case. For primitives this should
    /// have the same runtime cost as the `as` keyword (that is, none at all), at least in
    /// release builds.
    ///
    /// # Examples
    /// ```
    /// use cove::{Cast, CastResult};
    ///
    /// // Call a function `foo` via a cast; no type disambiguation required in this case
    /// fn foo(x: u8) -> u8 {x}
    /// assert_eq!(foo(7u32.cast().accept_lossy()), 7u8);
    ///
    /// // Accept the results of the cast; in this case, it is lossless anyway
    /// assert_eq!(7f32.cast::<usize>().accept_lossy(), 7usize);
    ///
    /// // Accept the results of the cast; it is lossy but by accepting we discard error information
    /// assert_eq!(7.1f32.cast::<usize>().accept_lossy(), 7usize);
    /// ```
    fn accept_lossy(self) -> T;

    fn assume_lossless(self) -> T;
}

/// Indicates that a cast between numeric types lost data
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