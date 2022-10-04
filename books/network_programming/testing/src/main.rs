mod here_io;
mod here_c;

use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write, Result};
use std::thread;

fn main() {
    if std::env::args().len() != 2 {
        std::process::exit(1);
    }

    let port = std::env::args().nth(1).expect("expecting port as first and only argument");

    server(port.parse::<u16>().expect("expecting port to be a number"));

}

fn server(port: u16) {
    let listener: TcpListener = TcpListener::bind(("0.0.0.0", port)).expect("couldn't bind to port");

    for stream in listener.incoming() {
        match stream {
            Err(e) => eprintln!("{}", e),
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap()
                });
            },
        }
    }
}

fn handle_client(mut stream: TcpStream) -> Result<()> {
    println!("connecting with {}", stream.peer_addr()?);

    loop {
        let mut buf = [0; 512];
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 { return Ok(()); }
        stream.write(&buf)?;
    }
}

