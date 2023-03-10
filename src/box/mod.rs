macro_rules! impl_all {
    ($(where $($trait:path),+ =>)? { $($t:tt)* }) => {
        #[cfg(feature = "alloc")]
        impl<K, V, A: Allocator> BoxMap<K, V, A> $(where K: $($trait+)*)? {
            $($t)*
        }

        #[cfg(not(feature = "alloc"))]
        impl<K, V> BoxMap<K, V> $(where K: $($trait+)*)? {
            $($t)*
        }
    };
}

use crate::vec::VecMap;
#[cfg(feature = "alloc")]
use alloc::alloc::*;
use alloc::{boxed::*, vec::Vec};
use core::fmt::Debug;
use core::{
    borrow::Borrow,
    ops::{Index, IndexMut},
};

cfg_if::cfg_if! {
    if #[cfg(feature = "alloc")] {
        pub type IntoIter<K, V, A = Global> = alloc::vec::IntoIter<(K, V), A>;
        pub type Drain<'a, K, V, A = Global> = alloc::vec::Drain<'a, (K, V), A>;
    } else {
        pub type IntoIter<K, V> = alloc::vec::IntoIter<(K, V)>;
        pub type Drain<'a, K, V> = alloc::vec::Drain<'a, (K, V)>;
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BoxMap<K, V, #[cfg(feature = "alloc")] A: Allocator = Global> {
    #[cfg(feature = "alloc")]
    inner: Box<[(K, V)], A>,
    #[cfg(not(feature = "alloc"))]
    inner: Box<[(K, V)]>,
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
}}

