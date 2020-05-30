use std::ffi::CString;
use std::net::UdpSocket;
use std::sync::{Arc, RwLock};
use std::{thread, time};

fn read(s: &UdpSocket) -> String {
    let mut buf = [0; 4096];
    let (nread, _where) = s.recv_from(&mut buf).unwrap();
    if nread > buf.len() {
        panic!("buffer overflow: {} > {}", nread, buf.len());
    }
    let message = CString::new(&buf[..nread]).unwrap();
    message.into_string().unwrap()
}

fn start_reader() -> Arc<RwLock<String>> {
    let s = UdpSocket::bind("localhost:29001").unwrap();
    let initial = read(&s);
    let message_lock = Arc::new(RwLock::new(initial));
    let thread_message_lock = Arc::clone(&message_lock);
    let _ = thread::spawn(move || {
        loop {
            let message = read(&s);
            let mut w = thread_message_lock.write().unwrap();
            *w = message;
        }
    });
    message_lock
}

fn main() {
    let message_lock = start_reader();
    let sleep_time = time::Duration::from_millis(100);
    loop {
        let message = message_lock.read().unwrap();
        println!("{}", message);
        drop(message);
        thread::sleep(sleep_time);
    }
}
