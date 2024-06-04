//! Provides extension traits to help make numerical casts safer and more explicit
//!
//! See the [`crate documentation`](crate) for an overview, or jump right in with the [`Cast`]
//! trait.

use crate::base::CastImpl;
use crate::errors::LossyCastError;

/// Extension trait for fallibly casting between numerical types with error detection
///
/// This is spiritually similar to [`TryFrom`]/[`TryInto`], but offers a few advantages.
/// Specifically, its narrower focus allows it to provide a richer error type and it is implemented
/// for additional conversions, such as to and from floating point numbers. Moreover, this enables
/// the use of the follow-on extension traits [`AssumedLossless`] / [`Closest`] / [`Lossless`] /
/// [`Lossy`].
///
/// # Support
/// Cove provides support for [`Cast`] for the following numerical types; `NonZero*` refers to all
/// the non-zero integers defined in [`core::num`].
///
/// | Origin Type       | Target Type       | Error Type                                          |
/// | ---               | ---               | ---                                                 |
/// | all primitives    | all primitives    | [`LossyCastError`]                                  |
/// | `NonZero*`        | all primitives    | [`LossyCastError`]                                  |
/// | all primitives    | `NonZero*`        | [`FailedCastError`](crate::errors::FailedCastError) |
///
/// # NaN
/// Casting from NaN always returns Err, even when the target type can represent NaN (e.g. is also
/// a floating point type, or even the same floating point type as the source of the cast).
pub trait Cast {
    /// Attempts to cast this numerical type to type `T`. Depending on the calling context, it
    /// may be necessary to disambiguate the target type, as with the turbofish operator (::<>).
    ///
    /// It is often possible to call a follow-on extension trait on the returned [`Result`]. See
    /// the documentation for the various traits for details:
    ///
    /// * [`AssumedLossless`]: for when the cast is expected to always be lossless
    /// * [`Closest`]: for when an approximation of the value is acceptable
    /// * [`Lossless`]: for when the cast is guaranteed at compile time to be lossless
    /// * [`Lossy`]: for niche circumstances when behavior akin to `as` is desirable
    ///
    /// # Examples
    /// ```
    /// use cove::prelude::*;
    ///
    /// // Call a function `foo` via a cast; no type disambiguation required in this case
    /// fn foo(x: u8) -> u8 {x}
    /// assert_eq!(foo(7u32.cast()?), 7u8);
    ///
    /// // Explicit disambiguation via turbofish required in this case
    /// assert_eq!(7u32.cast::<u8>()?, 7u8);
    /// # Ok::<(), cove::errors::LossyCastError<u32, u8>>(())
    /// ```
    ///
    /// ```
    /// use cove::prelude::*;
    ///
    /// // Cast a float to an integer losslessly
    /// assert_eq!(6f64.cast::<i8>()?, 6);
    /// # Ok::<(), cove::errors::LossyCastError<f64, i8>>(())
    /// ```
    ///
    /// ```
    /// use cove::prelude::*;
    ///
    /// // Cast a float to an integer lossily, extracting the lossy value from the error
    /// assert_eq!(6.3f32.cast::<i32>().unwrap_err().to, 6);
    /// # Ok::<(), cove::errors::LossyCastError<f32, i32>>(())
    /// ```
    ///
    /// ```
    /// # fn foo() -> Option<()> {
    /// use cove::prelude::*;
    /// use core::num::{NonZeroU8, NonZeroU32};
    ///
    /// // Cast a NonZeroU8 to NonZeroU32 losslessly
    /// assert_eq!(NonZeroU8::new(7)?.cast(), Ok(NonZeroU32::new(7)?));
    /// # Some(()) }
    /// # let _ = foo();
    /// ```
    ///
    /// # Errors
    /// Returns `Err` if the cast is lossy, meaning that the numerical value (in the
    /// mathematical sense) is not preserved completely across the type cast. The error type is
    /// defined by the implementation; for implementations provided by Cove this will be
    /// [`LossyCastError`] or
    /// [`FailedCastError`](crate::errors::FailedCastError).
    ///
    /// # Performance
    /// In an optimized build, [`Cast::cast`] in isolation should generally have performance similar
    /// to [`TryFrom::try_from`] / [`TryInto::try_into`]. Note that performance may actually improve
    /// when follow-on extension traits are applied to the returned [`Result`].
    #[inline]
    fn cast<T>(self) -> Result<T, Self::Error> where Self: Sized + CastImpl<T> {
        self.cast_impl()
    }
}

