mod config;
mod handler;
mod parser;
mod store;

use std::net::TcpListener;
use store::Store;

fn main() {
    let config = config::get_config();
    let listener = TcpListener::bind(&config.server_address).unwrap();
    let parser = parser::Parser::new();

    let mut store = store::MapStore::new(config.file_name);
    store.load();

    let mut command_handler = handler::CommandHandler::new(&mut store);

    println!(
        "server ready to accept connections on {}",
        config.server_address
    );

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let value = parser.parse(&stream);
        let command = value.array[0].to_string();

        command_handler.handle_command(stream, command, value);
    }
}
