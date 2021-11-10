use crate::{
    collections,
    error::Error,
    tree::{BranchKey, BranchNode},
    H256,
};

/// Trait for customize hash function
pub trait Hasher {
    fn write_h256(&mut self, h: &H256);
    fn write_byte(&mut self, b: u8);
    fn finish(self) -> H256;
}

/// Trait for define value structures
pub trait Value {
    fn to_h256(&self) -> H256;
    fn zero() -> Self;
}

impl Value for H256 {
    fn to_h256(&self) -> H256 {
        *self
    }
    fn zero() -> Self {
        H256::zero()
    }
}

/// Trait for customize backend storage
pub trait Store<V> {
    fn get_branch(&self, branch_key: &BranchKey) -> Result<Option<BranchNode>, Error>;
    fn get_leaf(&self, leaf_key: &H256) -> Result<Option<V>, Error>;
    fn prefetch_branches<'a>(
        &self,
        branch_keys: impl Iterator<Item = &'a BranchKey>,
    ) -> Result<Map<BranchKey, BranchNode>, Error>;
    fn insert_branch(&mut self, node_key: BranchKey, branch: BranchNode) -> Result<(), Error>;
    fn insert_leaf(&mut self, leaf_key: H256, leaf: V) -> Result<(), Error>;
    fn remove_branch(&mut self, node_key: &BranchKey) -> Result<(), Error>;
    fn remove_leaf(&mut self, leaf_key: &H256) -> Result<(), Error>;
}

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        pub type Map<K, V> = collections::HashMap<K, V>;
        pub type Entry<'a, K, V> = collections::hash_map::Entry<'a, K, V>;
    } else {
        pub type Map<K, V> = collections::BTreeMap<K, V>;
        pub type Entry<'a, K, V> = collections::btree_map::Entry<'a, K, V>;
    }
}
