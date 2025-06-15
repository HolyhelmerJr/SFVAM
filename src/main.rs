mod initialization_protocol;
mod file_manager;

use std::mem::uninitialized;
use std::io::Read;
use std::io::{BufRead, BufReader};
use ssh2::{Channel, Session};
use std::error::Error;
use crate::initialization_protocol::InitializationProtocol;

struct Current_setup{
    session: Session,
    file_path: String,
    text_manager: String,
    username: String,
    password: String,
    terminal: Channel
}

struct File_data{
    file_name: String,
    file_extension: String,
    file_path: String,
}

fn main() {
}
