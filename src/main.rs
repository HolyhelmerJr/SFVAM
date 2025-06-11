use std::fs::File;
use std::mem::uninitialized;
use std::io::Read;
use std::path::Path;
use openssh::{Session, KnownHosts};
use std::error::Error;

fn main() {
    let ssh_name: String = initlizeOptions();


    
}

fn initlizeOptions() -> String{
    let mut options = File::open("options.txt").unwrap();
    let mut ssh_name = String::new();

    options.read_to_string(&mut ssh_name).unwrap();
    return ssh_name;
}

async fn create_connection(ssh_name: &String) -> Session {
}
