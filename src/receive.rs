use std::ffi::CString;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex, Condvar};
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

/// Structure for dispersing messages from receiving thread.
struct Mailbox {
    /// Optional mailbox contents.
    contents: Mutex<Option<String>>,
    /// Raise the flag to allow emptying the mailbox.
    flag: Condvar,
}

impl Mailbox {
    /// Make a new mailbox.
    fn new() -> Self {
        Mailbox {
            contents: Mutex::new(None),
            flag: Condvar::new(),
        }
    }

    /// Put a message in the mailbox.
    fn put(&self, message: String) {
        let mut guard = self.contents.lock().unwrap();
        *guard = Some(message);
        self.flag.notify_one();
    }

    /// Get a message out of the mailbox.
    fn get(&self) -> String {
        loop {
            let guard = self.contents.lock().unwrap();
            let mut guard = self.flag.wait(guard).unwrap();
            if let Some(message) = guard.take() {
                return message;
            }
            // If there is a spurious wakeup, try again.
        }
    }
}

/// Start a receiver that will update the "current" message as
/// new messages are received. Returns a `Mailbox` that can
/// be used to retrieve the current message.
fn start_receiver() -> Arc<Mailbox> {
    let s = UdpSocket::bind("localhost:29001").unwrap();
    let mailbox = Arc::new(Mailbox::new());
    let thread_mailbox = Arc::clone(&mailbox);
    let _ = thread::spawn(move || loop {
        let message = read(&s);
        thread_mailbox.put(message);
    });
    mailbox
}

/// Start the receive thread, then loop printing the current
/// message every 100 ms.
fn main() {
    let mailbox = start_receiver();
    let sleep_time = time::Duration::from_millis(100);
    loop {
        println!("{}", mailbox.get());
        thread::sleep(sleep_time);
    }
}
