use std::{
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
    thread,
    time::Duration,
};

fn main() {
    let num_done = &AtomicUsize::new(0);

    thread::scope(|s| {
        // background
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    process_item(t * 25 + i);
                    num_done.fetch_add(1, Relaxed);
                }
            });
        }

        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }
            println!("Working.. {n}/100 done");
            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("Done!");
}

fn process_item(_i: usize) {}
