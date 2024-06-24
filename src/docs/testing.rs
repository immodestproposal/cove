//! Provides helpful information about testing cove
//! 
//! # Settings
//! Some of cove's test battery uses randomized inputs, using a simple full-period LCG as the 
//! randomizer. For each test, a random input is generated, a cast is performed, and the output 
//! is validated. The settings file for these tests is available for editing in 
//! `tests/integration/util/settings.rs`.
//! 
//! ## Random Seed
//! When cove is tested with `std` support (the default), the random LCG is seeded from the
//! system time. Without `std`, however, there is no source of entropy for seeding; consequently,
//! when testing without `std` the tests use a fixed seed which can be manually updated by the 
//! tester. To do so, edit the `RANDOM_SEED` constant in the settings file.
//!
//! ## Iteration Count
//! Cove's random tests are repeated for a number of iterations controlled by the 
//! `SLOW_ITERATIONS` and `FAST_ITERATIONS` constants in the settings file.
//! 
//! # Pointer Widths
//! Cove implements a number of platform-independent casts as well as some which depend on the
//! size of the platform's pointer widths (e.g. casts to `usize`). These pointer-width-specific 
//! casts are implemented for 16, 32, 64, and 128-bit platforms along with corresponding tests. 
//! However, they have only actually been tested on 32 and 64-bit platforms due to a lack of 
//! availability of other platforms during cove's development.
//! 
//! # Debug vs Release
//! Cove's behavior can differ according to whether it is built in debug or release; in particular, 
//! the semantics of the [`AssumedLossless`](crate::casts::AssumedLossless) trait differ in debug
//! from release. It can therefore be worthwhile testing cove under both configurations.
//! 
//! # Testing `no_std`
//! Some of cove's tests require more complete panic support than is provided by `core`, and thus
//! are only implemented on `std`.
//! 
//! # Doctests
//! Some of cove's test battery is implemented via doctests (under `src/doctests`) specifically to
//! leverage the fact that doctests support testing for compilation failures.
