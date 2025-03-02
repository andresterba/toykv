mod parser;
mod handler;
mod store;
mod config;

use std::net::TcpListener;
use store::Store;

const SERVER_ADDRESS: &str = "127.0.0.1:6379";

fn main() {
    let config = config::get_config();
    let listener = TcpListener::bind(SERVER_ADDRESS).unwrap();
    let parser = parser::Parser::new();

    let mut store = store::MapStore::new(config.file_name);
    store.load();

    let mut command_handler = handler::CommandHandler::new(&mut store);

    println!("server ready to accept connections on {}", SERVER_ADDRESS);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let value = parser.parse(&stream);
        let command = value.array[0].to_string();

        command_handler.handle_command(stream, command, value);
    }
}
