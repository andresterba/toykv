pub mod parser;

mod commands;
mod store;

use std::net::TcpListener;
use store::Store;

const SERVER_ADDRESS: &str = "127.0.0.1:6379";

fn main() {
    let listener = TcpListener::bind(SERVER_ADDRESS).unwrap();
    let parser = parser::Parser::new();
    let mut store = store::MapStore::new();
    store.load();

    let mut command_handler = commands::CommandHandler::new(&mut store);

    println!("server ready to accept connections on {}", SERVER_ADDRESS);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let value = parser.parse(&stream);

        println!("{value:?}");
        command_handler.handle_command(stream, value.array[1].to_string(), value);
    }
}
