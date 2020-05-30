use std::ffi::CString;
use std::net::UdpSocket;
use std::{thread, time};

fn main() {
    let s = UdpSocket::bind("localhost:29001").unwrap();
    let sleep_time = time::Duration::from_millis(100);
    let mut buf = [0; 4096];
    loop {
        let (nread, _where) = s.recv_from(&mut buf).unwrap();
        if nread > buf.len() {
            panic!("buffer overflow: {} > {}", nread, buf.len());
        }
        let message = CString::new(&buf[..nread]).unwrap();
        let message = message.into_string().unwrap();
        println!("{}", message);
        thread::sleep(sleep_time);
    }
}
