use std::ffi::CString;
use std::io;
use std::net::UdpSocket;

// XXX This is seriously gross.
// https://github.com/rust-lang-nursery/rust-cookbook/issues/500
fn find_port(addr: &str) -> io::Result<UdpSocket> {
    for p in 4096..0xffff {
        match UdpSocket::bind((addr, p)) {
            Ok(s) => return Ok(s),
            Err(e) => match e.kind() {
                io::ErrorKind::AddrInUse => (),
                _ => return Err(e),
            }
        }
    }
    Err(io::Error::new(
        io::ErrorKind::AddrNotAvailable,
        "no free port available",
    ))
}

fn main() {
    let s = find_port("localhost").unwrap();
    s.connect("localhost:29001").unwrap();
    let message = CString::new("ahoy ahoy!").unwrap();
    s.send(&message.into_bytes()).unwrap();
}
