//! Common utilities to work with aligned values and allocation.
//!
//! # Example
//!
//! ```
//! use aligned_utils::stack::Align8;
//! let mut arr = Align8([1, 2, 3]);
//! let bytes: &[u8] = &*arr;
//! ```
//!
//! ```
//! # #[cfg(feature="alloc")]
//! # {
//! use aligned_utils::bytes::AlignedBytes; // with feature "alloc"
//! let mut bytes = AlignedBytes::new_zeroed(1024, 8);
//! let buf: &mut [u8] = &mut *bytes;
//! # }
//! ```
//!

#![deny(
    anonymous_parameters,
    bare_trait_objects,
    elided_lifetimes_in_paths,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![allow(box_pointers, clippy::module_name_repetitions)]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod stack;

#[cfg(feature = "alloc")]
pub mod bytes;
