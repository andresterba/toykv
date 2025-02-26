use std::io::{BufRead, BufReader, Read};


#[derive(Debug, PartialEq)]
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
        // "-" => Commands::ERROR,
        // ":" => Commands::INTEGER,
        // "$" => Commands::BULK,
        "*" => Commands::ARRAY,
        _ => panic!("Invalid command type"),
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Parser {}

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }

    // Use Read trait instead of a real TcpStream here to enable easier testing.
    pub fn parse<T: Read>(self, stream: T) -> Value {
        let buf_reader = BufReader::new(stream);

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

                // we double the size here as each value is always in the format:
                // $5 hello
                // $5: string with size of 5 will follow
                // hello: the actual value
                let size = size.parse::<usize>().unwrap() * 2;

                array_size = Some(size);
                value.array = Vec::with_capacity(size);
            } else {
                counter += 1;

                println!("{line}");

                // Skip unnesesarry size indicators during array construction.
                if !line.starts_with("$") {
                    value.array.push(line);
                }
            }

            if array_size.unwrap() == counter {
                break;
            }
        }

        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inputs() {
        let input = b"*3\r\n$3\r\nset\r\n$5\r\nadmin\r\n$5\r\nandre";
        let expected: Vec<String> = vec![
            "set".to_string(),
            "admin".to_string(),
            "andre".to_string(),
        ];

        let parser = Parser {};
        let answer = parser.parse(&input[..]);

        assert_eq!(Commands::ARRAY, answer.typ);
        assert_eq!(expected, answer.array);
    }

    #[test]
    fn ping() {
        let input = b"*3\r\n$3\r\nset\r\n$5\r\nadmin\r\n$5\r\nandre";
        let expected: Vec<String> = vec![
            "set".to_string(),
            "admin".to_string(),
            "andre".to_string(),
        ];

        let parser = Parser {};
        let answer = parser.parse(&input[..]);

        assert_eq!(Commands::ARRAY, answer.typ);
        assert_eq!(expected, answer.array);
    }
}
