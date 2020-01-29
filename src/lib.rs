//!
//!  A Key-Value in-memory storage.
//!
//! Example:
//!
//! ```rust
//! # use kvs::KvStore;
//! let mut store = KvStore::new();
//! store.set("key1".to_owned(), "value1".to_owned());
//! assert_eq!(store.get("key1".to_owned()), Some("value1".to_owned()));
//! store.remove("key1".to_owned());
//! assert_eq!(store.get("key1".to_owned()), None);
//! ```
use std::collections::HashMap;

/// The struct contains inner data strcture.
#[derive(Debug, Default)]
pub struct KvStore {
    inner: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set(self: &mut Self, key: String, value: String) -> Option<String> {
        self.inner.insert(key, value)
    }

    pub fn get(self: &mut Self, key: String) -> Option<String> {
        self.inner.get(key.as_str()).map(|v| v.to_owned())
    }

    pub fn remove(self: &mut Self, key: String) -> Option<String> {
        self.inner.remove(key.as_str())
    }
}
