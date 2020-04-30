use std::borrow::Borrow;
use std::marker::PhantomData;
use crate::{State, Store, Encode, Decode, Result};

pub struct Map<S, K, V>
    where
        S: Store,
        K: Encode + Decode,
        V: Encode + Decode
{
    store: S,
    key_type: PhantomData<K>,
    value_type: PhantomData<V>
}

impl<S, K, V> State<S> for Map<S, K, V>
    where
        S: Store,
        K: Encode + Decode,
        V: Encode + Decode
{
    fn wrap_store(store: S) -> Result<Self> {
        Ok(Self {
            store,
            key_type: PhantomData,
            value_type: PhantomData
        })
    }
}

impl<S, K, V> Map<S, K, V>
    where
        S: Store,
        K: Encode + Decode,
        V: Encode + Decode
{
    pub fn insert(&mut self, key: K, value: V) -> Result<()> {
        let key_bytes = key.encode()?;
        let value_bytes = value.encode()?;
        self.store.put(key_bytes, value_bytes)
    }

    pub fn delete<B: Borrow<K>>(&mut self, key: B) -> Result<()> {
        let (key_bytes, key_length) = encode_key_array(key.borrow())?;
        self.store.delete(&key_bytes[..key_length])
    }

    pub fn get<B: Borrow<K>>(&self, key: B) -> Result<Option<V>> {
        let (key_bytes, key_length) = encode_key_array(key.borrow())?;
        self.store.get(&key_bytes[..key_length])?
            .map(|value_bytes| V::decode(value_bytes.as_slice()))
            .transpose()
    }
}

fn encode_key_array<K: Encode>(key: &K) -> Result<([u8; 256], usize)> {
    let mut bytes = [0; 256];
    key.encode_into(&mut &mut bytes[..])?;
    Ok((bytes, key.encoding_length()?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn simple() {
        let mut store = MapStore::new();
        let mut map: Map<_, u64, u64> =
            Map::wrap_store(&mut store).unwrap();

        assert_eq!(map.get(1234).unwrap(), None);

        map.insert(1234, 5678).unwrap();
        assert_eq!(map.get(1234).unwrap(), Some(5678));

        map.delete(1234).unwrap();
        assert_eq!(map.get(1234).unwrap(), None);
    }
}
