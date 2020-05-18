use crate::{Result, Store};

mod value;
mod wrapper;

pub use value::Value;
pub use wrapper::WrapperStore;

pub trait State: Sized {
    type Store: Store;

    fn wrap_store(store: Self::Store) -> Result<Self>;
}

pub trait IntoState {
    type Store: Store;
    type Target: State;

    fn into_state(self, store: Self::Store) -> Result<Self::Target>;
}
