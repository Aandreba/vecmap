#[cfg(feature = "alloc")]
use core::alloc::*;
use core::marker::PhantomData;
use serde::{Serialize, ser::SerializeMap, Deserialize, de::Visitor};
use crate::{vec::VecMap, r#box::BoxMap};

cfg_if::cfg_if! {
    if #[cfg(feature = "alloc")] {
        impl<K: Serialize, V: Serialize, A: Allocator> Serialize for VecMap<K, V, A> {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
                let mut ser = serializer.serialize_map(Some(self.len()))?;
                for (key, value) in self {
                    ser.serialize_entry(key, value)?;
                }
                return ser.end()
            }
        }
        
        impl<K: Serialize, V: Serialize, A: Allocator> Serialize for BoxMap<K, V, A> {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
                let mut ser = serializer.serialize_map(Some(self.len()))?;
                for (key, value) in self {
                    ser.serialize_entry(key, value)?;
                }
                return ser.end()
            }
        }
        
        impl<'de, K: 'de + Eq + Deserialize<'de>, V: 'de + Deserialize<'de>, A: Allocator + Default> Deserialize<'de> for VecMap<K, V, A> {
            #[inline]
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct LocalVisitor<'de, K, V, A> (PhantomData<(&'de (K, V), A)>);
                impl<'de, K: Eq + Deserialize<'de>, V: Deserialize<'de>, Al: Allocator + Default> Visitor<'de> for LocalVisitor<'de, K, V, Al> {
                    type Value = VecMap<K, V, Al>;
        
                    #[inline]
                    fn expecting(&self, formatter: &mut alloc::fmt::Formatter) -> alloc::fmt::Result {
                        formatter.write_str("a map")
                    }
        
                    #[inline]
                    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de>, {
                        let mut result = VecMap::with_capacity_in(map.size_hint().unwrap_or_default(), Al::default());
                        while let Some((key, value)) = map.next_entry()? {
                            let _ = result.insert(key, value);
                        }
                        return Ok(result)
                    }
                }
                
                return deserializer.deserialize_map(LocalVisitor::<'de, K, V, A>(PhantomData))
            }
        }
        
        impl<'de, K: 'de + Eq + Deserialize<'de>, V: 'de + Deserialize<'de>, A: Allocator + Default> Deserialize<'de> for BoxMap<K, V, A> {
            #[inline]
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                VecMap::<K, V, A>::deserialize(deserializer).map(Into::into)
            }
        }
    } else {
        impl<K: Serialize, V: Serialize> Serialize for VecMap<K, V> {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
                let mut ser = serializer.serialize_map(Some(self.len()))?;
                for (key, value) in self {
                    ser.serialize_entry(key, value)?;
                }
                return ser.end()
            }
        }
        
        impl<K: Serialize, V: Serialize> Serialize for BoxMap<K, V> {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
                let mut ser = serializer.serialize_map(Some(self.len()))?;
                for (key, value) in self {
                    ser.serialize_entry(key, value)?;
                }
                return ser.end()
            }
        }
        
        impl<'de, K: 'de + Eq + Deserialize<'de>, V: 'de + Deserialize<'de>> Deserialize<'de> for VecMap<K, V> {
            #[inline]
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct LocalVisitor<'de, K, V> (PhantomData<&'de (K, V)>);
                impl<'de, K: Eq + Deserialize<'de>, V: Deserialize<'de>> Visitor<'de> for LocalVisitor<'de, K, V> {
                    type Value = VecMap<K, V>;
        
                    #[inline]
                    fn expecting(&self, formatter: &mut alloc::fmt::Formatter) -> alloc::fmt::Result {
                        formatter.write_str("a map")
                    }
        
                    #[inline]
                    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de>, {
                        let mut result = VecMap::with_capacity(map.size_hint().unwrap_or_default());
                        while let Some((key, value)) = map.next_entry()? {
                            let _ = result.insert(key, value);
                        }
                        return Ok(result)
                    }
                }
                
                return deserializer.deserialize_map(LocalVisitor::<'de, K, V>(PhantomData))
            }
        }
        
        impl<'de, K: 'de + Eq + Deserialize<'de>, V: 'de + Deserialize<'de>> Deserialize<'de> for BoxMap<K, V> {
            #[inline]
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                VecMap::<K, V>::deserialize(deserializer).map(Into::into)
            }
        }
    }
}
