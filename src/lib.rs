use std::collections::HashMap;

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
