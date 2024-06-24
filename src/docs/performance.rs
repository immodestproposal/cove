//! Provides insight on the runtime overhead of cove casts
//! 
//! # Performance
//! Cove's primary mission is to improve the casting situation by replacing as many use cases for
//! the `as` keyword as possible. Since one of the reasons to use `as` is performance, cove strives
//! to provide implementations which can compete on runtime speed, so that there is no need for the 
//! programmer to choose between safer, self-documenting casts and speedy ones.
//!
//! Several of the casts provided in this crate can be expected to optimize to the same
//! assembly as the `as` keyword in release builds. For example, consider this function:
//!
//! ```
//! #[inline(never)]
//! fn cast_u32_to_u8(value: u32) {
//!     // core::hint::black_box(value as u8);
//!     // core::hint::black_box(value.cast::<u8>().lossy());
//!     // core::hint::black_box(value.cast::<u8>().assumed_lossless());
//! }
//! ```
//!
//! Commenting in each of these lines in turn and compiling the function in release with Rust
//! 1.72.0 on stable-x86_64-pc-windows-msvc yields the exact same assembly for all three:
//!
//! ```ignore
//! push rax
//! mov byte ptr [rsp + 7], cl
//! lea rax, [rsp + 7]
//! pop rax
//! ret
//! ```
//!
//! Optimizer results are subject to variation by version and platform and can never be completely
//! relied upon, but the core point remains: there is no need to a priori favor `as` over cove's
//! casts strictly for performance.
//!
//! Consult the documentation on each casting trait for performance notes. Also refer to `asm.rs`
//! in cove's `examples` directory for assistance with testing assembly generation for your 
//! platform.