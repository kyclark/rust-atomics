use std::{
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
    thread,
    time::Duration,
};

fn main() {
    let num_done = AtomicUsize::new(0);
    let main_thread = thread::current();

    thread::scope(|s| {
        // background
        s.spawn(|| {
            for i in 0..100 {
                process_item(i);
                num_done.store(i + 1, Relaxed);
                main_thread.unpark(); // Wake up!
            }
        });

        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }
            println!("Working.. {n}/100 done");
            thread::park_timeout(Duration::from_secs(1));
        }
    });

    println!("Done!");
}

fn process_item(_i: usize) {}
