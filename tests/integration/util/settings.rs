/// Specifies how many random test iterations to perform for slow tests
pub const SLOW_ITERATIONS: usize = 1000;

/// Specifies how many random test iterations to perform for fast tests
pub const FAST_ITERATIONS: usize = 1_000_000;

/// Provides a seed to the RNG for random tests when on no_std
#[cfg(not(feature = "std"))]
pub const RANDOM_SEED: u32 = 0;