/// Follow-on extension trait for infallibly casting between numerical types
///
/// As a follow-on extension trait, this is intended to be applied to a [`Result`] returned from
/// [`Cast::cast`]. This trait is spiritually similar to [`From`]/[`Into`] in the sense that it
/// is only defined for casts which are guaranteed at compile time by the involved types to be
/// lossless. The primary difference between [`Lossless`] and [`From`]/[`Into`] is that the former
/// supports casts involving [`usize`] / [`isize`] / [`NonZeroUsize`](core::num::NonZeroUsize) /
/// [`NonZeroIsize`](core::num::NonZeroIsize) on a per-platform bases, while the latter does not.
///
/// For example, on a 64-bit platform [`Lossless`] may be used to cast a u64 to a usize, while on a
/// 32-bit platform the same cast will not compile. Compared to [`From`]/[`Into`], therefore,
/// [`Lossless`] sacrifices a measure of portability to gain a broader scope. Be aware of this
/// tradeoff when considering which mechanism to use. As a rule of thumb, you should not use
/// [`Lossless`] if your use case allows for [`From`]/[`Into`]; just use one of those instead.
///
/// # Support
/// Cove provides support for [`Lossless`] whenever [`From`]/[`Into`] is supported. In
/// addition, it supports casts involving [`usize`] / [`isize`] whenever it is also supported for
/// the corresponding sized primitive on the target platform. For example, on a 64-bit platform
/// [`Lossless`] is supported for [`usize`] / [`isize`] whenever it is also supported for [`u64`]
/// / [`i64`] respectively.
///
/// In addition, [`Lossless`] is supported for casts from the `NonZero*` family in [`core::num`]
/// whenever it is supported for the corresponding primitive. For example, [`Lossless`] is
/// supported for casting [`NonZeroU32`](core::num::NonZeroU32) to
/// [`NonZeroUsize`](core::num::NonZeroUsize) or to [`usize`] on 32-bit and 64-bit platforms
/// because it is also supported for casting [`u32`] to [`usize`] on those platforms.
///
/// Note that [`Lossless`] does not support casting from primitives to the `NonZero*` family,
/// since the origin value could be zero.
pub trait Lossless<T> {
    /// Unwraps a [`Result`] returned from [`Cast::cast`], extracting its `Ok` variant with no
    /// possibility of panic. Will only compile for casts for which this guarantee can be made on
    /// the target platform. May not be portable to other target platforms.
    ///
    /// # Examples
    /// ```
    /// use cove::prelude::*;
    /// use core::num::{NonZeroI16, NonZeroU8};
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
    ///
    /// // Cast between NonZero types losslessly
    /// assert_eq!(
    ///     NonZeroU8::new(5).unwrap().cast::<NonZeroI16>().lossless(),
    ///     NonZeroI16::new(5).unwrap()
    /// );
    ///
    /// // Cast from NonZero to primitive losslessly
    /// assert_eq!(NonZeroU8::new(19).unwrap().cast::<usize>().lossless(), 19usize);
    /// ```
    ///
    /// ```compile_fail
    /// use cove::prelude::*;
    ///
    /// // Cast a float to an integer losslessly -- OOPS, won't compile on any platform since this
    /// // cannot be guaranteed to be lossless at compile time
    /// assert_eq!(6.3f32.cast::<i32>().lossless(), 6);
    /// ```
    ///
    #[cfg_attr(target_pointer_width = "64", doc = "```")]
    #[cfg_attr(not(target_pointer_width = "64"), doc = "```compile_fail")]
    /// use cove::prelude::*;
    ///
    /// // Cast a u64 to usize; compiles on platforms where usize is 64 bits, but not 16 or 32
    /// assert_eq!(8u64.cast::<usize>().lossless(), 8usize);
    ///
    /// ```
    ///
    #[cfg_attr(any(target_pointer_width = "16", target_pointer_width = "32"), doc = "```")]
    #[cfg_attr(not(any(target_pointer_width = "16", target_pointer_width = "32")), doc = "```compile_fail")]
    /// use cove::prelude::*;
    ///
    /// // Cast an isize to i32; compiles on platforms where isize is 16 or 32 bits, but not 64
    /// assert_eq!(8isize.cast::<i32>().lossless(), 8usize);
    ///
    /// ```
    ///
    /// # Performance
    /// In an optimized build, the combination of [`Cast::cast`] and [`Lossless::lossless`]
    /// generally compiles to the same assembly as the `as` keyword and thus is zero-overhead.
    fn lossless(self) -> T;
}

