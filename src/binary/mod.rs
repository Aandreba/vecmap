use alloc::{vec::Vec, boxed::Box};
use core::{borrow::Borrow, ops::{Index, IndexMut}};

flat_mod! { entry }

pub type Iter<'a, K, V> = crate::vec::Iter<'a, K, V>;
pub type IterMut<'a, K, V> = crate::vec::IterMut<'a, K, V>;
pub type IntoIter<K, V> = crate::vec::IntoIter<K, V>;
pub type Drain<'a, K, V> = alloc::vec::Drain<'a, (K, V)>;

pub type Keys<'a, K, V> = crate::vec::Keys<'a, K, V>;
pub type Values<'a, K, V> = crate::vec::Values<'a, K, V>;

pub type IntoKeys<K, V> = crate::vec::IntoKeys<K, V>;
pub type IntoValues<K, V> = crate::vec::IntoValues<K, V>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinaryMap<K, V> {
    inner: Vec<(K, V)>,
}

impl<K, V> BinaryMap<K, V> {
    #[inline]
    pub const fn new() -> Self {
        Self { inner: Vec::new() }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear()
    }
}

impl<K, V> BinaryMap<K, V> {
    #[inline]
    pub fn contains_key<Q>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: ?Sized + Ord,
    {
        self.inner
            .binary_search_by(|(x, _)| x.borrow().cmp(k))
            .is_ok()
    }

    #[inline]
    pub fn drain(&mut self) -> Drain<'_, K, V> {
        self.inner.drain(..)
    }
}

impl<K: Ord, V> BinaryMap<K, V> {
    #[inline]
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        return match self.entry(k) {
            Entry::Occupied(mut x) => Some(x.insert(v)),
            Entry::Vacant(x) => {
                x.insert(v);
                None
            }
        };
    }

    #[inline]
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        match self.inner.binary_search_by(|(x, _)| x.cmp(&key)) {
            Ok(idx) => Entry::Occupied(OcuppiedEntry {
                parent: &mut self.inner,
                idx,
            }),
            Err(idx) => Entry::Vacant(VacantEntry {
                parent: &mut self.inner,
                key,
                idx,
            }),
        }
    }
}

impl<K, V> BinaryMap<K, V> {
    #[inline]
    pub fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Ord,
    {
        match self.inner.binary_search_by(|(x, _)| x.borrow().cmp(k)) {
            Ok(idx) => unsafe { Some(&self.inner.get_unchecked(idx).1) },
            Err(_) => None,
        }
    }

    #[inline]
    pub fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Ord,
    {
        match self.inner.binary_search_by(|(x, _)| x.borrow().cmp(k)) {
            Ok(idx) => unsafe { Some(&mut self.inner.get_unchecked_mut(idx).1) },
            Err(_) => None,
        }
    }

    #[inline]
    pub fn get_key_value<Q>(&self, k: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Ord,
    {
        match self.inner.binary_search_by(|(x, _)| x.borrow().cmp(k)) {
            Ok(idx) => unsafe {
                let (key, value) = self.inner.get_unchecked(idx);
                Some((key, value))
            },
            Err(_) => None,
        }
    }

    #[inline]
    pub fn remove<Q>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Ord,
    {
        return match self.inner.binary_search_by(|(x, _)| x.borrow().cmp(k)) {
            Ok(idx) => Some(self.inner.remove(idx).1),
            Err(_) => None,
        };
    }

    #[inline]
    pub fn remove_entry<Q>(&mut self, k: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Ord,
    {
        return match self.inner.binary_search_by(|(x, _)| x.borrow().cmp(k)) {
            Ok(idx) => Some(self.inner.remove(idx)),
            Err(_) => None,
        };
    }
}


impl<K, V> BinaryMap<K, V> {
    #[inline]
    pub fn iter(&self) -> Iter<'_, K, V> {
        return crate::vec::Iter(self.inner.iter())
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        return crate::vec::IterMut(self.inner.iter_mut())
    }

    #[inline]
    pub fn keys(&self) -> Keys<'_, K, V> {
        return crate::vec::Keys(self.inner.iter())
    }

    #[inline]
    pub fn values(&self) -> Values<'_, K, V> {
        return crate::vec::Values(self.inner.iter())
    }

    #[inline]
    pub fn into_keys(self) -> IntoKeys<K, V> {
        return crate::vec::IntoKeys(self.inner.into_iter())
    }

    #[inline]
    pub fn into_values(self) -> IntoValues<K, V> {
        return crate::vec::IntoValues(self.inner.into_iter())
    }
}

impl<K, V> Default for BinaryMap<K, V> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> IntoIterator for BinaryMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a, K, V> IntoIterator for &'a BinaryMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        BinaryMap::iter(self)
    }
}

impl<'a, K, V> IntoIterator for &'a mut BinaryMap<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        BinaryMap::iter_mut(self)
    }
}

impl<K: Ord, V> Extend<(K, V)> for BinaryMap<K, V> {
    #[inline]
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (k, v) in iter.into_iter() { let _ = self.insert(k, v); }
    }
}

impl<'a, 'b, K: Ord + Clone, V: Clone> Extend<(&'a K, &'b V)> for BinaryMap<K, V> {
    #[inline]
    fn extend<T: IntoIterator<Item = (&'a K, &'b V)>>(&mut self, iter: T) {
        <Self as Extend<(K, V)>>::extend(self, iter.into_iter().map(|(x, y)| (x.clone(), y.clone())))
    }
}

impl<Q: ?Sized + Ord, K: Borrow<Q>, V> Index<&Q> for BinaryMap<K, V> {
    type Output = V;

    #[inline]
    fn index(&self, index: &Q) -> &Self::Output {
        self.get(index).expect("index not found")
    }
}

impl<Q: ?Sized + Ord, K: Borrow<Q>, V> IndexMut<&Q> for BinaryMap<K, V> {
    #[inline]
    fn index_mut(&mut self, index: &Q) -> &mut Self::Output {
        self.get_mut(index).expect("index not found")
    }
}

impl<K: Ord, V> From<Vec<(K, V)>> for BinaryMap<K, V> {
    #[inline]
    fn from(mut inner: Vec<(K, V)>) -> Self {
        inner.sort_unstable_by(|(x, _), (y, _)| x.cmp(y));
        Self { inner }
    }
}

impl<K: Ord, V> From<Box<[(K, V)]>> for BinaryMap<K, V> {
    #[inline]
    fn from(inner: Box<[(K, V)]>) -> Self {
        inner.into_vec().into()
    }
}

impl<K: Ord, V, const N: usize> From<[(K, V); N]> for BinaryMap<K, V> {
    #[inline]
    fn from(inner: [(K, V); N]) -> Self {
        Vec::from(inner).into()
    }
}