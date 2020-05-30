# lossy-echo: Demonstration of dropping "late" UDP Packets.
Bart Massey

This code is a thrown-together demo of dropping "late" UDP
packets: packets that arrived while previous packet
processing was in-flight and are now stale. Processing time
is simulated with a sleep (as usual).

This demo was inspired by a
[Reddit question](https://www.reddit.com/r/learnrust/comments/gsypnz/unbuffered_streams/).

# Usage

Open two terminals. On the first, say `cargo run --bin
receive`. On the second say `cargo run --bin send`.

The consumer will block waiting for the first message to be
sent. The first message printed once send starts will most
likely be greater than 0, as the receive thread is racing
with the consumer.  The interval between "processed" packets
in the `receive` window should be quite large. When the
sender is interrupted, the consumer should immediately
block. Restart the sender to send more messages.

# License

This work is made available under the "MIT License". Please
see the file `LICENSE` in this distribution for license
terms.
