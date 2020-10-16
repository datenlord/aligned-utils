//! Aligned wrappers

use core::ops::{Deref, DerefMut};

macro_rules! define_align_newtype {
    {$($(#[$ctor_attr:meta])* $ctor:ident => { $(#[$id_attr:meta])* $id:ident: $align:tt },)+} => {
        $(
            $(#[$ctor_attr])*
            pub const fn $ctor<T>(x: T)->$id<T>{
                $id{ inner: x }
            }

            $(#[$id_attr])*
            #[repr(align($align))]
            #[derive(Debug, Clone, Copy)]
            pub struct $id<T: ?Sized>{
                inner: T
            }

            impl<T: ?Sized> Deref for $id<T> {
                type Target = T;
                fn deref(&self) -> &T {
                    &self.inner
                }
            }

            impl<T: ?Sized> DerefMut for $id<T> {
                fn deref_mut(&mut self) -> &mut T {
                    &mut self.inner
                }
            }

            impl<T> $id<T> {
                /// Consumes the aligned wrapper, returning the wrapped value.
                pub fn into_inner(self) -> T {
                    self.inner
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
                        let a = $ctor([0u8;1]);
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

define_align_newtype! {
    /// Wraps a value with alignment of at least 2 bytes
    align2 => {
        /// A newtype with alignment of at least 2 bytes
        Align2: 2
    },
    /// Wraps a value with alignment of at least 4 bytes
    align4 =>  {
        /// A newtype with alignment of at least 4 bytes
        Align4: 4
    },
    /// Wraps a value with alignment of at least 8 bytes
    align8 =>  {
        /// A newtype with alignment of at least 8 bytes
        Align8: 8
    },
    /// Wraps a value with alignment of at least 16 bytes
    align16 => {
        /// A newtype with alignment of at least 16 bytes
        Align16: 16
    },
    /// Wraps a value with alignment of at least 32 bytes
    align32 => {
        /// A newtype with alignment of at least 32 bytes
        Align32: 32
    },
    /// Wraps a value with alignment of at least 64 bytes
    align64 => {
        /// A newtype with alignment of at least 64 bytes
        Align64: 64
    },
}
