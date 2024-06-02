/// Creates a seed to a PRNG based on the current system time; note that this a terrible way to
/// generate numbers that are truly close to random, but is good enough for our purposes and avoids
/// introducing dependencies on crates outside of std.
#[cfg(feature = "std")]
#[allow(clippy::cast_possible_truncation)]
pub fn random_seed() -> u32 {
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
pub fn random_seed() -> u32 {
    0
}

/// Creates a new "random" number based on the given `seed` using a simple LCG. Note that this is a
/// terrible way to generate numbers that are truly close to random, but is good enough for our
/// purposes and avoids introducing dependencies on additional crates.
pub fn random_next(seed: u32) -> u32 {
    // Values come from Numerical Recipes book (summarized by Wikipedia); chosen to produce a
    // full-period generator
    seed
        .wrapping_mul(1_664_525)
        .wrapping_add(1_013_904_223)
}