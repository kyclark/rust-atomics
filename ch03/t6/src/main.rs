use std::{
    sync::atomic::{
        AtomicBool,
        Ordering::{Acquire, Release},
    },
    thread,
    time::Duration,
};

static mut DATA: u64 = 0;
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    thread::spawn(|| {
        unsafe { DATA = 123 };
        READY.store(true, Release); // Everything before this
    });

    while !READY.load(Acquire) {
        thread::sleep(Duration::from_millis(100));
        println!("waiting...")
    }
    println!("{}", unsafe { DATA });
}
