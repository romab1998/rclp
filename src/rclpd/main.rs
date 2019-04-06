use std::fs::{File};
use std::io::{BufReader, Write, Read};
use rodio::Sink;
use rclplib::{Message, MessageType, get_rclp_dir, get_address_file_path};
use bincode;
use std::net::{TcpListener};

fn handle_message(msg: &Message, sink: &Sink) {
    match msg.message_type {
        MessageType::AddTrack => {
            let file = File::open(msg.data.as_str()).unwrap();
            let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
            sink.append(source);
            sink.play()
        }
        MessageType::Exit => {
            println!("Exiting");
            let path = get_address_file_path();
            println!("{:?}", path);
            std::fs::remove_file(path).unwrap();
            std::process::exit(0);
        }
        _ => {}
    }
}

fn bind() -> TcpListener {
    let mut start_port = 40000;
    loop {
        let try_bind = TcpListener::bind(format!("127.0.0.1:{}", start_port));
        match try_bind {
            Ok(val) => {
                return val;
            }
            Err(_) => { start_port += 1 }
        }
    }
}

fn main() {
    let path = get_rclp_dir();
    if !path.exists() {
        std::fs::create_dir(path.clone()).unwrap();
    }
    let mut file = File::create(get_address_file_path().as_path()).unwrap();
    let listener = bind();
    file.write(format!("{}", listener.local_addr().unwrap()).as_bytes()).unwrap();

    let device = rodio::default_output_device().unwrap();
    println!("default: {}", device.name());
    for device in rodio::devices() {
        println!("{}", device.name())
    }
    let sink = Sink::new(&device);
    for stream in listener.incoming() {
        let mut raw_data: Vec<u8> = Vec::new();
        let mut tcp_stream = stream.unwrap();
        tcp_stream.read_to_end(&mut raw_data).unwrap();
        println!("{:?}", raw_data);
        tcp_stream.write(bincode::serialize(
            &Message{message_type:MessageType::ReceivedSuccess, data: "".to_string()}
        ).unwrap().as_ref()).unwrap();
        let data = bincode::deserialize(&raw_data).unwrap();
        println!("{:?}", data);
        handle_message(&data, &sink);
    }
}