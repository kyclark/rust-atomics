use std::{
    sync::atomic::{AtomicBool, Ordering::Relaxed},
    thread,
};

fn main() {
    static STOP: AtomicBool = AtomicBool::new(false);

    // Spawn a thread to do the work
    let background_thread = thread::spawn(|| {
        while !STOP.load(Relaxed) {
            some_work();
        }
    });

    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("commands: help, stop"),
            "stop" => break,
            cmd => println!("unknown command: {cmd:?}"),
        }
    }

    STOP.store(true, Relaxed);

    background_thread.join().unwrap();
}

fn some_work() {}
