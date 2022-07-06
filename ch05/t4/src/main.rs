use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::atomic::{
        AtomicU8,
        Ordering::{Acquire, Relaxed, Release},
    },
    thread,
};

const EMPTY: u8 = 0;
const WRITING: u8 = 1;
const READY: u8 = 2;
const READING: u8 = 3;

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    state: AtomicU8,
}

unsafe impl<T: Send> Sync for Channel<T> {}

impl<T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            state: AtomicU8::new(EMPTY),
        }
    }

    pub fn send(&self, message: T) {
        if self
            .state
            .compare_exchange(EMPTY, WRITING, Relaxed, Relaxed)
            .is_err()
        {
            panic!("can't send more than one message!")
        }
        unsafe { (*self.message.get()).write(message) };
        self.state.store(READY, Release);
    }

    pub fn is_ready(&self) -> bool {
        self.state.load(Relaxed) == READY
    }

    pub fn receive(&self) -> T {
        if !self
            .state
            .compare_exchange(READY, READING, Acquire, Relaxed)
            .is_err()
        {
            panic!("No message avalable!")
        }

        unsafe { (*self.message.get()).assume_init_read() }
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.state.get_mut() == READY {
            unsafe { self.message.get_mut().assume_init_drop() }
        }
    }
}

fn main() {
    let channel = Channel::new();
    let t = thread::current();
    thread::scope(|s| {
        s.spawn(|| {
            channel.send("hello world!");
            t.unpark();
        });

        while !channel.is_ready() {
            thread::park();
        }

        assert_eq!(channel.receive(), "hello world!");
    });
}
