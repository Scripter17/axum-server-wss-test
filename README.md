An example repository for [tokio-tungstenite issue 373](https://github.com/snapview/tokio-tungstenite/issues/373)

How to use:

1. Install `tls-ca.crt`

2. `cargo run --bin raw-server -r -- --tls`

3. `cargo build --bin client -r; while target/release/client --tls; do echo -n; done`

4. Wait for it to crash. You may need to add extra load to your CPU.

5. `cargo build --bin client-sync -r; while target/release/client-sync --tls; do echo -n; done`

6. Notice that it seems to never crash?

I first noticed this issue as arising in [Axum Server](https://github.com/programatik29/axum-server/issues/182), but I seem to have tracked it down to being an issue in either `tokio-tungstenite` or `tokio-rustls`.

The crash happens when the client tries to read the close message at the end of the stream and instead gets an `Protocol(ResetWithoutClosingHandshake)` error.

Interestingly, there doesn't seem to ever be any data loss. All prior messages are received correctly.

Also interestingly, whether or not this happens seems to be completely random and somehow affected by CPU usage, as shown by my laptop being able to hit this consistently after a few minutes while my more powerful desktop needs to also do a bunch of other stuff to hit it. Maybe it's a race condition?
