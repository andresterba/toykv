pub mod parser;

mod commands;
mod store;

use std::net::TcpListener;

use store::Store;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    let mut store = store::MapStore::new();

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

// fn read_line(mut stream: TcpStream) {
//     let mut buf_reader = BufReader::new(&mut stream);
//     // let it = buf_reader.lines().map(|line| line.unwrap());
//     let mut read_line = String::new();
//     buf_reader.read_line(&mut read_line).unwrap();

//     println!("{read_line:?}");

// }

// fn handle_connection(mut stream: TcpStream) {
//     let buf_reader = BufReader::new(&mut stream);

//     // functional
//     // let mut array_size: Option<usize> = None;
//     // let mut counter = 0;

//     // buf_reader
//     //     .lines()
//     //     .map(|line| line.unwrap())
//     //     .take_while(|line| {
//     //          if array_size.is_none() {
//     //             let (_, size) = line.split_at(1);
//     //             array_size = Some(size.parse::<usize>().unwrap() * 2)
//     //          } else {

//     //             counter += 1;
//     //          }

//     //         array_size.unwrap() == counter
//     //     }).for_each(|_| {});

//     // Iterator with loop
//     let mut array_size: Option<usize> = None;
//     let mut counter = 0;
//     let mut value = Value {
//         typ: Commands::UNKNOWN,
//         array: vec![],
//     };

//     let it = buf_reader.lines().map(|line| line.unwrap());

//     for line in it {
//         if array_size.is_none() {
//             // determine command type
//             let (cmd_type, size) = line.split_at(1);

//             value.typ = get_command_type(cmd_type);

//             // we double the size here is each value is always in the format:
//             // $5 hello
//             // $5 -> string with size of 5 will follow
//             // hello -> the actual value
//             let size = size.parse::<usize>().unwrap() * 2;

//             array_size = Some(size);
//             value.array = Vec::with_capacity(size);
//         } else {
//             counter += 1;

//             value.array.push(line);
//         }

//         if array_size.unwrap() == counter {
//             break;
//         }
//     }

//     println!("{value:?}");

//     // non functional

//     // let mut iter = buf_reader.lines().into_iter();
//     // let first_line = iter.next().unwrap().unwrap();
//     // let( _, size) = first_line.split_at(1);

//     // println!("{first_line:?}");
//     // // println!("{size:?}");

//     // let intSize = size.parse::<usize>().unwrap();
//     // let mut cmd = 0;

//     // // intSize*2 because each line always represents a type description followed by the actual type value.
//     // while cmd < intSize*2  {
//     //     let line = iter.next().unwrap().unwrap();
//     //     println!("{line:?}");
//     //     cmd += 1;
//     // }

//     let mut value_to_write = "+OK\r\n";
//     let mut prefix = String::from("+");

//     if value.array[1] == "ping" {
//         prefix.push_str(value.array[3].as_str());
//         prefix.push_str("\r\n");

//         value_to_write = prefix.as_str();
//     }

//     stream.write_all(value_to_write.as_bytes()).unwrap();
// }
