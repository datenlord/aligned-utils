use crate::aligned_alloc;

use alloc::alloc::{alloc, dealloc, Layout};
use core::fmt::{self, Debug};
use core::marker::PhantomData;
use core::mem;
use core::ops::{Deref, DerefMut};
use core::ptr::{drop_in_place, NonNull};

/// Aligned heap allocation
pub struct AlignedBox<T: ?Sized> {
    inner: NonNull<T>,
    align: usize,
    _marker: PhantomData<T>,
}

unsafe impl<T: ?Sized + Send> Send for AlignedBox<T> {}
unsafe impl<T: ?Sized + Sync> Sync for AlignedBox<T> {}

#[cfg(feature = "std")]
mod std_impl {
    use super::AlignedBox;

    use std::panic::{RefUnwindSafe, UnwindSafe};

    impl<T: ?Sized + UnwindSafe> UnwindSafe for AlignedBox<T> {}
    impl<T: ?Sized + RefUnwindSafe> RefUnwindSafe for AlignedBox<T> {}
}

impl<T> AlignedBox<T> {
    /// Allocates memory on the heap with specified alignment and then places x into it.
    ///
    /// This doesn't actually allocate if T is zero-sized.
    pub fn new(x: T, align: usize) -> Self {
        let size = mem::size_of::<T>();
        let align = mem::align_of::<T>().max(align);

        let inner = if size == 0 {
            NonNull::dangling()
        } else {
            unsafe {
                let ptr = aligned_alloc(alloc, size, align).cast::<T>();
                ptr.write(x);
                NonNull::new_unchecked(ptr)
            }
        };

        debug_assert!(inner.as_ptr() as usize % align == 0);

        Self {
            align,
            inner,
            _marker: PhantomData,
        }
    }
}

impl<T: ?Sized> AlignedBox<T> {
    /// Consumes the box, returning a raw pointer and alignment.
    pub fn into_raw(this: Self) -> (*mut T, usize) {
        let ans = (this.inner.as_ptr(), this.align);
        mem::forget(this);
        ans
    }

    /// Constructs a aligned box from a raw pointer and alignment
    /// # Safety
    /// + `ptr` must be non-null and well aligned for `align`
    /// + `ptr` must be allocated with the global allocator
    pub unsafe fn from_raw(ptr: *mut T, align: usize) -> Self {
        Self {
            inner: NonNull::new_unchecked(ptr),
            align,
            _marker: PhantomData,
        }
    }

    /// Returns the alignment of the box
    pub fn alignment(this: &Self) -> usize {
        this.align
    }
}

impl<T: ?Sized> Drop for AlignedBox<T> {
    fn drop(&mut self) {
        unsafe {
            let size = mem::size_of_val(self.inner.as_ref());
            if size == 0 {
                return;
            }
            let ptr = self.inner.as_ptr();
            drop_in_place(ptr);

            let layout = Layout::from_size_align_unchecked(size, self.align);
            dealloc(ptr.cast(), layout)
        }
    }
}

impl<T: ?Sized> Deref for AlignedBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.inner.as_ptr() }
    }
}

impl<T: ?Sized> DerefMut for AlignedBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.inner.as_ptr() }
    }
}

impl<T: ?Sized + Debug> Debug for AlignedBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        T::fmt(self, f)
    }
}

#[cfg(feature = "unstable")]
mod unstable_impl {
    use super::AlignedBox;

    use core::marker::Unsize;
    use core::ops::CoerceUnsized;

    impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<AlignedBox<U>> for AlignedBox<T> {}
}

#[cfg(test)]
mod tests {
    use super::AlignedBox;

    #[test]
    fn check_zst() {
        let b = AlignedBox::new((), 1);
        assert_eq!(&*b, &());
        drop(b);
    }

    #[cfg(feature = "unstable")]
    #[test]
    fn check_coerce() {
        let b: AlignedBox<[u8]> = AlignedBox::new([1, 2, 3, 4], 8);
        assert_eq!(&*b, &[1, 2, 3, 4]);
    }
}
