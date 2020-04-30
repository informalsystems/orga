use super::prefix::Prefixed;
use super::share::Shared;
use super::{Store, Read, Write};
use crate::Result;

// TODO: can we do this without copying every time we prefix the key? can
// possibly change Store methods to generically support iterator-based
// concatenated keys, maybe via a Key type.

pub struct Splitter<S: Store> {
    store: Shared<S>,
    index: u8
}

impl<S: Store> Splitter<S> {
    pub fn new(store: S) -> Self {
        Splitter {
            store: store.into_shared(),
            index: 0
        }
    }

    pub fn split(&mut self) -> Prefixed<Shared<S>> {
        if self.index == 255 {
            panic!("Reached split limit");
        }
        
        let index = self.index;
        self.index += 1;

        self.store
            .clone()
            .prefix(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MapStore, Read, Write};

    #[test]
    fn split() {
        let mut store = MapStore::new();

        let mut splitter = Splitter::new(&mut store);
        let mut sub0 = splitter.split();
        let mut sub1 = splitter.split();

        sub0.put(vec![123], vec![5]).unwrap();
        assert_eq!(sub0.get(&[123]).unwrap(), Some(vec![5]));
        assert_eq!(sub1.get(&[123]).unwrap(), None);

        sub1.put(vec![123], vec![6]).unwrap();
        assert_eq!(sub0.get(&[123]).unwrap(), Some(vec![5]));
        assert_eq!(sub1.get(&[123]).unwrap(), Some(vec![6]));

        assert_eq!(store.get(&[0, 123]).unwrap(), Some(vec![5]));
        assert_eq!(store.get(&[1, 123]).unwrap(), Some(vec![6]));
    }
}
