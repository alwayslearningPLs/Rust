
#[test]
fn endian() {
    let port: u16 = 80u16.swap_bytes();
    let p: *const u16 = &port as *const u16;

    unsafe {
        let addr = libc::addrinfo {
            ai_family: libc::AF_INET,
            ai_protocol: 0,
            ai_socktype: libc::SOCK_STREAM,
        };
    }
}
