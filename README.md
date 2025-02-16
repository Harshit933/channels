## One-shot thread based channels written in rust

This is done for learning purpose only. You can make a thread
using `Channels::new()`, then split the (sender, receiver) using
`channels.split()`.

The channels are purely thread safe because of the power of
different memory ordering that rust provides by default.
