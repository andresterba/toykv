use std::io::{BufRead, BufReader, Read};

#[derive(Debug, PartialEq)]
pub enum Commands {
    STRING,
    // ERROR,
    // INTEGER,
    // BULK,
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

        let mut value = Value {
            typ: Commands::UNKNOWN,
            array: vec![],
        };

        let mut it = buf_reader.lines().map(|line| line.unwrap());
        let first_line = it.next().unwrap();

        // determine command type
        let (cmd_type, size) = first_line.split_at(1);
        let size = size.parse::<usize>().unwrap();

        value.typ = get_command_type(cmd_type);
        value.array = Vec::with_capacity(size);

        let mut counter = 0;

        for line in it {
            // Skip unnecessary size indicators during array construction.
            if !line.starts_with("$") {
                value.array.push(line);

                counter += 1;
            }

            // We already know the size of the recieved payload due to the
            // protocol. Therefore we can stop reading once we reached to expected
            // amount of parameters.
            if size == counter {
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
    fn parse_set() {
        let input = b"*3\r\n$3\r\nset\r\n$5\r\nadmin\r\n$5\r\nandre";
        let expected: Vec<String> =
            vec!["set".to_string(), "admin".to_string(), "andre".to_string()];

        let parser = Parser {};
        let result = parser.parse(&input[..]);

        assert_eq!(Commands::ARRAY, result.typ);
        assert_eq!(expected, result.array);
    }

    #[test]
    fn parse_get() {
        let input = b"*2\r\n$3\r\nget\r\n$5\r\nadmin";
        let expected: Vec<String> = vec!["get".to_string(), "admin".to_string()];

        let parser = Parser {};
        let result = parser.parse(&input[..]);

        assert_eq!(Commands::ARRAY, result.typ);
        assert_eq!(expected, result.array);
    }

    #[test]
    fn parse_ping() {
        let input = b"*2\r\n$4\r\nping\r\n$5\r\nhello";
        let expected: Vec<String> = vec!["ping".to_string(), "hello".to_string()];

        let parser = Parser {};
        let result = parser.parse(&input[..]);

        assert_eq!(Commands::ARRAY, result.typ);
        assert_eq!(expected, result.array);
    }
}
