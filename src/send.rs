use std::ffi::CString;
use std::io;
use std::net::UdpSocket;

///! Send side of lossy echo demo. Together with the receive
///! side, this code demonstrates dropping "late" UDP
///! messages.

/// Get a UDP socket bound to the first available UDP port
/// above 4095 as a sending address.
/// 
/// This is seriously gross.  See
/// [this issue](https://github.com/rust-lang-nursery/rust-cookbook/issues/500)
/// for the status of something better as part of `std::net`.
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

/// Send tick count messages as fast as possible. Messages
/// are sent as C strings. Do not recommend doing this over
/// the Internet, as it can send quite quickly.
fn main() {
    let s = find_port("localhost").unwrap();
    s.connect("localhost:29001").unwrap();
    for tick in 0u64.. {
        let message = CString::new(tick.to_string()).unwrap();
        let message = message.into_bytes();
        s.send(&message).unwrap();
    }
}
