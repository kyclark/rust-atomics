use std::{
    cell::UnsafeCell,
    sync::atomic::{
        AtomicBool,
        Ordering::{Acquire, Release},
    },
};

pub struct SpinLock<T> {
    value: UnsafeCell<T>,
    locked: AtomicBool,
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

impl<T> SpinLock<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            locked: AtomicBool::new(false),
        }
    }

    pub fn lock<'a>(&'a self) -> &'a mut T {
        while self.locked.swap(true, Acquire) {
            std::hint::spin_loop();
        }
        unsafe { &mut &self.value.get() }
    }

    /// Safety: The &mut T from lock() from be gone!
    /// (And no cheating by keeping reference to fields of T
    pub unsafe fn unlock(&self) {
        self.locked.store(false, Release);
    }
}

fn main() {
    println!("Hello, world!");
}