/// Follow-on extension trait for accepting the result of a [`Cast::cast`], even if it was lossy
///
/// As a follow-on extension trait, this is intended to be applied to a [`Result`] returned from
/// [`Cast::cast`]. This trait is spiritually similar to the `as` keyword; indeed, for primitive
/// casts it is guaranteed to return the same value as having used `as`. Similarly, for casts
/// from the `NonZero*` family defined in [`core::num`] to primitives it is guaranteed to return
/// the same value as calling `.get()` and then using `as`.
///
/// This trait offers a few advantages over the `as` keyword. Foremost among these is to improve
/// self-documentation of code by expressing that the author intended the conversion to be
/// potentially-lossy. This helps a maintainer who might otherwise wonder if the cast were an
/// oversight. In addition, this trait allows for use in generic contexts, and enables
/// implementation of lossy casts on non-primitive types where applicable. Finally, it is more
/// easily-searchable in a codebase than the `as` keyword, which is overloaded for non-numerical
/// casts. This is relevant because it could be a source of errors.
///
/// [`Lossy`] is rarely the correct cast for a given situation. In almost all use cases it is
/// better to use [`From`]/[`Into`], [`Cast`], or one of the other follow-on extension traits
/// provided by cove. That said, [`Lossy`] should usually be preferred over the raw `as` keyword;
/// see the [`crate guidelines`](crate#guidelines) for more discussion on this topic.
///
/// An example of a legitimate use case for [`Lossy`] appears when working with API calls which
/// use specific primitive values as meaningful constants, but are inconsistent about which type
/// to give those values. This comes up not infrequently when working with the Win32 API, which
/// might take an [`i32`] as a function parameter but supply the constant definition as a [`u32`].
/// In this case the fact that the mathematical value is changing is irrelevant so [`Lossy`] is
/// appropriate.
///
/// # Support
/// Cove provides support for [`Lossy`] whenever [`Cast::cast`] returns a [`Result`] based on
/// [`LossyCastError`]. In practice this means [`Lossy`] is supported for all cove-provided casts 
/// except from a primitive to one of the `NonZero*` family defined in [`core::num`].
pub trait Lossy<T> {
    /// Called on a [`Result`] returned from [`Cast::cast`] to accept the result of the cast, even
    /// if it was lossy. This is essentially a convenience wrapper around unwrapping in the success
    /// case or extracting [`LossyCastError::to`](crate::errors::LossyCastError::to) in the fail
    /// case.
    ///
    /// # Examples
    /// ```
    /// use cove::prelude::*;
    /// use core::num::NonZeroI32;
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
    ///
    /// // Also works for NonZero* to primitive, but not primitive to NonZero*
    /// assert_eq!(NonZeroI32::new(-300).unwrap().cast::<i8>().lossy(), -44i8);
    /// ```
    ///
    /// # Performance
    /// In an optimized build, the combination of [`Cast::cast`] and [`Lossy::lossy`] generally
    /// compiles to the same assembly as the `as` keyword and thus is zero-overhead.
    fn lossy(self) -> T;
}


