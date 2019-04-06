use std::fs::File;
use std::io::Read;
use rclplib::{Message, MessageType, get_address_file_path};
use std::env;
use std::fs;
use std::path::PathBuf;
use bincode;
use std::net::TcpStream;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("no args provided");
        std::process::exit(1);
    }
    let filename = &args[1];

    let path = get_address_file_path();
    if !path.exists() {
        println!("it seems like rclpd not started");
        Command::new("rclpd").spawn().unwrap();
    }

    while !path.exists() {
        println!("waiting for start");
        std::thread::sleep(Duration::new(1, 0));
    }

    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    println!("{}", data);
    let mut stream = TcpStream::connect(data).unwrap();

    if filename == "exit" {
        stream.write(bincode::serialize(&Message {
            message_type: MessageType::Exit,
            data: "".to_string(),
        }).unwrap().as_ref()).unwrap();
        return;
    }

    println!("input: {}", filename);

    let abs_path = fs::canonicalize(PathBuf::from(filename)).unwrap();

    println!("input: {:?}", abs_path);


    stream.write(bincode::serialize(&Message {
        message_type: MessageType::AddTrack,
        data: abs_path.to_str().unwrap().to_string(),
    }).unwrap().as_ref()).unwrap();
}
