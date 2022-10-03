use std::net::{SocketAddr, ToSocketAddrs, IpAddr, Ipv4Addr, Ipv6Addr};
use std::io::{Read, Write};

fn main() {
    unsafe {
        let _port: &[i8] = &[0x38i8, 0x30i8, 0x38i8, 0x30i8, 0x00i8]; // ['8', '0', '8', '8', '\0'];
        let _fd: i32 = libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0);

        // libc::getaddrinfo(&0x00i8 as *const i8, port[0] as *const i8, &hints, &mut address_to_connect);
    }
    echo_server();
}

#[allow(dead_code)]
fn creating_sockets() {
    let _ = ("127.0.0.1".to_owned(), 8080u16).to_socket_addrs();
    let _ = ("127.0.0.1", 8080u16).to_socket_addrs();
    let _ = "127.0.0.1:8080".to_socket_addrs();
    let _ = String::from("127.0.0.1:8080").to_socket_addrs();

    // let _ = "127.0.0.1:8080".parse();
   
    let _ = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080u16);
    let _ = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 8080u16);

    // 2001:0db8:0000:0000:0000:8a2e:0370:7334
    let _ = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0x2001, 0x0db8, 0x0000, 0x0000, 0x0000, 0x8a2e, 0x0370, 0x7334)), 8080u16);
}

fn echo_server() {
    let listener: std::net::TcpListener = std::net::TcpListener::bind("127.0.0.1:8080").expect("couldn't bind to port 8080");

    for stream in listener.incoming() {
        match stream {
            Err(e) => eprintln!("{}", e),
            Ok(stream) => {
                std::thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|err| eprintln!("{:?}", err));
                });
            }
        }
    }
}

fn handle_client(mut stream: std::net::TcpStream) -> Result<(), std::io::Error> {
    println!("Handling client with IP: {:?}", stream.peer_addr()?);

    loop {
        let mut buf = [0; 512];
        let bytes_read: usize = stream.read(&mut buf)?;
        if bytes_read == 0 { return Ok(()); }
        if String::from_utf8_lossy(&buf[..bytes_read]).starts_with("bye") {
            stream.write("bye".as_bytes())?;
            return Ok(());
        }
        stream.write(&buf[..bytes_read])?;
    }
}
