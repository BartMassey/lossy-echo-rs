# lossy-echo: Demonstration of dropping "late" UDP Packets.
Bart Massey

This code is a thrown-together demo of dropping "late" UDP
packets: packets that arrived while previous packet
processing was in-flight and are now stale. Processing time
is simulated with a long sleep.

This demo was inspired by a
[Reddit question](https://www.reddit.com/r/learnrust/comments/gsypnz/unbuffered_streams/).

# Usage

Open two terminals. On the first, say `cargo run --bin
receive`. On the second say `cargo run --bin send`. Watch
the interval between "processed" packets in the `receive`
window: it should be quite large.

# License

This work is made available under the "MIT License". Please
see the file `LICENSE` in this distribution for license
terms.
