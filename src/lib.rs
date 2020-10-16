#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "unstable", feature(unsize, coerce_unsized))]

//! A continuous fixed-length byte array with a specified alignment.
//!
//! # Example
//! ```
//! use aligned_bytes::AlignedBytes;
//! let mut bytes = AlignedBytes::new_zeroed(1024, 8);
//! let buf: &mut [u8] = &mut *bytes;
//! ```

extern crate alloc;

mod boxed;
mod bytes;
pub mod stack;

pub use self::boxed::AlignedBox;
pub use self::bytes::AlignedBytes;

use alloc::alloc::{handle_alloc_error, Layout};

unsafe fn aligned_alloc(alloc: unsafe fn(Layout) -> *mut u8, size: usize, align: usize) -> *mut u8 {
    let layout = match Layout::from_size_align(size, align) {
        Ok(layout) => layout,
        Err(_) => panic!("Invalid layout: size = {}, align = {}", size, align),
    };

    let ptr = alloc(layout);
    if ptr.is_null() {
        handle_alloc_error(layout);
    }
    debug_assert!(
        (ptr as usize) % align == 0,
        "pointer = {:p} is not a multiple of alignment = {}",
        ptr,
        align
    );
    ptr
}
