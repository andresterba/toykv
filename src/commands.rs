use std::{io::Write, net::TcpStream};

use crate::{parser, store::Store};

pub struct CommandHandler<'a, T: Store> {
    store: &'a mut T,
}

impl<'a, T: Store> CommandHandler<'a, T> {
    pub fn new(store: &'a mut T) -> CommandHandler<'a, T> {
        CommandHandler { store }
    }

    pub fn handle_command(&mut self, mut stream: TcpStream, command: String, value: parser::Value) {
        match command.as_str() {
            "ping" => {
                self.handle_ping(stream, value.array[3].to_string());
            }
            "set" => {
                if value.array.len() != 6 {
                    stream
                        .write_all(
                            "-ERR wrong number of arguments for 'set' command\r\n".as_bytes(),
                        )
                        .unwrap();
                    return;
                }

                self.handle_set(stream, value.array[3].to_string(), value.array[5].to_string());
            }
            "get" => {
                if value.array.len() != 4 {
                    stream
                        .write_all(
                            "-ERR wrong number of arguments for 'get' command\r\n".as_bytes(),
                        )
                        .unwrap();
                    return;
                }

                self.handle_get(stream, value.array[3].to_string());
            }
            _ => {}
        }
    }

    fn handle_ping(&mut self, mut stream: TcpStream, payload: String) {
        let mut prefix = String::from("+");

        prefix.push_str(&payload.as_str());
        prefix.push_str("\r\n");

        let value_to_write = prefix.as_str();

        // self.stream.write_all(value_to_write.as_bytes()).unwrap();
        stream.write_all(value_to_write.as_bytes()).unwrap();
    }

    fn handle_set(&mut self, mut stream: TcpStream, key: String, value: String) {
        self.store.set(key, value);

        stream.write_all("+OK\r\n".as_bytes()).unwrap();
    }

    fn handle_get(&mut self, mut stream: TcpStream, key: String) {
        let value = self.store.get(key);

        match value {
            Some(v) => {
                let mut prefix = String::from("+");

                prefix.push_str(&v.as_str());
                prefix.push_str("\r\n");

                let value_to_write = prefix.as_str();

                stream.write_all(value_to_write.as_bytes()).unwrap();
            }
            None => {
                stream.write_all("$-1\r\n".as_bytes()).unwrap();
            }
        }
    }
}
