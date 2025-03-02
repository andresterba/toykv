use std::env;

pub struct Config {
    pub file_name: String,
}


pub fn get_config() -> Config {
    let file_name = env::var("FILE_NAME").unwrap_or("aof.txt".to_string());

    Config {
        file_name
    }
}
