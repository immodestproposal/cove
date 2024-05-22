mod util;

use cove::prelude::*;
use core::fmt::{Display, Write};
use cove::base::CastImpl;
use cove::errors::LossyCastError;
use util::FixedString;

#[test]

fn random() {
    // Determine the initial seed
    let mut value = random_seed();

    for _ in 0 .. 10 {
        //check_cast::<f32, i64>(f32::from_bits(value));
        check_cast::<f32, i64>(22_080_868f32);
        value = random_next(value);
    }
}

fn check_cast<
    FROM: Copy + Display + Cast + CastImpl<TO, Error = LossyCastError<FROM, TO>>, 
    TO: Copy + Display
>(from: FROM) {
    let result = from.cast::<TO>();
    let to = result.lossy();

    println!("[MATT] Checking {} -> {} [Lossless: {}]", from, to, result.is_ok());
    #[allow(clippy::match_bool)]
    match result.is_ok() {
        true if are_equal(from, to) => {},
        true => panic!("Called lossy cast lossless [{from} -> {to}]"),
        false if are_equal(from, to) => panic!("Called lossless cast lossy [{from} -> {to}]"),
        false => {}
    }
}

/// Determines whether two numbers are equal through a bit of an unorthodox method: comparing their
/// formatted view. This allows us to not depend on casting, which is after all what we are testing.
fn are_equal(from: impl Display, to: impl Display) -> bool {
    // Format the values into buffers
    let mut from_buffer = FixedString::<128>::new();
    write!(&mut from_buffer, "{from}").unwrap();

    let mut to_buffer = FixedString::<128>::new();
    write!(&mut to_buffer, "{to}").unwrap();

    // Compare based on string value
    from_buffer.as_str() == to_buffer.as_str()
}

/// Creates a seed to a PRNG based on the current system time; note that this a terrible way to
/// generate numbers that are truly close to random, but is good enough for our purposes and avoids
/// introducing dependencies on crates outside of std.
#[cfg(feature = "std")]
#[allow(clippy::cast_possible_truncation)]
fn random_seed() -> u32 {
    use std::time::SystemTime;

    // Take the milliseconds since the epoch and truncate to the least-significant (and therefore
    // most volatile over time) digits
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
        as u32
}

/// Provides a hardcoded seed that can be manually edited, for use with no_std
#[cfg(not(feature = "std"))]
#[allow(clippy::cast_possible_truncation)]
fn random_seed() -> u32 {
    0
}

/// Creates a new "random" number based on the given `seed` using a simple LCG. Note that this is a
/// terrible way to generate numbers that are truly close to random, but is good enough for our
/// purposes and avoids introducing dependencies on additional crates.
fn random_next(seed: u32) -> u32 {
    // Values come from Numerical Recipes book (summarized by Wikipedia); chosen to produce a
    // full-period generator
    seed
        .wrapping_mul(1_664_525)
        .wrapping_add(1_013_904_223)
}

