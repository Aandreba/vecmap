#![no_std]
#![cfg_attr(feature = "alloc", feature(allocator_api))]
#![cfg_attr(docsrs, feature(doc_cfg))]

macro_rules! flat_mod {
    ($($i:ident),+) => {
        $(
            mod $i;
            pub use $i::*;
        )+
    }
}

macro_rules! impl_all {
    ($(where $($trait:path),+ =>)? { $($t:tt)* }) => {
        #[cfg(feature = "alloc")]
        impl<K, V, A: Allocator> VecMap<K, V, A> $(where K: $($trait+)*)? {
            $($t)*
        }

        #[cfg(not(feature = "alloc"))]
        impl<K, V> VecMap<K, V> $(where K: $($trait+)*)? {
            $($t)*
        }
    };
}

pub(crate) extern crate alloc;
pub(crate) use alloc::vec::*;

flat_mod! { entry }

use docfg::docfg;
#[cfg(feature = "alloc")]
use core::alloc::*;
use core::{borrow::Borrow};

cfg_if::cfg_if! {
    if #[cfg(feature = "alloc")] {
        pub type IntoIter<K, V, A = Global> = alloc::vec::IntoIter<(K, V), A>;
        pub type Drain<'a, K, V, A = Global> = alloc::vec::Drain<'a, (K, V), A>;
    } else {
        pub type IntoIter<K, V> = alloc::vec::IntoIter<(K, V)>;
        pub type Drain<'a, K, V> = alloc::vec::Drain<'a, (K, V)>;
    }
}

pub struct VecMap<K, V, #[cfg(feature = "alloc")] A: Allocator = Global> {
    #[cfg(feature = "alloc")]
    inner: Vec<(K, V), A>,
    #[cfg(not(feature = "alloc"))]
    inner: Vec<(K, V)>,
}

impl<K, V> VecMap<K, V> {
    #[inline]
    pub const fn new() -> Self {
        return Self { inner: Vec::new() };
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        return Self {
            inner: Vec::with_capacity(capacity),
        };
    }
}

#[docfg(feature = "alloc")]
impl<K, V, A: Allocator> VecMap<K, V, A> {
    #[inline]
    pub const fn new_in(alloc: A) -> Self {
        return Self {
            inner: Vec::new_in(alloc),
        };
    }

    #[inline]
    pub fn with_capacity_in(capacity: usize, alloc: A) -> Self {
        return Self {
            inner: Vec::with_capacity_in(capacity, alloc),
        };
    }
}

impl_all! {{
    #[inline]
    pub fn len (&self) -> usize {
        return self.inner.len();
    }

    #[inline]
    pub fn is_empty (&self) -> bool {
        return self.inner.is_empty();
    }

    #[inline]
    pub fn capacity (&self) -> usize {
        return self.inner.capacity()
    }

    #[inline]
    pub fn clear (&mut self) {
        self.inner.clear()
    }
}}

impl_all! {
    where Eq => {
        #[inline]
        pub fn get<Q: ?Sized + Eq> (&self, key: &Q) -> Option<&V> where K: Borrow<Q> {
            for (k, v) in self.inner.iter() {
                if k.borrow() == key { return Some(v) }
            }
            return None
        }

        #[inline]
        pub fn get_key_value<Q: ?Sized + Eq> (&self, key: &Q) -> Option<(&K, &V)> where K: Borrow<Q> {
            for (k, v) in self.inner.iter() {
                if k.borrow() == key { return Some((k, v)) }
            }
            return None
        }

        #[inline]
        pub fn get_mut<Q: ?Sized + Eq> (&mut self, key: &Q) -> Option<&mut V> where K: Borrow<Q> {
            for (k, v) in self.inner.iter_mut() {
                if <K as Borrow<Q>>::borrow(k) == key { return Some(v) }
            }
            return None
        }
    }
}


impl_all! {
    where Eq => {
        #[inline]
        pub fn insert (&mut self, key: K, value: V) -> Option<V> {
            return match self.entry(key) {
                Entry::Occupied(mut x) => Some(x.insert(value)),
                Entry::Vacant(x) => {
                    x.insert(value);
                    None
                }
            }
        }

        #[inline]
        pub fn remove<Q: ?Sized + Eq> (&mut self, key: &Q) -> Option<V> where K: Borrow<Q> {
            return self.remove_entry(key).map(|x| x.1)
        }

        #[inline]
        pub fn remove_entry<Q: ?Sized + Eq> (&mut self, key: &Q) -> Option<(K, V)> where K: Borrow<Q> {
            for i in 0..self.len() {
                if unsafe { <K as Borrow<Q>>::borrow(&self.inner.get_unchecked(i).0) } == key {
                    return Some(self.inner.swap_remove(i))
                }
            }
            return None
        }

        #[inline]
        pub fn contains_key<Q: ?Sized + Eq> (&self, k: &Q) -> bool where K: Borrow<Q> {
            self.keys().any(|x| x.borrow() == k)
        }
    }
}