impl_all! {
    {
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
impl<K, V, A: Allocator> BoxMap<K, V, A> {
    #[inline]
    pub unsafe fn from_vec_unchecked (vec: Vec<(K, V), A>) -> Self {
        return Self::from_box_unchecked(vec.into_boxed_slice())
    }

    #[inline]
    pub unsafe fn from_box_unchecked (bx: Box<[(K, V)], A>) -> Self {
        return Self { inner: bx }
    }

    #[inline]
    pub fn into_vec(self) -> Vec<(K, V), A> {
        return self.inner.into_vec();
    }

    #[inline]
    pub fn into_box(self) -> Box<[(K, V)], A> {
        return self.inner;
    }

    #[inline]
    pub fn into_keys(self) -> IntoKeys<K, V, A> {
        return IntoKeys(self.into_vec().into_iter());
    }

    #[inline]
    pub fn into_values(self) -> IntoValues<K, V, A> {
        return IntoValues(self.into_vec().into_iter());
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "alloc")] {
        impl<K, V, A: Allocator> IntoIterator for BoxMap<K, V, A> {
            type Item = (K, V);
            type IntoIter = IntoIter<K, V, A>;

            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                self.into_vec().into_iter()
            }
        }

        impl<'a, K, V, A: Allocator> IntoIterator for &'a BoxMap<K, V, A> {
            type Item = (&'a K, &'a V);
            type IntoIter = Iter<'a, K, V>;

            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                BoxMap::iter(self)
            }
        }

        impl<'a, K, V, A: Allocator> IntoIterator for &'a mut BoxMap<K, V, A> {
            type Item = (&'a K, &'a mut V);
            type IntoIter = IterMut<'a, K, V>;

            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                BoxMap::iter_mut(self)
            }
        }

        impl<K: Eq, V, A: Allocator + Default> FromIterator<(K, V)> for BoxMap<K, V, A> {
            #[inline]
            fn from_iter<T: IntoIterator<Item = (K, V)>> (iter: T) -> Self {
                VecMap::from_iter(iter).into()
            }
        }

        impl<K, V, A: Allocator + Default> Default for BoxMap<K, V, A> {
            #[inline]
            fn default () -> Self {
                return Self { inner: Box::new_in([], A::default()) }
            }
        }

        impl<K: Debug, V: Debug, A: Allocator> Debug for BoxMap<K, V, A> {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_map().entries(self.iter()).finish()
            }
        }

        impl<K, V, A: Allocator> From<VecMap<K, V, A>> for BoxMap<K, V, A> {
            #[inline]
            fn from(value: VecMap<K, V, A>) -> Self {
                Self { inner: value.into_box() }
            }
        }

        impl<Q: ?Sized + Eq, K: Borrow<Q>, V, A: Allocator> Index<&Q> for BoxMap<K, V, A> {
            type Output = V;

            #[inline]
            fn index(&self, index: &Q) -> &Self::Output {
                self.get(index).expect("index not found")
            }
        }

        impl<Q: ?Sized + Eq, K: Borrow<Q>, V, A: Allocator> IndexMut<&Q> for BoxMap<K, V, A> {
            #[inline]
            fn index_mut (&mut self, index: &Q) -> &mut Self::Output {
                self.get_mut(index).expect("index not found")
            }
        }
    } else {
        impl<K, V> BoxMap<K, V> {
            #[inline]
            pub unsafe fn from_vec_unchecked (vec: Vec<(K, V)>) -> Self {
                return Self::from_box_unchecked(vec.into_boxed_slice())
            }

            #[inline]
            pub unsafe fn from_box_unchecked (bx: Box<[(K, V)]>) -> Self {
                return Self { inner: bx }
            }

            #[inline]
            pub fn into_vec (self) -> Vec<(K, V)> {
                return self.inner.into_vec()
            }

            #[inline]
            pub fn into_box (self) -> Box<[(K, V)]> {
                return self.inner
            }

            #[inline]
            pub fn into_keys (self) -> IntoKeys<K, V> {
                return IntoKeys(self.into_vec().into_iter())
            }

            #[inline]
            pub fn into_values (self) -> IntoValues<K, V> {
                return IntoValues(self.into_vec().into_iter())
            }
        }

        impl<K, V> IntoIterator for BoxMap<K, V> {
            type Item = (K, V);
            type IntoIter = IntoIter<K, V>;

            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                self.into_vec().into_iter()
            }
        }

        impl<'a, K, V> IntoIterator for &'a BoxMap<K, V> {
            type Item = (&'a K, &'a V);
            type IntoIter = Iter<'a, K, V>;

            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                BoxMap::iter(self)
            }
        }

        impl<'a, K, V> IntoIterator for &'a mut BoxMap<K, V> {
            type Item = (&'a K, &'a mut V);
            type IntoIter = IterMut<'a, K, V>;

            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                BoxMap::iter_mut(self)
            }
        }

        impl<K: Eq, V> FromIterator<(K, V)> for BoxMap<K, V> {
            #[inline]
            fn from_iter<T: IntoIterator<Item = (K, V)>> (iter: T) -> Self {
                VecMap::from_iter(iter).into()
            }
        }

        impl<K, V> Default for BoxMap<K, V> {
            #[inline]
            fn default () -> Self {
                return Self { inner: Box::new([]) }
            }
        }

        impl<K: Debug, V: Debug> Debug for BoxMap<K, V> {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_map().entries(self.iter()).finish()
            }
        }

        impl<K, V> From<VecMap<K, V>> for BoxMap<K, V> {
            #[inline]
            fn from(value: VecMap<K, V>) -> Self {
                Self { inner: value.into_box() }
            }
        }

        impl<Q: ?Sized + Eq, K: Borrow<Q>, V> Index<&Q> for BoxMap<K, V> {
            type Output = V;

            #[inline]
            fn index(&self, index: &Q) -> &Self::Output {
                self.get(index).expect("index not found")
            }
        }

        impl<Q: ?Sized + Eq, K: Borrow<Q>, V> IndexMut<&Q> for BoxMap<K, V> {
            #[inline]
            fn index_mut (&mut self, index: &Q) -> &mut Self::Output {
                self.get_mut(index).expect("index not found")
            }
        }
    }
}

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct Keys<'a, K: 'a, V: 'a>(core::slice::Iter<'a, (K, V)>);

