use alloc::vec::Vec;

#[derive(Debug)]
pub enum Entry<'a, K, V> {
    Occupied (OcuppiedEntry<'a, K, V>),
    Vacant (VacantEntry<'a, K, V>)
}

#[derive(Debug)]
pub struct OcuppiedEntry<'a, K, V> {
    pub(super) parent: &'a mut Vec<(K, V)>,
    pub(super) idx: usize
}

#[derive(Debug)]
pub struct VacantEntry<'a, K, V> {
    pub(super) parent: &'a mut Vec<(K, V)>,
    pub(super) idx: usize,
    pub(super) key: K,
}

impl<'a, K, V> OcuppiedEntry<'a, K, V> {
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

impl<'a, K, V> VacantEntry<'a, K, V> {
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