impl_all! {{
    #[inline]
    pub fn iter (&self) -> Iter<'_, K, V> {
        return Iter(self.inner.iter())
    }

    #[inline]
    pub fn iter_mut (&mut self) -> IterMut<'_, K, V> {
        return IterMut(self.inner.iter_mut())
    }
    
    #[inline]
    pub fn keys (&self) -> Keys<'_, K, V> {
        return Keys(self.inner.iter())
    }

    #[inline]
    pub fn values (&self) -> Values<'_, K, V> {
        return Values(self.inner.iter())
    }

    #[inline]
    pub fn values_mut (&mut self) -> ValuesMut<'_, K, V> {
        return ValuesMut(self.inner.iter_mut())
    }
}}

#[cfg(feature = "alloc")]
impl<K, V, A: Allocator> VecMap<K, V, A> {
    #[inline]
    pub fn into_vec (self) -> Vec<(K, V), A> {
        return self.inner
    }

    #[inline]
    pub fn entry<'a>(&'a mut self, key: K) -> Entry<'a, K, V, A> where K: Eq {
        for i in 0..self.inner.len() {
            if unsafe { &self.inner.get_unchecked(i).0 } == &key {
                return Entry::Occupied(OccupiedEntry {
                    parent: &mut self.inner,
                    idx: i,
                });
            }
        }
        return Entry::Vacant(VacantEntry {
            parent: &mut self.inner,
            key,
        });
    }

    #[inline]
    pub fn into_keys (self) -> IntoKeys<K, V, A> {
        return IntoKeys(self.inner.into_iter())
    }

    #[inline]
    pub fn into_values (self) -> IntoValues<K, V, A> {
        return IntoValues(self.inner.into_iter())
    }

    #[inline]
    pub fn drain (&mut self) -> Drain<'_, K, V, A> {
        return self.inner.drain(..)
    }
}

#[cfg(not(feature = "alloc"))]
impl<K, V> VecMap<K, V> {
    #[inline]
    pub fn into_vec (self) -> Vec<(K, V)> {
        return self.inner
    }

    #[inline]
    pub fn entry<'a>(&'a mut self, key: K) -> Entry<'a, K, V> where K: Eq {
        for i in 0..self.inner.len() {
            if unsafe { &self.inner.get_unchecked(i).0 } == &key {
                return Entry::Occupied(OccupiedEntry {
                    parent: &mut self.inner,
                    idx: i,
                });
            }
        }
        return Entry::Vacant(VacantEntry {
            parent: &mut self.inner,
            key,
        });
    }

    #[inline]
    pub fn into_keys (self) -> IntoKeys<K, V> {
        return IntoKeys(self.inner.into_iter())
    }

    #[inline]
    pub fn into_values (self) -> IntoValues<K, V> {
        return IntoValues(self.inner.into_iter())
    }

    #[inline]
    pub fn drain (&mut self) -> Drain<'_, K, V> {
        return self.inner.drain(..)
    }
}

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct Keys<'a, K: 'a, V: 'a> (core::slice::Iter<'a, (K, V)>);

impl<'a, K, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (key, _) = self.0.next()?;
        return Some(key)
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (key, _) = self.0.nth(n)?;
        return Some(key)
    }

    #[inline]
    fn count(self) -> usize where Self: Sized, {
        self.0.count()
    }

    #[inline]
    fn last(self) -> Option<Self::Item> where Self: Sized, {
        let (key, _) = self.0.last()?;
        return Some(key)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a, K, V> DoubleEndedIterator for Keys<'a, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let (key, _) = self.0.next_back()?;
        return Some(key)
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let (key, _) = self.0.nth_back(n)?;
        return Some(key)
    }
}

impl<'a, K, V> ExactSizeIterator for Keys<'a, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct Values<'a, K: 'a, V: 'a> (core::slice::Iter<'a, (K, V)>);

impl<'a, K, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (_, values) = self.0.next()?;
        return Some(values)
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (_, values) = self.0.nth(n)?;
        return Some(values)
    }

    #[inline]
    fn count(self) -> usize where Self: Sized, {
        self.0.count()
    }

    #[inline]
    fn last(self) -> Option<Self::Item> where Self: Sized, {
        let (_, values) = self.0.last()?;
        return Some(values)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a, K, V> DoubleEndedIterator for Values<'a, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let (_, values) = self.0.next_back()?;
        return Some(values)
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let (_, values) = self.0.nth_back(n)?;
        return Some(values)
    }
}

