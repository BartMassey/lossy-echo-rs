use std::ffi::CString;
use std::net::UdpSocket;

fn main() {
    let s = UdpSocket::bind("localhost:29001").unwrap();
    let mut buf = [0; 4096];
    let (nread, _where) = s.recv_from(&mut buf).unwrap();
    if nread > buf.len() {
        panic!("buffer overflow: {} > {}", nread, buf.len());
    }
    let message = CString::new(&buf[..nread]).unwrap();
    let message = message.into_string().unwrap();
    println!("{}", message);
}