/// Follow-on extension trait for converting the result of a [`Cast::cast`] into the closest
/// possible value
///
/// As a follow-on extension trait, this is intended to be applied to a [`Result`] returned from
/// [`Cast::cast`]. When the cast is lossless (that is, `Ok` is returned), this just returns
/// the casted value. Otherwise, this converts the origin value to the closest value expressible in
/// the target type and returns that.
///
/// Note that 'closest' does not imply what is colloquially-understood to mean 'close'. For
/// example:
///
/// ```
/// # use cove::prelude::*;
/// assert_eq!(1_000_000_000u64.cast::<u8>().closest(), 255u8);
/// ```
///
/// When used to cast between integers, [`Closest`] is effectively a saturating cast; that is, in
/// those cases it will return either the exact value or the `MAX` or `MIN` of the target type as
/// appropriate.
///
/// If more than one value of the target type is equidistant from the origin value, the
/// implementation is free to choose any of the nearest values; there is no guarantee which one
/// will be chosen in the general case. However, the following guarantees are made in specific
/// cases; note that `NonZero*` refers to the family of non-zero integers defined in [`core::num`]:
///
/// | Origin Types      | Target Types          | Guarantee                                     |
/// | ---               | ---                   | ---                                           |
/// | float             | int                   | rounded with `.5` rounding away from 0        |
/// | float             | unsigned `NonZero*`   | float → int, then ±0.0 → 1                    |
/// | float             | signed `NonZero*`     | float → int, then -0.0 → -1 and +0.0 → 1      |
/// | float: NaN        | float                 | target will also be NaN                       |
/// | int or `NonZero*` | float                 | rounded according to `roundTiesToEven` mode*  |
/// | int               | `NonZero*`            | 0 → 1                                         |
///
/// *as defined in `IEEE 754-2008 §4.3.1`: pick the nearest floating point number, preferring the
/// one with an even least significant digit if exactly halfway between two floating point numbers.
/// This is taken directly from the behavior specified for the `as` keyword.
///
/// # Support
/// Cove provides support for [`Closest`] for all casts supported by [`Cast::cast`]; that is,
/// between all primitives and members of the `NonZero*` family defined in [`core::num`].
pub trait Closest<T> {
    /// Called on a [`Result`] returned from [`Cast::cast`] to accept the closest value
    /// expressible in the target type, even if it was lossy.
    ///
    /// # Examples
    /// ```
    /// use cove::prelude::*;
    /// use core::num::{NonZeroU16, NonZeroI32};
    ///
    /// // Call a function `foo` via a cast; no type disambiguation required in this case
    /// fn foo(x: u8) -> u8 {x}
    /// assert_eq!(foo(7u32.cast().closest()), 7u8);
    ///
    /// // The closest value in lossy integer to integer casts is MAX or MIN
    /// assert_eq!((-5000i64).cast::<i8>().closest(), -128i8);
    /// assert_eq!(71234.cast::<NonZeroU16>().closest(), NonZeroU16::MAX);
    ///
    /// // Floats will round as needed
    /// assert_eq!(5.4f32.cast::<isize>().closest(), 5isize);
    /// assert_eq!(5.5f32.cast::<isize>().closest(), 6isize);
    ///
    /// // Cast to the closest NonZero value possible
    /// assert_eq!(0u8.cast::<NonZeroU16>().closest(), NonZeroU16::new(1).unwrap());
    /// assert_eq!((-0.0f64).cast::<NonZeroI32>().closest(), NonZeroI32::new(-1).unwrap());
    /// assert_eq!((0.0f64).cast::<NonZeroI32>().closest(), NonZeroI32::new(1).unwrap());
    /// ```
    ///
    /// # Performance
    /// [`Closest::closest`] is generally **NOT** zero-overhead compared to the `as` keyword, as
    /// it involves at least one branch. That said, it is sufficiently lightweight that only in
    /// very rare cases would its performance be relevant.
    fn closest(self) -> T;
}