impl<'a, K, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (key, _) = self.0.next()?;
        return Some(key);
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (key, _) = self.0.nth(n)?;
        return Some(key);
    }

    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.0.count()
    }

    #[inline]
    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        let (key, _) = self.0.last()?;
        return Some(key);
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
        return Some(key);
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let (key, _) = self.0.nth_back(n)?;
        return Some(key);
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
pub struct Values<'a, K: 'a, V: 'a>(core::slice::Iter<'a, (K, V)>);

impl<'a, K, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (_, values) = self.0.next()?;
        return Some(values);
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (_, values) = self.0.nth(n)?;
        return Some(values);
    }

    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.0.count()
    }

    #[inline]
    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        let (_, values) = self.0.last()?;
        return Some(values);
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
        return Some(values);
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let (_, values) = self.0.nth_back(n)?;
        return Some(values);
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
pub struct ValuesMut<'a, K: 'a, V: 'a>(core::slice::IterMut<'a, (K, V)>);

impl<'a, K, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (_, values) = self.0.next()?;
        return Some(values);
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (_, values) = self.0.nth(n)?;
        return Some(values);
    }

    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.0.count()
    }

    #[inline]
    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        let (_, values) = self.0.last()?;
        return Some(values);
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
        return Some(values);
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let (_, values) = self.0.nth_back(n)?;
        return Some(values);
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
pub struct IntoKeys<K, V, #[cfg(feature = "alloc")] A: Allocator = Global>(
    #[cfg(feature = "alloc")] alloc::vec::IntoIter<(K, V), A>,
    #[cfg(not(feature = "alloc"))] alloc::vec::IntoIter<(K, V)>,
);

impl<K, V> Iterator for IntoKeys<K, V> {
    type Item = K;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (key, _) = self.0.next()?;
        return Some(key);
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (key, _) = self.0.nth(n)?;
        return Some(key);
    }

    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.0.count()
    }

    #[inline]
    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        let (key, _) = self.0.last()?;
        return Some(key);
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
        return Some(key);
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let (key, _) = self.0.nth_back(n)?;
        return Some(key);
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
pub struct IntoValues<K, V, #[cfg(feature = "alloc")] A: Allocator = Global>(
    #[cfg(feature = "alloc")] alloc::vec::IntoIter<(K, V), A>,
    #[cfg(not(feature = "alloc"))] alloc::vec::IntoIter<(K, V)>,
);

impl<K, V> Iterator for IntoValues<K, V> {
    type Item = V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (_, values) = self.0.next()?;
        return Some(values);
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (_, values) = self.0.nth(n)?;
        return Some(values);
    }

    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.0.count()
    }

    #[inline]
    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        let (_, values) = self.0.last()?;
        return Some(values);
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
        return Some(values);
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let (_, values) = self.0.nth_back(n)?;
        return Some(values);
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
pub struct Iter<'a, K: 'a, V: 'a>(core::slice::Iter<'a, (K, V)>);

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (key, value) = self.0.next()?;
        return Some((key, value));
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (key, value) = self.0.nth(n)?;
        return Some((key, value));
    }

    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.0.count()
    }

    #[inline]
    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        let (key, value) = self.0.last()?;
        return Some((key, value));
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
        return Some((key, value));
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let (key, value) = self.0.nth_back(n)?;
        return Some((key, value));
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
pub struct IterMut<'a, K: 'a, V: 'a>(core::slice::IterMut<'a, (K, V)>);

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (key, value) = self.0.next()?;
        return Some((key, value));
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (key, value) = self.0.nth(n)?;
        return Some((key, value));
    }

    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.0.count()
    }

    #[inline]
    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        let (key, value) = self.0.last()?;
        return Some((key, value));
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
        return Some((key, value));
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let (key, value) = self.0.nth_back(n)?;
        return Some((key, value));
    }
}

impl<'a, K, V> ExactSizeIterator for IterMut<'a, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}
