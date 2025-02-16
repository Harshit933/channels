## One-shot thread based channels written in rust

**Q) What are channels?**

Channels are a mean to transport a message across
threads safely such that no unambiguos activity takes
place.

**Q) What are one-shot channel?**

One-shot channels are channels only but are meant to 
be used only once for sending/receiving.

This is done for learning purpose only.

**Usage**
```rust
Channels::new();
channels.split();
```
The channels are purely thread safe because of the power of
different memory ordering that rust provides by default.
