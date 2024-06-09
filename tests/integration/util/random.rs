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

/// Generates a random byte buffer of length LEN based on the `seed` using a simple LCG. Returns
/// the byte buffer and a new seed value which can be supplied to future calls.
pub fn random_bytes<const LEN: usize>(mut seed: u32) -> ([u8; LEN], u32) {
    let mut buffer = [0; LEN];
    
    // Copy the random bytes into each chunk in turn
    for chunk in buffer.chunks_mut(std::mem::size_of::<u32>()) {
        chunk.copy_from_slice(&seed.to_ne_bytes()[.. chunk.len()]);
        seed = random_next(seed);
    }
    
    (buffer, seed)
}