use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

#[derive(Debug)]
pub enum Commands {
    STRING,
    ERROR,
    INTEGER,
    BULK,
    ARRAY,
    UNKNOWN,
}

#[derive(Debug)]
pub struct Value {
    pub typ: Commands,
    // str: String,
    // num: u64,
    // bulk: String,
    pub array: Vec<String>,
}

pub fn get_command_type(cmd_typ: &str) -> Commands {
    match cmd_typ {
        "+" => Commands::STRING,
        "-" => Commands::ERROR,
        ":" => Commands::INTEGER,
        "$" => Commands::BULK,
        "*" => Commands::ARRAY,
        _ => panic!("Invalid command type"),
    }
}

pub struct Parser<'a> {
    stream: &'a TcpStream,
}

impl<'a> Parser<'a> {
    pub fn new(stream: &'a TcpStream) -> Parser<'a> {
        Parser { stream }
    }
    pub fn parse(self) -> Value {
        let buf_reader = BufReader::new(self.stream);

        let mut array_size: Option<usize> = None;
        let mut counter = 0;
        let mut value = Value {
            typ: Commands::UNKNOWN,
            array: vec![],
        };

        let it = buf_reader.lines().map(|line| line.unwrap());

        for line in it {
            if array_size.is_none() {
                // determine command type
                let (cmd_type, size) = line.split_at(1);

                value.typ = get_command_type(cmd_type);

                // we double the size here is each value is always in the format:
                // $5 hello
                // $5 -> string with size of 5 will follow
                // hello -> the actual value
                let size = size.parse::<usize>().unwrap() * 2;

                array_size = Some(size);
                value.array = Vec::with_capacity(size);
            } else {
                counter += 1;

                value.array.push(line);
            }

            if array_size.unwrap() == counter {
                break;
            }
        }

        value
    }
}
