use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufRead, Write},
    path::Path,
};

const FILE_NAME: &str = "aof.txt";

pub struct MapStore {
    map: HashMap<String, String>,
}

pub trait Store {
    fn new() -> MapStore;

    fn set(&mut self, key: String, value: String);
    fn get(&self, key: String) -> Option<&String>;

    fn persist(&self, key: String, value: String);
    fn load(&mut self);
}

impl Store for MapStore {
    fn new() -> MapStore {
        MapStore {
            map: HashMap::new(),
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
        let file = OpenOptions::new().write(true).append(true).open(FILE_NAME);

        match file {
            Ok(mut file) => {
                if let Err(e) = writeln!(file, "{}", kv_pair) {
                    eprintln!("Couldn't write to file: {}", e);
                }
            }
            Err(_e) => {
                let path = Path::new(FILE_NAME);
                let mut file = File::create(&path).expect("couldn't create file");
                if let Err(e) = writeln!(file, "{}", kv_pair) {
                    eprintln!("Couldn't write to file: {}", e);
                }
            }
        }
    }

    fn load(&mut self) {
        let file = File::open(FILE_NAME);

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
    use super::*;

    #[test]
    fn test_1() {
        let mut store = MapStore::new();
        store.set("key".to_string(), "value".to_string());
        let value = store.get("key".to_string());
        assert_eq!(value, Some(&"value".to_string()));
    }

    #[test]
    fn test_2() {
        let mut store = MapStore::new();
        store.set("key".to_string(), "value".to_string());
        store.set("key2".to_string(), "value2".to_string());
        let value = store.get("key2".to_string());
        assert_eq!(value, Some(&"value2".to_string()));
    }
}
