use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref STORE: Mutex<HashMap<String, Vec<u8>>> = Mutex::new(HashMap::new());
}

pub fn insert(key: &str, value: &[u8]) {
    STORE
        .lock()
        .unwrap()
        .insert(key.to_string(), value.to_vec());
}
pub fn get(key: &str) -> Option<Vec<u8>> {
    STORE.lock().unwrap().get(key).cloned()
}
pub fn delete(key: &str) {
    STORE.lock().unwrap().remove(key);
}
pub fn list() -> Vec<String> {
    STORE.lock().unwrap().keys().cloned().collect()
}
pub fn clear() {
    STORE.lock().unwrap().clear();
}
pub fn exists(key: &str) -> bool {
    STORE.lock().unwrap().contains_key(key)
}
