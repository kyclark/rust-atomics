use std::{
    array,
    sync::atomic::{
        fence, AtomicBool,
        Ordering::{Acquire, Relaxed, Release},
    },
    thread,
    time::Duration,
};

static mut DATA: [u64; 10] = [0; 10];

const ATOMIC_FALSE: AtomicBool = AtomicBool::new(false);
static READY: [AtomicBool; 10] = [ATOMIC_FALSE; 10];

fn main() {
    for i in 0..10 {
        thread::spawn(move || {
            let data = some_calculation(i);
            unsafe { DATA[i] = data };
            READY[i].store(true, Release);
        });
    }

    thread::sleep(Duration::from_millis(500));

    let ready: [bool; 10] = array::from_fn(|i| READY[i].load(Relaxed));
    if ready.contains(&true) {
        fence(Acquire);
        for i in 0..10 {
            if ready[i] {
                println!("data = {}", unsafe { DATA[i] });
            }
        }
    }
}

fn some_calculation(val: usize) -> u64 {
    2 * val as u64
}
