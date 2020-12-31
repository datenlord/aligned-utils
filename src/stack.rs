//! Aligned wrappers

use core::ops::{Deref, DerefMut};

macro_rules! define_align_newtype {
    ($($(#[$id_attr:meta])* $id:ident: $align:tt,)+) => {
        $(
            $(#[$id_attr])*
            #[repr(align($align))]
            #[derive(Debug, Clone, Copy)]
            pub struct $id<T: ?Sized>(pub T);


            impl<T: ?Sized> Deref for $id<T> {
                type Target = T;
                fn deref(&self) -> &T {
                    &self.0
                }
            }

            impl<T: ?Sized> DerefMut for $id<T> {
                fn deref_mut(&mut self) -> &mut T {
                    &mut self.0
                }
            }

            impl<T> $id<T> {
                /// Consumes the aligned wrapper, returning the wrapped value.
                pub fn into_inner(this: Self) -> T {
                    this.0
                }
            }
        )+

        #[cfg(test)]
        mod tests{
            use super::*;

            #[test]
            fn check_aligned_wrappers(){
                $(
                    {
                        let a = $id([0u8;1]);
                        assert_eq!(core::mem::align_of_val(&a), $align);
                        assert_eq!(a.as_ptr() as usize % $align, 0);
                        assert_eq!(a.as_ref(), &[0u8]);

                        let b = Box::new(a);
                        assert_eq!(&*b as *const $id<[u8;1]> as usize % $align, 0);

                        let c: &$id<[u8]> = &a;
                        assert_eq!(c.as_ref(), &[0u8]);
                    }
                )+
            }
        }
    };
}

define_align_newtype! (
    /// A newtype with alignment of at least 2 bytes
    Align2: 2,
    /// A newtype with alignment of at least 4 bytes
    Align4: 4,
    /// A newtype with alignment of at least 8 bytes
    Align8: 8,
    /// A newtype with alignment of at least 16 bytes
    Align16: 16,
    /// A newtype with alignment of at least 32 bytes
    Align32: 32,
    /// A newtype with alignment of at least 64 bytes
    Align64: 64,
);
