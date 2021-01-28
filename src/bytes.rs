//! A continuous fixed-length byte array with a specified alignment.
//!
//! # Example
//!
//! ```
//! use aligned_utils::bytes::AlignedBytes;
//! let mut bytes = AlignedBytes::new_zeroed(1024, 8);
//! let buf: &mut [u8] = &mut *bytes;
//! ```
//!

use alloc::alloc::{alloc, alloc_zeroed, dealloc, handle_alloc_error, Layout};
use core::fmt;
use core::mem;
use core::ops::{Deref, DerefMut};
use core::ptr::{self, NonNull};
use core::slice;

fn layout(size: usize, align: usize) -> Layout {
    match Layout::from_size_align(size, align) {
        Ok(layout) => layout,
        Err(_) => panic!("Invalid layout: size = {}, align = {}", size, align),
    }
}

unsafe fn aligned_alloc(alloc: unsafe fn(Layout) -> *mut u8, layout: Layout) -> *mut u8 {
    let ptr = alloc(layout);
    if ptr.is_null() {
        handle_alloc_error(layout);
    }
    debug_assert!(
        (ptr as usize) % layout.align() == 0,
        "pointer = {:p} is not a multiple of alignment = {}",
        ptr,
        layout.align()
    );
    ptr
}

/// A continuous fixed-length byte array with a specified alignment.
pub struct AlignedBytes {
    buf: NonNull<[u8]>,
    align: usize,
}

unsafe impl Send for AlignedBytes {}
unsafe impl Sync for AlignedBytes {}

#[cfg(feature = "std")]
mod std_impl {
    use super::AlignedBytes;

    use std::panic::{RefUnwindSafe, UnwindSafe};

    impl RefUnwindSafe for AlignedBytes {}
    impl UnwindSafe for AlignedBytes {}
}

impl Drop for AlignedBytes {
    fn drop(&mut self) {
        unsafe {
            let size = self.buf.as_mut().len();
            if size != 0 {
                let ptr = self.buf.as_ptr() as *mut u8;
                let layout = Layout::from_size_align_unchecked(size, self.align);
                dealloc(ptr, layout);
            }
        }
    }
}

impl AlignedBytes {
    /// Allocates a zero-initialized byte array with an exact alignment.
    #[must_use]
    pub fn new_zeroed(len: usize, align: usize) -> Self {
        let layout = layout(len, align);
        unsafe {
            let ptr = if len == 0 {
                align as *mut u8
            } else {
                aligned_alloc(alloc_zeroed, layout)
            };
            let buf = NonNull::from(slice::from_raw_parts_mut(ptr, len));
            Self { buf, align }
        }
    }

    /// Allocates a byte array with an exact alignment and copy `bytes` into it.
    #[must_use]
    pub fn new_from_slice(bytes: &[u8], align: usize) -> Self {
        let len = bytes.len();

        let layout = layout(len, align);
        unsafe {
            let ptr = if len == 0 {
                align as *mut u8
            } else {
                let dst = aligned_alloc(alloc, layout);
                ptr::copy_nonoverlapping(bytes.as_ptr(), dst, len);
                dst
            };
            let buf = NonNull::from(slice::from_raw_parts_mut(ptr, len));
            Self { buf, align }
        }
    }

    /// Returns the alignment of the byte array.    
    #[must_use]
    pub const fn align(&self) -> usize {
        self.align
    }

    /// Consumes the byte array, returning a pointer and the alignment.
    ///
    /// To release the memory, call [`AlignedBytes::from_raw`].
    ///
    #[must_use]
    pub fn into_raw(this: Self) -> (NonNull<[u8]>, usize) {
        let ret = (this.buf, this.align);
        mem::forget(this);
        ret
    }

    /// Constructs an [`AlignedBytes`] from a pointer and the alignment.
    ///
    /// # Safety
    /// This function is unsafe because improper use may lead to memory problems.
    /// For example, a double-free may occur if the function is called twice on the same raw pointer.
    ///
    #[must_use]
    pub const unsafe fn from_raw(buf: NonNull<[u8]>, align: usize) -> Self {
        Self { buf, align }
    }
}

impl Clone for AlignedBytes {
    fn clone(&self) -> Self {
        Self::new_from_slice(self, self.align)
    }
}

impl Deref for AlignedBytes {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        unsafe { self.buf.as_ref() }
    }
}

impl DerefMut for AlignedBytes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.buf.as_mut() }
    }
}

impl AsRef<[u8]> for AlignedBytes {
    fn as_ref(&self) -> &[u8] {
        self
    }
}

impl AsMut<[u8]> for AlignedBytes {
    fn as_mut(&mut self) -> &mut [u8] {
        self
    }
}

impl fmt::Debug for AlignedBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <[u8] as fmt::Debug>::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::AlignedBytes;

    #[test]
    fn check_content() {
        {
            let bytes = AlignedBytes::new_zeroed(8, 8);
            assert_eq!(&*bytes, &[0, 0, 0, 0, 0, 0, 0, 0,]);
        }
        {
            let bytes = &[1, 2, 3, 4, 5, 6, 7, 8];
            let aligned_bytes = AlignedBytes::new_from_slice(bytes, 8);
            assert_eq!(&*aligned_bytes, bytes);

            let aligned_bytes_cloned = aligned_bytes.clone();
            drop(aligned_bytes);
            assert_eq!(&*aligned_bytes_cloned, bytes);
        }
    }

    #[test]
    fn check_alignment() {
        let align = 4096;
        let bytes = AlignedBytes::new_zeroed(8, align);
        assert_eq!(bytes.align(), align);
        assert!(bytes.as_ptr() as usize % align == 0);
    }

    #[should_panic(expected = "Invalid layout: size = 1, align = 0")]
    #[test]
    fn check_layout_zero_align() {
        let bytes = AlignedBytes::new_zeroed(1, 0);
        drop(bytes);
    }

    #[should_panic(expected = "Invalid layout: size = 0, align = 0")]
    #[test]
    fn check_layout_zero_len_align() {
        let bytes = AlignedBytes::new_zeroed(0, 0);
        drop(bytes);
    }

    #[should_panic(expected = "Invalid layout: size = 1, align = 3")]
    #[test]
    fn check_layout_align_not_power_of_2() {
        let bytes = AlignedBytes::new_zeroed(1, 3);
        drop(bytes);
    }

    #[should_panic]
    #[test]
    fn check_layout_overflow() {
        let size = core::mem::size_of::<usize>() * 8;
        let bytes = AlignedBytes::new_zeroed((1_usize << (size - 1)) + 1, 1_usize << (size - 1));
        drop(bytes);
    }

    macro_rules! require {
        ($ty:ty: $($markers:tt)+) => {{
            fn __require<T: $($markers)*>() {}
            __require::<$ty>();
        }};
    }

    #[test]
    fn check_markers() {
        require!(AlignedBytes: Send + Sync);

        #[cfg(feature = "std")]
        {
            use std::panic::{RefUnwindSafe, UnwindSafe};
            require!(AlignedBytes: RefUnwindSafe + UnwindSafe);
        }
    }

    #[test]
    fn check_zst() {
        let bytes = AlignedBytes::new_zeroed(0, 2);
        drop(bytes);
    }

    #[test]
    fn check_into_raw() {
        let bytes = AlignedBytes::new_zeroed(0, 2);
        let (buf, align) = AlignedBytes::into_raw(bytes);
        drop(unsafe { AlignedBytes::from_raw(buf, align) });
    }
}
