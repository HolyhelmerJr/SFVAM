use crate::{Current_setup, File_data};
use ssh2::Session;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::path::Path;

pub struct InitializationProtocol;

//Reads the options file and connects to the ssh 
impl InitializationProtocol {
    pub fn initlize_options() -> Current_setup {
        let options = File::open("options.txt").unwrap();
        let mut reader = BufReader::new(options);

        let ssh_ip = Self::read_next_line(&mut reader);
        let username = Self::read_next_line(&mut reader);
        let file_path = Self::read_next_line(&mut reader);
        let password = Self::read_next_line(&mut reader);
        let text_manager = Self::read_next_line(&mut reader);

        let tcp = TcpStream::connect(&*ssh_ip).expect("Failed to connect to server.");
        let mut session = Session::new().expect("Failed to initialize session.");
        session.set_tcp_stream(tcp);
        session.handshake().expect("Failed to handshake.");
        let mut terminal = session.channel_session().expect("Failed to access the terminal");

        terminal.exec("cd {file_path}").expect("Failed to jump to main folder");
        session.userauth_password(&*username, &*password).unwrap();
        assert!(session.authenticated(), "Failed to authenticate with session.");

        Current_setup {
            session,
            file_path,
            text_manager,
            username,
            password,
            terminal,
        }
    }


    //Reads the next line from a buf reader and returns it 
    fn read_next_line(buf_reader: &mut BufReader<File>) -> String {
        let line = match buf_reader.lines().next() {
            Some(Ok(line)) => line,
            Some(Err(e)) => panic!("Error reading line{}", e),
            None => panic!("Error reading line")
        };
        return line
    }
    
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