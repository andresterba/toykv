use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufRead, Write},
    path::Path,
};

pub struct MapStore {
    filename: String,
    map: HashMap<String, String>,
}

pub trait Store {
    fn new(file_name: String) -> MapStore;

    fn set(&mut self, key: String, value: String);
    fn get(&self, key: String) -> Option<&String>;

    fn persist(&self, key: String, value: String);
    fn load(&mut self);
}

impl Store for MapStore {
    fn new(file_name: String) -> MapStore {
        MapStore {
            map: HashMap::new(),
            filename: file_name,
        }
    }

    fn set(&mut self, key: String, value: String) {
        self.map.insert(key.clone(), value.clone());
        self.persist(key, value);
    }

    fn get(&self, key: String) -> Option<&String> {
        let value = self.map.get(&key);

        value
    }

    fn persist(&self, key: String, value: String) {
        let kv_pair = format!("{}={}", key, value);
        let file = OpenOptions::new().write(true).append(true).open(&self.filename);

        match file {
            Ok(mut file) => {
                if let Err(e) = writeln!(file, "{}", kv_pair) {
                    eprintln!("Couldn't write to file: {}", e);
                }
            }
            Err(_e) => {
                let path = Path::new(&self.filename);
                let mut file = File::create(&path).expect("couldn't create file");
                if let Err(e) = writeln!(file, "{}", kv_pair) {
                    eprintln!("Couldn't write to file: {}", e);
                }
            }
        }
    }

    fn load(&mut self) {
        let file = File::open(&self.filename);

        match file {
            Ok(file) => {
                let reader = std::io::BufReader::new(file);

                reader.lines().for_each(|line| {
                    let line = line.unwrap();

                    if line.len() != 0 {
                        let kv: Vec<&str> = line.split('=').collect();

                        println!("{kv:#?}");

                        self.map.insert(kv[0].to_string(), kv[1].to_string());
                    }
                });
            }
            Err(_e) => {
                eprintln!("Couldn't open file");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn cleanup() {
        let path = Path::new("test-file.txt");

        match fs::remove_file(path) {
            Ok(_) => {},
            Err(_) => {
                panic!("failed to clean up test artifacts")
            },
            }

    }

    #[test]
    fn test_1() {
        let mut store = MapStore::new("test-file.txt".to_string());
        store.set("key".to_string(), "value".to_string());
        let value = store.get("key".to_string());
        assert_eq!(value, Some(&"value".to_string()));
        cleanup();
    }

    #[test]
    fn test_2() {
        let mut store = MapStore::new("test-file.txt".to_string());
        store.set("key".to_string(), "value".to_string());
        store.set("key2".to_string(), "value2".to_string());
        let value = store.get("key2".to_string());
        assert_eq!(value, Some(&"value2".to_string()));
        cleanup();
    }
}
