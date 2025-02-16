use std::{
    cell::UnsafeCell, marker::PhantomData, mem::MaybeUninit, sync::atomic::AtomicBool, thread,
};

pub struct Channel<T> {
    data: UnsafeCell<MaybeUninit<T>>,
    item_ready: AtomicBool,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            data: UnsafeCell::new(MaybeUninit::uninit()),
            item_ready: AtomicBool::new(false),
        }
    }

    // Gets the sender and receiver from the self
    // This is used to guarantee that the sender and
    // receiver lifetime should be in-sync with the
    // channel ones
    pub fn split<'a>(&'a mut self) -> (Sender<'a, T>, Receiver<'a, T>) {
        *self = Self::new();
        (
            Sender {
                channel: self,
                receiving_thread: std::thread::current(),
            },
            Receiver {
                channel: self,
                _no_data: PhantomData,
            },
        )
    }
}
pub struct Sender<'a, T> {
    channel: &'a Channel<T>,
    receiving_thread: std::thread::Thread, // This would be holding the thread which needs to be unparked
}

impl<T> Sender<'_, T> {
    // Takes self by value as we don't need some
    // other object calling this function
    pub fn send(self, message: T) {
        unsafe {
            (*self.channel.data.get()).write(message);
        }
        self.channel
            .item_ready
            .store(true, std::sync::atomic::Ordering::Release);
        // Unpark the waiting thread
        self.receiving_thread.unpark();
    }
}

pub struct Receiver<'a, T> {
    channel: &'a Channel<T>,
    _no_data: PhantomData<*const ()>, // This is here so that the whole struct could not be send into another thread
}

impl<T> Receiver<'_, T> {
    pub fn is_ready(&self) -> bool {
        self.channel
            .item_ready
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn receive(self) -> T {
        if !self
            .channel
            .item_ready
            .swap(false, std::sync::atomic::Ordering::Acquire)
        {
            thread::park();
        }
        unsafe { (*self.channel.data.get()).assume_init_read() }
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.item_ready.get_mut() {
            unsafe {
                self.data.get_mut().assume_init_drop();
            }
        }
    }
}