/// Follow-on extension trait for assuming that the result of a [`Cast::cast`] is lossless
///
/// As a follow-on extension trait, this is intended to be applied to a [`Result`] returned from
/// [`Cast::cast`]. When the cast is lossless (that is, `Ok` is returned), this just returns
/// the casted value. Otherwise, this will panic in a build with `debug_assertions` turned on,
/// which is the default for dev builds but not release builds. If `debug_assertions` are not
/// turned on, this accepts the lossy value in the same fashion as [`Lossy`].
///
/// The intended use case for [`AssumedLossless`] are those circumstances when the programmer can
/// determine that a cast will always be lossless but the compiler cannot. It offers some
/// advantages over alternate methods:
///
/// * Compared to the `as` keyword, [`AssumedLossless`] works in generic contexts, documents intent
/// better, and will help catch bugs by panicking in dev builds if the assumption was incorrect
/// * Compared to [`Result::unwrap`] after using [`Cast`] / [`TryFrom`] / [`TryInto`],
/// [`AssumedLossless`] offers a minor performance improvement in release builds
/// * Compare to [`Result::unwrap_unchecked`] after using [`Cast`] / [`TryFrom`] / [`TryInto`],
/// [`AssumedLossless`] offers an approach that does not invoke undefined behavior in case of a bug
///
/// Consider using [`From`] / [`Into`] or [`Lossless`] instead of [`AssumedLossless`] if the
/// compiler can verify from the types alone that a cast will be lossless.
///
/// # Support
/// Cove provides support for [`AssumedLossless`] whenever [`Cast::cast`] returns a [`Result`] based
/// on [`LossyCastError`]. In practice this means [`AssumedLossless`] is supported for all 
/// cove-provided casts except from a primitive to one of the `NonZero*` family defined in 
/// [`core::num`].
pub trait AssumedLossless<T> {
    /// Called on a [`Result`] returned from [`Cast::cast`] to accept the result of the cast
    /// under the assumption that it was lossless. This will panic in dev builds if the cast was
    /// actually lossy but will use the lossy value in release builds.
    ///
    /// # Examples
    /// ```
    /// use cove::prelude::*;
    /// use core::num::NonZeroI32;
    ///
    /// // Call a function `foo` via a cast; no type disambiguation required in this case
    /// fn foo(x: u8) -> u8 {x}
    /// assert_eq!(foo(7u32.cast().assumed_lossless()), 7u8);
    ///
    /// // Assume the results of the cast are lossless
    /// assert_eq!(13f32.cast::<usize>().assumed_lossless(), 13usize);
    ///
    /// // Also works for NonZero* to primitive, but not primitive to NonZero*
    /// assert_eq!(NonZeroI32::new(42).unwrap().cast::<i8>().assumed_lossless(), 42i8);
    /// ```
    ///
    #[cfg_attr(debug_assertions, doc = "```should_panic")]
    #[cfg_attr(not(debug_assertions), doc = "```")]
    /// use cove::prelude::*;
    ///
    /// // Incorrectly assume a lossy cast is lossless; this will panic in a dev build and yield a
    /// // lossy value in a release build
    /// assert_eq!((-4isize).cast::<u8>().assumed_lossless(), 252u8);
    ///
    /// ```
    ///
    /// # Performance
    /// In an optimized build, the combination of [`Cast::cast`] and
    /// [`AssumedLossless::assumed_lossless`] generally compiles to the same assembly as the `as`
    /// keyword and thus is zero-overhead.
    fn assumed_lossless(self) -> T;
}