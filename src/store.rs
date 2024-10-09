use std::collections::HashMap;


pub struct MapStore {
    map: HashMap<String, String>
}

pub trait Store {
     fn new() -> MapStore;

     fn set(&mut self, key: String, value: String);

     fn get(&self, key: String) -> String;
}

impl Store for MapStore {
     fn new() -> MapStore{
        MapStore{
            map: HashMap::new()
        }
    }

     fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);

    }

     fn get(&self, key: String) -> String {
        let value = self.map.get(&key).unwrap();

        value.to_owned()
    }
}