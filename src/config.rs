use std::env;

pub struct Config {
    pub file_name: String,
    pub server_address: String,
}


pub fn get_config() -> Config {
    let file_name = env::var("FILE_NAME").unwrap_or("aof.txt".to_string());
    let server_address = env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:6379".to_string());

    Config {
        file_name,
        server_address,
    }
}
