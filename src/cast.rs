
use crate::base::CastImpl;
use core::fmt::{Debug, Display, Formatter};

/// Extension trait for fallibly casting between numerical types with error detection
///
/// This is spiritually similar to [`TryFrom`]/[`TryInto`], but offers a few advantages.
/// Specifically, its narrower focus allows it to provide a richer error type and it is implemented
/// for additional conversions, such as to and from floating point numbers.
///
/// This is intended for fallible casting with error detection. If a cast is known to be infallible,
/// consider using [`Lossless`] or [`From`]/[`Into`] instead. If a cast is fallible but there
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
    /// # Ok::<(), cove::LossyCastError<u32, u8>>(())
    /// ```
    ///
    /// ```
    /// use cove::Cast;
    ///
    /// // Cast a float to an integer losslessly
    /// assert_eq!(6f64.cast::<i8>()?, 6);
    /// # Ok::<(), cove::LossyCastError<f64, i8>>(())
    /// ```
    ///
    /// ```
    /// use cove::Cast;
    ///
    /// // Cast a float to an integer lossily, extracting the lossy value from the error
    /// assert_eq!(6.3f32.cast::<i32>().unwrap_err().to, 6);
    /// # Ok::<(), cove::LossyCastError<f32, i32>>(())
    /// ```
    #[inline]
    fn cast<T>(self) -> Result<T, LossyCastError<Self, T>> where Self: Sized + CastImpl<T> {
        self.cast_impl()
    }
}

/// Extension trait for saturating an integer to a target type, applied after a [`Cast::cast`]
///
/// When applied to a cast that was lossless this will simply return the casted value. If lossy, it
/// will return the target type's MIN or MAX, whichever is closest to the source value. This is
/// provided for CastError<F, T> and Result<T, CastError<F, T>> for integer types. It is not
/// provided for floating point types (except f32 ⮕ f64, which is lossless) to avoid ambiguity in
/// semantics (e.g., what does it mean to saturate NaN to an integer?); consider using an alternate
/// cast for floats, such as [`Lossy`] or [`Closest`].
pub trait Saturated<T> {
    /// Called on a CastError<F, T> or Result<T, CastError<F, T>> to yield the closest possible
    /// value of type `T` to the original source value. Concretely, if source < `T::MIN` this will
    /// return `T::MIN`; if source > `T::MAX` this will return `T::MAX`, and otherwise this will
    /// return the source value but as type `T`.
    ///
    /// # Examples
    /// ```
    /// use cove::{Cast, Saturated};
    ///
    /// // Call a function `foo` via a cast; no type disambiguation required in this case
    /// fn foo(x: u8) -> u8 {x}
    /// assert_eq!(foo(7u32.cast().saturated()), 7u8);
    ///
    /// // Saturating after a lossless cast just yields the original value
    /// assert_eq!((-3i32).cast::<i8>().saturated(), -3);
    ///
    /// // Saturating after a lossy cast yields the MIN or MAX, as appropriate
    /// assert_eq!((-3i32).cast::<u8>().saturated(), u8::MIN);
    /// assert_eq!(300u16.cast::<u8>().saturated(), u8::MAX);
    /// ```
    ///
    /// ```compile_fail
    /// use cove::{Cast, Saturated};
    ///
    /// // Attempting to saturate a floating point cast is a compile error; not defined
    /// let _fail = f32::NAN.cast::<u16>().saturated();
    /// ```
    fn saturated(self) -> T;
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
pub trait Lossy<T> {
    /// Called on a Result<T, CastError<F, T>> to accept the result of the cast, even if it was
    /// lossy. This is essentially a convenience wrapper around unwrapping in the success case or
    /// extracting the `to` field of the [`CastError`] in the fail case. For primitives this should
    /// have the same runtime cost as the `as` keyword (that is, none at all), at least in
    /// release builds.
    ///
    /// # Examples
    /// ```
    /// use cove::{Cast, Lossy};
    ///
    /// // Call a function `foo` via a cast; no type disambiguation required in this case
    /// fn foo(x: u8) -> u8 {x}
    /// assert_eq!(foo(7u32.cast().lossy()), 7u8);
    ///
    /// // Accept the results of the cast; in this case, it is lossless anyway
    /// assert_eq!(7f32.cast::<usize>().lossy(), 7usize);
    ///
    /// // Accept the results of the cast; it is lossy but by accepting we discard error information
    /// assert_eq!(7.1f32.cast::<usize>().lossy(), 7usize);
    /// ```
    fn lossy(self) -> T;
}

pub trait Closest<T> {
    fn closest(self) -> T;
}

pub trait AssumeLossless<T> {
    fn assume_lossless(self) -> T;
}

/// Indicates that a cast between numeric types lost data
#[derive(Copy, Clone, Debug)]
pub struct LossyCastError<CastFrom, CastTo> {
    /// The original value before the cast
    pub from: CastFrom,

    /// The value after the cast
    pub to: CastTo
}

impl<CastFrom: Display, CastTo: Display> Display for LossyCastError<CastFrom, CastTo> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            formatter,
            "Numerical cast was lossy [{} ({}) -> {} ({})]",
            self.from, stringify!(FromType),
            self.to, stringify!(ToType)
        )
    }
}

#[cfg(feature = "std")]
impl<CastFrom: Debug + Display, CastTo: Debug + Display>
std::error::Error for LossyCastError<CastFrom, CastTo> {}