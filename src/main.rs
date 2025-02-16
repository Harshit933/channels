use std::thread;

mod channel;

pub fn main() {
    let mut channel = channel::Channel::new();
    thread::scope(|s| {
        let (sender, receiver) = channel.split();
        s.spawn(move || {
            sender.send("Hello from scoped thread!");
        });
        assert_eq!(receiver.receive(), "Hello from scoped thread!");
    })
}
