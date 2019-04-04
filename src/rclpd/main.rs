use std::fs::{File, remove_file};
use std::io::{BufReader, Write};
use rodio::{Sink};
use ipc_channel::ipc::{IpcOneShotServer};
use rclplib::{Message, MessageType};
use std::path::Path;
use daemonize::Daemonize;

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
            std::process::exit(0);
        }
    }
}


fn main() {
    let stdout = File::create("/tmp/daemon.out").unwrap();
    let stderr = File::create("/tmp/daemon.err").unwrap();
    let path = "/tmp/hello";

    let daemonize = Daemonize::new()
        .pid_file("/tmp/test.pid")
        .stdout(stdout)
        .stderr(stderr);

    match daemonize.start() {
        Ok(_) => println!("Success, daemonized"),
        Err(e) => eprintln!("Error, {}", e),
    }

    let device = rodio::default_input_device().unwrap();
    let sink = Sink::new(&device);
    loop {
        let name_path = Path::new(path);
        if name_path.exists() {
            remove_file(name_path).unwrap();
        }
        let mut name_file = File::create(path).unwrap();
        let (serv, name): (IpcOneShotServer<Message>, String) = IpcOneShotServer::new().unwrap();
        name_file.write(name.as_bytes()).unwrap();
        let (_, data) = serv.accept().unwrap();
        println!("{:?}", data);
        handle_message(&data, &sink);
    }
}