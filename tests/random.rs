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

    for _ in 0 .. 100000 {
        let float = f32::from_bits(value);
        check_cast::<f32, i8>(float);
        check_cast::<f32, i16>(float);
        check_cast::<f32, i32>(float);
        check_cast::<f32, i64>(float);
        check_cast::<f32, i128>(float);
        check_cast::<f32, u8>(float);
        check_cast::<f32, u16>(float);
        check_cast::<f32, u32>(float);
        check_cast::<f32, u64>(float);
        check_cast::<f32, u128>(float);

        value = random_next(value);
    }
}

fn check_cast<
    FROM: Copy + Display + Cast + CastImpl<TO, Error = LossyCastError<FROM, TO>>, 
    TO: Copy + Display
>(from: FROM) {
    let result = from.cast::<TO>();
    let to = result.lossy();

    // Determines whether two numbers are equal through a bit of an unorthodox method:
    // comparing their formatted view. This allows us to not depend on casting, which is after all
    // what we are testing. Start by formatting the values into buffer.
    let mut from_buffer = FixedString::<256>::new();
    write!(&mut from_buffer, "{from:.64}").unwrap();

    let mut to_buffer = FixedString::<256>::new();
    write!(&mut to_buffer, "{to:.64}").unwrap();

    // Normalize the strings and compare
    let from_text = normalize(from_buffer.as_str());
    let to_text = normalize(to_buffer.as_str());
    
    #[allow(clippy::match_bool)]
    match result.is_ok() {
        true if from_text == to_text => {},
        true => panic!(
            "Called lossy cast lossless [{from_text}_{} -> {to_text}_{}]",
            core::any::type_name::<FROM>(),
            core::any::type_name::<TO>()
        ),
        false if from_text == to_text => panic!(
            "Called lossless cast lossy [{from_text}_{} -> {to_text}_{}]",
            core::any::type_name::<FROM>(),
            core::any::type_name::<TO>()
        ),
        false => {}
    }
}

/// "Normalize" the string representation of a number; this means the following:
/// * -0 is reduced to just 0
/// * Trailing zeroes after a decimal point are removed; if that is all the digits after the
/// decimal point, the decimal point is also removed.
fn normalize(value: &str) -> &str {
    // If there a decimal point, trim trailing zeros and possibly the decimal point as well
    #[allow(clippy::match_bool)]
    let value = match value.find('.').is_some() {
        true => value.trim_end_matches('0').trim_end_matches('.'),
        false => value
    };

    // Special case fix-ups
    match value {
        "-0" => "0",
        _ => value
    }
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

