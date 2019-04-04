use std::fs::File;
use std::io::Read;
use ipc_channel::ipc::IpcSender;
use rclplib::{Message, MessageType};
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <2 {
        panic!()
    }
    let filename = &args[1];

    let mut file = File::open("/tmp/hello").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let sender = IpcSender::connect(data).unwrap();

    if filename == "exit" {
        sender.send(Message{
            message_type: MessageType::Exit,
            data: "".to_string()
        }).unwrap();
        std::process::exit(0);
    }

    println!("input: {}", filename);

    let abs_path = fs::canonicalize(PathBuf::from(filename)).unwrap();

    println!("input: {:?}", abs_path);


    sender.send(Message {
        message_type: MessageType::AddTrack,
        data: abs_path.to_str().unwrap().to_string()
    }).unwrap();
}
