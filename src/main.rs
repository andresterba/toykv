pub mod parser;

mod commands;
mod store;

use std::net::TcpListener;
use store::Store;

const SERVER_ADDRESS: &str = "127.0.0.1:6379";

fn main() {
    let listener = TcpListener::bind(SERVER_ADDRESS).unwrap();
    let mut store = store::MapStore::new();
    store.load();

    println!("server ready to accept connections on {}", SERVER_ADDRESS);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let parser = parser::Parser::new(&stream);
        let mut command_handler = commands::CommandHandler::new(&stream, &mut store);

        let value = parser.parse();
        println!("{value:?}");
        command_handler.handle_command(value.array[1].to_string(), value);

        // let mut value_to_write = "+OK\r\n";
        // let mut prefix = String::from("+");

        // if value.array[1] == "ping" {
        //     prefix.push_str(value.array[3].as_str());
        //     prefix.push_str("\r\n");

        //     value_to_write = prefix.as_str();
        // }

        // stream.write_all(value_to_write.as_bytes()).unwrap();

        // handle_connection(stream);
    }
}
