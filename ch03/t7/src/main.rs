use std::{
    sync::atomic::{
        AtomicBool,
        Ordering::{Acquire, Relaxed, Release},
    },
    thread,
};

static mut DATA: String = String::new();
static LOCKED: AtomicBool = AtomicBool::new(false);

fn f() {
    //if LOCKED
    //    .compare_exchange(false, true, Acquire, Relaxed)
    //    .is_ok()
    //{

    if LOCKED.swap(true, Acquire) == false {
        // Another way to write
        // Safety: We hold the exclusive lock
        unsafe { DATA.push('!') };
        LOCKED.store(false, Release);
    }
}

fn main() {
    thread::scope(|s| {
        for _ in 0..100 {
            s.spawn(f);
        }
    });

    println!("{}", unsafe { DATA.to_string() });
}
