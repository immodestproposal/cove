//! Test expected compilation failures for `Bitwise`; success cases are tested in normal
//! integration tests.
//!
//! ```compile_fail
//! use cove::prelude::*;
//!
//! let _ = 0u16.cast::<u8>().bitwise();
//! ```
//!
//! ```compile_fail
//! use cove::prelude::*;
//!
//! let _ = core::num::NonZeroI128::new(1).unwrap().cast::<i64>().bitwise();
//! ```
//!
//! ```compile_fail
//! use cove::prelude::*;
//!
//! let _ = 0i32.cast::<core::num::NonZeroI32>().bitwise();
//! ```

#[cfg(target_pointer_width = "16")]
/// ```compile_fail
/// use cove::prelude::*;
///
/// let _ = 0usize.cast::<i32>().bitwise();
/// ```
///
/// ```compile_fail
/// use cove::prelude::*;
///
/// let _ = 0isize.cast::<f64>().bitwise();
/// ```
///
/// ```compile_fail
/// use cove::prelude::*;
///
/// let _ = u64.cast::<usize>().bitwise();
/// ```
///
/// ```compile_fail
/// use cove::prelude::*;
///
/// let _ = core::num::NonZeroI64::new(1).unwrap().cast::<isize>().bitwise();
/// ```
mod platform_dependent {}

#[cfg(target_pointer_width = "32")]
/// ```compile_fail
/// use cove::prelude::*;
///
/// let _ = 0usize.cast::<i64>().bitwise();
/// ```
///
/// ```compile_fail
/// use cove::prelude::*;
///
/// let _ = 0isize.cast::<f64>().bitwise();
/// ```
///
/// ```compile_fail
/// use cove::prelude::*;
///
/// let _ = u64.cast::<usize>().bitwise();
/// ```
///
/// ```compile_fail
/// use cove::prelude::*;
///
/// let _ = core::num::NonZeroI64::new(1).unwrap().cast::<isize>().bitwise();
/// ```
mod platform_dependent {}

#[cfg(target_pointer_width = "64")]
/// ```compile_fail
/// use cove::prelude::*;
///
/// let _ = 0usize.cast::<i16>().bitwise();
/// ```
///
/// ```compile_fail
/// use cove::prelude::*;
///
/// let _ = 0isize.cast::<f32>().bitwise();
/// ```
///
/// ```compile_fail
/// use cove::prelude::*;
///
/// let _ = u32.cast::<usize>().bitwise();
/// ```
///
/// ```compile_fail
/// use cove::prelude::*;
///
/// let _ = core::num::NonZeroI32::new(1).unwrap().cast::<isize>().bitwise();
/// ```
mod platform_dependent {}

#[cfg(target_pointer_width = "128")]
/// ```compile_fail
/// use cove::prelude::*;
///
/// let _ = 0usize.cast::<i32>().bitwise();
/// ```
///
/// ```compile_fail
/// use cove::prelude::*;
///
/// let _ = 0isize.cast::<f64>().bitwise();
/// ```
///
/// ```compile_fail
/// use cove::prelude::*;
///
/// let _ = u64.cast::<usize>().bitwise();
/// ```
///
/// ```compile_fail
/// use cove::prelude::*;
///
/// let _ = core::num::NonZeroI64::new(1).unwrap().cast::<isize>().bitwise();
/// ```
mod platform_dependent {}