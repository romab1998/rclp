use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    AddTrack, Exit, ReceivedSuccess, ReceivedError
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub message_type: MessageType,
    pub data: String
}

pub fn get_address_file_path() -> PathBuf {
    let mut path = get_rclp_dir();
    path.push("address");
    path
}

pub fn get_rclp_dir() -> PathBuf {
    let mut path = PathBuf::new();
    path.push(std::env::temp_dir());
    path.push(".rclpd");
    path
}