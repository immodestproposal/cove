//! Test expected compilation failures for `Lossless`; success cases are tested in normal 
//! integration tests.
//!
//! ```compile_fail
//! use cove::prelude::*;
//! 
//! let _ = 0u16.cast::<u8>().lossless();
//! ```
//!
//! ```compile_fail
//! use cove::prelude::*;
//!
//! let _ = core::num::NonZeroI128::new(1).unwrap().cast::<i64>().lossless();
//! ```

#[cfg(target_pointer_width = "16")]
/// ```compile_fail
/// use cove::prelude::*;
///
/// let _ = 0usize.cast::<i16>().lossless();
/// ```
///
/// ```compile_fail
/// use cove::prelude::*;
///
/// let _ = 0isize.cast::<u16>().lossless();
/// ```
///
/// ```compile_fail
/// use cove::prelude::*;
///
/// let _ = u32.cast::<usize>().lossless();
/// ```
///
/// ```compile_fail
/// use cove::prelude::*;
///
/// let _ = u16.cast::<isize>().lossless();
/// ```
mod platform_dependent {}

#[cfg(target_pointer_width = "32")]
/// ```compile_fail
/// use cove::prelude::*;
///
/// let _ = 0usize.cast::<i32>().lossless();
/// ```
///
/// ```compile_fail
/// use cove::prelude::*;
///
/// let _ = 0isize.cast::<u32>().lossless();
/// ```
///
/// ```compile_fail
/// use cove::prelude::*;
///
/// let _ = u64.cast::<usize>().lossless();
/// ```
///
/// ```compile_fail
/// use cove::prelude::*;
///
/// let _ = u32.cast::<isize>().lossless();
/// ```
mod platform_dependent {}

#[cfg(target_pointer_width = "64")]
/// ```compile_fail
/// use cove::prelude::*;
/// 
/// let _ = 0usize.cast::<i64>().lossless();
/// ```
///
/// ```compile_fail
/// use cove::prelude::*;
///
/// let _ = 0isize.cast::<u64>().lossless();
/// ```
///
/// ```compile_fail
/// use cove::prelude::*;
///
/// let _ = u128.cast::<usize>().lossless();
/// ```
/// 
/// ```compile_fail
/// use cove::prelude::*;
///
/// let _ = u64.cast::<isize>().lossless();
/// ```
mod platform_dependent {}