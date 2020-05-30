use std::ffi::CString;
use std::net::UdpSocket;
use std::sync::{Arc, RwLock};
use std::{thread, time};

///! Receive side of lossy echo demo. Together with the send
///! side, this code demonstrates dropping "late" UDP
///! messages.

/// Read and return a message from the socket. Messages are
/// received as C strings and converted to Rust strings here.
fn read(s: &UdpSocket) -> String {
    let mut buf = [0; 4096];
    let (nread, _where) = s.recv_from(&mut buf).unwrap();
    if nread > buf.len() {
        panic!("buffer overflow: {} > {}", nread, buf.len());
    }
    let message = CString::new(&buf[..nread]).unwrap();
    message.into_string().unwrap()
}

/// Start a reader that will update the "current" message as
/// new messages are received. Returns a `RwLock` that can
/// be used to retrieve the current message.
///
/// This function *will block* until the first message is
/// received. (I don't see any way to write-lock the message
/// lock until the child receives the first message.)
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

/// Start the receive thread, then loop printing the current
/// message every 100 ms.
fn main() {
    let message_lock = start_reader();
    let sleep_time = time::Duration::from_millis(100);
    loop {
        println!("{}", message_lock.read().unwrap());
        thread::sleep(sleep_time);
    }
}