impl<'a, K, V> ExactSizeIterator for Values<'a, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct ValuesMut<'a, K: 'a, V: 'a> (core::slice::IterMut<'a, (K, V)>);

impl<'a, K, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (_, values) = self.0.next()?;
        return Some(values)
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (_, values) = self.0.nth(n)?;
        return Some(values)
    }

    #[inline]
    fn count(self) -> usize where Self: Sized, {
        self.0.count()
    }

    #[inline]
    fn last(self) -> Option<Self::Item> where Self: Sized, {
        let (_, values) = self.0.last()?;
        return Some(values)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a, K, V> DoubleEndedIterator for ValuesMut<'a, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let (_, values) = self.0.next_back()?;
        return Some(values)
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let (_, values) = self.0.nth_back(n)?;
        return Some(values)
    }
}

impl<'a, K, V> ExactSizeIterator for ValuesMut<'a, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct IntoKeys<K, V, #[cfg(feature = "alloc")] A: Allocator = Global> (
    #[cfg(feature = "alloc")]
    alloc::vec::IntoIter<(K, V), A>,
    #[cfg(not(feature = "alloc"))]
    alloc::vec::IntoIter<(K, V)>,
);

impl<K, V> Iterator for IntoKeys<K, V> {
    type Item = K;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (key, _) = self.0.next()?;
        return Some(key)
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (key, _) = self.0.nth(n)?;
        return Some(key)
    }

    #[inline]
    fn count(self) -> usize where Self: Sized, {
        self.0.count()
    }

    #[inline]
    fn last(self) -> Option<Self::Item> where Self: Sized, {
        let (key, _) = self.0.last()?;
        return Some(key)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<K, V> DoubleEndedIterator for IntoKeys<K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let (key, _) = self.0.next_back()?;
        return Some(key)
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let (key, _) = self.0.nth_back(n)?;
        return Some(key)
    }
}

impl<K, V> ExactSizeIterator for IntoKeys<K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct IntoValues<K, V, #[cfg(feature = "alloc")] A: Allocator = Global> (
    #[cfg(feature = "alloc")]
    alloc::vec::IntoIter<(K, V), A>,
    #[cfg(not(feature = "alloc"))]
    alloc::vec::IntoIter<(K, V)>,
);

impl<K, V> Iterator for IntoValues<K, V> {
    type Item = V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (_, values) = self.0.next()?;
        return Some(values)
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (_, values) = self.0.nth(n)?;
        return Some(values)
    }

    #[inline]
    fn count(self) -> usize where Self: Sized, {
        self.0.count()
    }

    #[inline]
    fn last(self) -> Option<Self::Item> where Self: Sized, {
        let (_, values) = self.0.last()?;
        return Some(values)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<K, V> DoubleEndedIterator for IntoValues<K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let (_, values) = self.0.next_back()?;
        return Some(values)
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let (_, values) = self.0.nth_back(n)?;
        return Some(values)
    }
}

impl<K, V> ExactSizeIterator for IntoValues<K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct Iter<'a, K: 'a, V: 'a> (core::slice::Iter<'a, (K, V)>);

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (key, value) = self.0.next()?;
        return Some((key, value))
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (key, value) = self.0.nth(n)?;
        return Some((key, value))
    }

    #[inline]
    fn count(self) -> usize where Self: Sized, {
        self.0.count()
    }

    #[inline]
    fn last(self) -> Option<Self::Item> where Self: Sized, {
        let (key, value) = self.0.last()?;
        return Some((key, value))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a, K, V> DoubleEndedIterator for Iter<'a, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let (key, value) = self.0.next_back()?;
        return Some((key, value))
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let (key, value) = self.0.nth_back(n)?;
        return Some((key, value))
    }
}

impl<'a, K, V> ExactSizeIterator for Iter<'a, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct IterMut<'a, K: 'a, V: 'a> (core::slice::IterMut<'a, (K, V)>);

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (key, value) = self.0.next()?;
        return Some((key, value))
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (key, value) = self.0.nth(n)?;
        return Some((key, value))
    }

    #[inline]
    fn count(self) -> usize where Self: Sized, {
        self.0.count()
    }

    #[inline]
    fn last(self) -> Option<Self::Item> where Self: Sized, {
        let (key, value) = self.0.last()?;
        return Some((key, value))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a, K, V> DoubleEndedIterator for IterMut<'a, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let (key, value) = self.0.next_back()?;
        return Some((key, value))
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let (key, value) = self.0.nth_back(n)?;
        return Some((key, value))
    }
}

impl<'a, K, V> ExactSizeIterator for IterMut<'a, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}