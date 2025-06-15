use crate::{Current_setup, File_data};
use ssh2::Session;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::path::Path;



pub struct file_manager; 

impl file_manager {
    
    
    
    //Creates an instance of the struct file_data 
    fn create_file_data_from_terminal_ls_1a(file_name: String, file_path: String) -> File_data {
        let path = Path::new(&file_name);
        let file_extension = path.extension().and_then(|e| e.to_str()).map(|s| s.to_string()).unwrap_or_default();
        File_data{
            file_name,
            file_extension,
            file_path
        }
    }
}