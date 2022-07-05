use std::sync::atomic::{
    AtomicBool,
    Ordering::{Acquire, Release},
};

struct SpinLock {
    locked: AtomicBool,
}

pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl SpinLock {
    pub const fn new() -> Self {
        Self {
            locked: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) -> Guard<T> {
        while self.locked.swap(true, Acquire) {
            std::hint::spin_loop();
        }
        Guard { lock: self }
    }

    pub fn unlock(&self) {
        self.locked.store(false, Release);
    }
}

fn main() {
    println!("Hello, world!");
}
