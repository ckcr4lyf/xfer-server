use std::fs::File;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str;
use std::env;

//read first byte to determine length of filename
//parse this into an int (x). read next x bytes into a vector(?) - made using vec! 
//this is out filename
//read now till end of stream, appending to a new, fresh vector
//when done, write this vector to the file
//done.

fn handle_client(mut stream: TcpStream){

    let mut binary = Vec::new();

    let mut size_header = [0 as u8; 2]; //First two bytes are size of filename. (NOT THE FILE)
    match stream.read_exact(&mut size_header) {
        Ok(_) => {
            println!("Legit");
        },
        Err(_) => {
            println!("We fucked up");
            return;
        }
    }

    let size = u16::from_be_bytes(size_header);
    
    let mut filename = vec![0u8; size as usize]; //Dynamic sized filename
    match stream.read_exact(&mut filename) {
        Ok(_) => {

        },
        Err(_) => {
            println!("Failed to receive filename");
            return;
        }
    }

    let mut f2 = File::create(str::from_utf8(&filename).unwrap()).unwrap();

    let mut data = [0 as u8; 1024]; // using 50 byte buffer at a time
    'damn: while match stream.read(&mut data) {
        Ok(size) => {
            if size == 0 {
                break 'damn; //No more data; EOF
            }

            // println!("{:?}", size);
            binary.extend_from_slice(&data[0..size]); //Add the binary data to our vector.
            true
        },

        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}

    println!("Exited while");
    // let mut file = File::create("data.bin").unwrap();
    f2.write(&binary).unwrap(); //Write data to the file
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: ./xfer-server [port]");
        return;
    }

    let mut listening = String::new();
    let port = &args[1];
    listening.push_str("0.0.0.0:");
    listening.push_str(port);

    let listener =  match TcpListener::bind(listening) {
        Ok(listener) => listener,
        Err(_) => {
            println!("Failed to bind to the port.");
            return;
        }
    };

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection from {}", stream.peer_addr().unwrap());
                handle_client(stream);
            }
            Err(_) => {
                println!("fux");
            }
        }
    }
}
