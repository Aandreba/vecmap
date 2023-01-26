#[cfg(feature = "alloc")]
use core::alloc::*;
#[cfg(feature = "alloc")]
use alloc::alloc::*;
use core::{
    mem::{MaybeUninit},
};
use alloc::vec::*;

cfg_if::cfg_if! {
    if #[cfg(feature = "alloc")] {
        pub enum Entry<'a, K: 'a, V: 'a, A: Allocator = Global> {
            Occupied(OccupiedEntry<'a, K, V, A>),
            Vacant(VacantEntry<'a, K, V, A>),
        }
    } else {
        pub enum Entry<'a, K: 'a, V: 'a> {
            Occupied(OccupiedEntry<'a, K, V>),
            Vacant(VacantEntry<'a, K, V>),
        }
    }
}

macro_rules! impl_all {
    ($name:ident => { $($t:tt)* }) => {
        #[cfg(feature = "alloc")]
        impl<'a, K, V, A: Allocator> $name<'a, K, V, A> {
            $($t)*
        }

        #[cfg(not(feature = "alloc"))]
        impl<'a, K, V> $name<'a, K, V> {
            $($t)*
        }
    };
}

#[derive(Debug)]
pub struct OccupiedEntry<'a, K: 'a, V: 'a, #[cfg(feature = "alloc")] A: Allocator = Global> {
    #[cfg(feature = "alloc")]
    pub(super) parent: &'a mut Vec<(K, V), A>,
    #[cfg(not(feature = "alloc"))]
    pub(super) parent: &'a mut Vec<(K, V)>,
    pub(super) idx: usize,
}

impl_all! {
    OccupiedEntry => {
        #[inline]
        pub fn get(&self) -> &V {
            unsafe { &self.parent.get_unchecked(self.idx).1 }
        }

        #[inline]
        pub fn get_mut(&mut self) -> &mut V {
            unsafe { &mut self.parent.get_unchecked_mut(self.idx).1 }
        }

        #[inline]
        pub fn into_mut(self) -> &'a mut V {
            unsafe { &mut self.parent.get_unchecked_mut(self.idx).1 }
        }

        #[inline]
        pub fn insert(&mut self, value: V) -> V {
            core::mem::replace(self.get_mut(), value)
        }

        #[inline]
        pub fn remove(self) -> V {
            self.parent.swap_remove(self.idx).1
        }
    }
}

#[derive(Debug)]
pub struct VacantEntry<'a, K: 'a, V: 'a, #[cfg(feature = "alloc")] A: Allocator = Global> {
    #[cfg(feature = "alloc")]
    pub(super) parent: &'a mut Vec<(K, V), A>,
    #[cfg(not(feature = "alloc"))]
    pub(super) parent: &'a mut Vec<(K, V)>,
    pub(super) key: K,
}

impl_all! {
    VacantEntry => {
        #[inline]
        pub fn insert(self, value: V) -> &'a mut V {
            unsafe {
                self.parent.reserve(1);
                let entry = &mut *self.parent
                    .as_mut_ptr()
                    .add(self.parent.len())
                    .cast::<MaybeUninit<(K, V)>>();
                
                self.parent.set_len(self.parent.len() + 1);
                return &mut entry.write((self.key, value)).1
            }
        }
        
        #[inline]
        pub fn key(&self) -> &K {
            return &self.key;
        }
        
        #[inline]
        pub fn into_key(self) -> K {
            return self.key;
        }
    }
}