//! This example exists as a sort of compile-time benchmark. It is designed to be used with the
//! cargo-show-asm subcommand (invoked as `cargo asm`). Its purpose is to allow for easier
//! comparison of the generated assembly when using the casts provided by cove as opposed to the
//! `as` keyword. The goal is to increase certainty that the relevant casts are zero-overhead
//! compared with `as`. The examination of the generated assembly is not automatic; manual analysis
//! is required, including possibly adjusting the tests.
//!
//! Note that not all cove casts are intended to be zero-overhead; these are generally not examined
//! here. Moreover, the zero-overhead designation only applies to release (i.e. optimized) builds.
//! See the cove documentation for a more detailed discussion of the performance implications of
//! cove.
//!
//! # `no_std`
//! To use these tests without std support, it is recommended to implement the `no_std` version of
//! the `unpredictable` function stub provided in this file. The tests should function without doing
//! so, but there would be less certainty that the results are useful in the face of aggressive
//! optimizations.

use cove::prelude::*;
use cove::bounds::CastTo;

/// Integer-to-integer narrowing conversions for comparing generated ASM; use
/// `cargo asm --example asm cast_u32_to_u8` to compare. When comparing assembly, it may be helpful
/// to comment out individual lines.
#[inline(never)]
#[allow(clippy::cast_possible_truncation)]
fn cast_u32_to_u8(value: u32) {
    core::hint::black_box(value as u8);
    core::hint::black_box(value.cast::<u8>().lossy());
    core::hint::black_box(value.cast::<u8>().assumed_lossless());
}

/// Integer-to-integer widening conversions for comparing generated ASM; use
/// `cargo asm --example asm cast_u8_to_u32` to compare. When comparing assembly, it may be helpful
/// to comment out individual lines.
#[inline(never)]
#[allow(clippy::cast_lossless)]
fn cast_u8_to_u32(value: u8) {
    core::hint::black_box(value as u32);
    core::hint::black_box(value.cast::<u32>().lossy());
    core::hint::black_box(value.cast::<u32>().assumed_lossless());
    core::hint::black_box(value.cast::<u32>().lossless());
}

/// Integer-to-float conversions for comparing generated ASM; use
/// `cargo asm --example asm cast_u64_to_f32` to compare. When comparing assembly, it may be helpful
/// to comment out individual lines.
#[inline(never)]
#[allow(clippy::cast_precision_loss)]
fn cast_u64_to_f32(value: u64) {
    core::hint::black_box(value as f32);
    core::hint::black_box(value.cast::<f32>().lossy());
    core::hint::black_box(value.cast::<f32>().assumed_lossless());
}

/// Float-to-integer conversions for comparing generated ASM; use
/// `cargo asm --example asm cast_f64_to_i16` to compare. When comparing assembly, it may be helpful
/// to comment out individual lines.
#[inline(never)]
#[allow(clippy::cast_possible_truncation)]
fn cast_f64_to_i16(value: f64) {
    core::hint::black_box(value as i16);
    core::hint::black_box(value.cast::<i16>().lossy());
    core::hint::black_box(value.cast::<i16>().assumed_lossless());
}

/// Provides a basic driver for invoking the casts
fn main() {
    cast_u32_to_u8(unpredictable());
    cast_u8_to_u32(unpredictable());
    cast_u64_to_f32(unpredictable());
    cast_f64_to_i16(unpredictable());
}

/// Provides a value which cannot be known at compile time, to help disable optimizations
#[cfg(feature = "std")]
#[allow(clippy::cast_possible_truncation)]
fn unpredictable<T>() -> T where u128: CastTo<T> {
    use std::time::SystemTime;

    // Just use the count of milliseconds since the epoch
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH).unwrap()
        .as_millis()
        .cast()
        .lossy()
}

/// Provides a no_std variant of `unpredictable`; this is a placeholder which should be filled in
/// as appropriate for the target system. It should produce a value which cannot be known at
/// compile time, to help disable optimizations.
#[cfg(not(feature = "std"))]
fn unpredictable<T>() -> T where u8: CastTo<T> {
    // Placeholder value chosen arbitrarily
    12u8.cast().lossy()
}