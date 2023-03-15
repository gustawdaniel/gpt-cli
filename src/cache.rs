use std::collections::HashMap;
use std::fs::{read_to_string, write};

pub struct Cache {
    path: String,
    map: HashMap<String, String>,
}

impl Cache {
    pub fn new(path: Option<&str>) -> Self {
        let default_path = format!("{}/.gpt-cache.json", dirs::home_dir().unwrap().display());
        let path = path.map_or(default_path, |p| p.to_string());
        Cache {
            path,
            map: HashMap::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn get(&mut self, key: &str) -> Option<String> {
        if !std::path::Path::new(&self.path).exists() {
            return None;
        }
        if self.is_empty() {
            let contents = read_to_string(&self.path).unwrap();
            self.map = serde_json::from_str(&contents).unwrap();
        }
        self.map.get(key).cloned()
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.map.insert(key.to_string(), value.to_string());
        let contents = serde_json::to_string(&self.map).unwrap();
        write(&self.path, contents).unwrap();
    }
}

#[cfg(test)]
mod tests {
    mod rand_hash;

    use super::*;
    use crate::cache::tests::rand_hash::get_random_hash;
    use std::fs::remove_file;

    #[test]
    fn test_new_with_path() {
        let cache = Cache::new(Some("/tmp/test_cache.json"));
        assert_eq!(cache.path, "/tmp/test_cache.json");
    }

    #[test]
    fn test_new_without_path() {
        let cache = Cache::new(None);
        let default_path = format!("{}/.gpt-cache.json", dirs::home_dir().unwrap().display());
        assert_eq!(cache.path, default_path);
    }

    #[test]
    fn test_get_nonexistent_key() {
        let path = &format!("/tmp/.gpt-cache-{}.json", get_random_hash());
        let mut cache = Cache::new(Some(path));
        assert_eq!(cache.get("nonexistent"), None);
    }

    #[test]
    fn test_set_and_get() {
        let path = &format!("/tmp/.gpt-cache-{}.json", get_random_hash());
        let mut cache = Cache::new(Some(path));
        cache.set("key", "value");
        assert_eq!(cache.get("key"), Some("value".to_string()));
        remove_file(path).expect("can't remove cache file");
    }

    #[test]
    fn test_set_multiple_and_get() {
        let path = &format!("/tmp/.gpt-cache-{}.json", get_random_hash());
        let mut cache = Cache::new(Some(path));
        cache.set("key1", "value1");
        cache.set("key2", "value2");
        assert_eq!(cache.get("key1"), Some("value1".to_string()));
        assert_eq!(cache.get("key2"), Some("value2".to_string()));
        remove_file(path).expect("can't remove cache file");
    }

    #[test]
    fn test_set_same_key() {
        let path = &format!("/tmp/.gpt-cache-{}.json", get_random_hash());
        let mut cache = Cache::new(Some(path));
        cache.set("key", "value1");
        cache.set("key", "value2");
        assert_eq!(cache.get("key"), Some("value2".to_string()));
        remove_file(path).expect("can't remove cache file");
    }
}
