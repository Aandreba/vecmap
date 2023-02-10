#[cfg(feature = "alloc")]
use core::alloc::*;
#[cfg(feature = "alloc")]
use alloc::alloc::*;
use alloc::vec::Vec;

#[cfg(feature = "alloc")]
#[derive(Debug)]
pub enum Entry<'a, K, V, A: Allocator = Global> {
    Occupied (OcuppiedEntry<'a, K, V, A>),
    Vacant (VacantEntry<'a, K, V, A>)
}

#[cfg(not(feature = "alloc"))]
#[derive(Debug)]
pub enum Entry<'a, K, V> {
    Occupied (OcuppiedEntry<'a, K, V>),
    Vacant (VacantEntry<'a, K, V>)
}

#[derive(Debug)]
pub struct OcuppiedEntry<'a, K, V, #[cfg(feature = "alloc")] A: Allocator = Global> {
    #[cfg(feature = "alloc")]
    pub(super) parent: &'a mut Vec<(K, V), A>,
    #[cfg(not(feature = "alloc"))]
    pub(super) parent: &'a mut Vec<(K, V)>,
    pub(super) idx: usize
}

#[derive(Debug)]
pub struct VacantEntry<'a, K, V, #[cfg(feature = "alloc")] A: Allocator = Global> {
    #[cfg(feature = "alloc")]
    pub(super) parent: &'a mut Vec<(K, V), A>,
    #[cfg(not(feature = "alloc"))]
    pub(super) parent: &'a mut Vec<(K, V)>,
    pub(super) idx: usize,
    pub(super) key: K,
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

impl_all! {
    OcuppiedEntry => {
        #[inline]
        pub fn key (&self) -> &K {
            unsafe { &self.parent.get_unchecked(self.idx).0 }
        }
        
        #[inline]
        pub fn remove_entry (self) -> (K, V) {
            self.parent.remove(self.idx)
        }
        
        #[inline]
        pub fn get (&self) -> &V {
            unsafe { &self.parent.get_unchecked(self.idx).1 }
        }
        
        #[inline]
        pub fn get_mut (&mut self) -> &mut V {
            unsafe { &mut self.parent.get_unchecked_mut(self.idx).1 }
        }
        
        #[inline]
        pub fn into_mut (self) -> &'a mut V {
            unsafe { &mut self.parent.get_unchecked_mut(self.idx).1 }
        }
        
        #[inline]
        pub fn insert (&mut self, value: V) -> V {
            unsafe { core::mem::replace(&mut self.parent.get_unchecked_mut(self.idx).1, value) }
        }
        
        #[inline]
        pub fn remove (self) -> V {
            let (_, v) = self.remove_entry();
            return v
        }
    }
}

impl_all! {
    VacantEntry => {
        #[inline]
        pub fn key (&self) -> &K {
            &self.key
        }
        
        #[inline]
        pub fn into_key (self) -> K {
            self.key
        }
        
        #[inline]
        pub fn insert (self, value: V) -> &'a mut V {
            self.parent.insert(self.idx, (self.key, value));
            return unsafe { &mut self.parent.get_unchecked_mut(self.idx).1 }
        }
    }
}