use crate::aligned_alloc;
use crate::boxed::AlignedBox;

use alloc::alloc::alloc_zeroed;
use core::fmt::{self, Debug};
use core::ops::{Deref, DerefMut};
use core::slice;

/// A continuous fixed-length byte array with a specified alignment.
pub struct AlignedBytes {
    inner: AlignedBox<[u8]>,
}

impl AlignedBytes {
    /// Allocate a zero-initialized byte array with an exact alignment.
    pub fn new_zeroed(len: usize, align: usize) -> Self {
        let inner = unsafe {
            let ptr = if len == 0 {
                align as *mut u8
            } else {
                aligned_alloc(alloc_zeroed, len, align)
            };
            AlignedBox::from_raw(slice::from_raw_parts_mut(ptr, len), align)
        };
        debug_assert!(inner.as_ptr() as usize % align == 0);
        Self { inner }
    }

    /// Returns the alignment of the byte array.
    pub fn alignment(&self) -> usize {
        AlignedBox::alignment(&self.inner)
    }
}

impl Debug for AlignedBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <[u8] as Debug>::fmt(&**self, f)
    }
}

impl Deref for AlignedBytes {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl DerefMut for AlignedBytes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.inner
    }
}

impl AsRef<[u8]> for AlignedBytes {
    fn as_ref(&self) -> &[u8] {
        &*self
    }
}

impl AsMut<[u8]> for AlignedBytes {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut *self
    }
}

#[cfg(test)]
mod tests {
    use super::AlignedBytes;

    #[test]
    fn check_alignment() {
        let align = 4096;
        let bytes = AlignedBytes::new_zeroed(8, align);
        assert_eq!(bytes.alignment(), align);
        assert!(bytes.as_ptr() as usize % align == 0);
    }

    #[should_panic(expected = "Invalid layout: size = 1, align = 0")]
    #[test]
    fn check_layout_zero_align() {
        AlignedBytes::new_zeroed(1, 0);
    }

    #[should_panic(expected = "Invalid layout: size = 1, align = 3")]
    #[test]
    fn check_layout_align_not_power_of_2() {
        AlignedBytes::new_zeroed(1, 3);
    }

    #[should_panic]
    #[test]
    fn check_layout_overflow() {
        let size = core::mem::size_of::<usize>() * 8;
        AlignedBytes::new_zeroed((1usize << (size - 1)) + 1, 1usize << (size - 1));
    }

    #[test]
    fn check_markers() {
        fn require_send<T: Send>() {}
        fn require_sync<T: Sync>() {}

        require_send::<AlignedBytes>();
        require_sync::<AlignedBytes>();

        #[cfg(feature = "std")]
        {
            use std::panic::{RefUnwindSafe, UnwindSafe};
            fn require_unwind_safe<T: UnwindSafe>() {}
            fn require_ref_unwind_safe<T: RefUnwindSafe>() {}
            require_unwind_safe::<AlignedBytes>();
            require_ref_unwind_safe::<AlignedBytes>();
        }
    }

    #[test]
    fn check_zst() {
        AlignedBytes::new_zeroed(0, 2);
    }
}
