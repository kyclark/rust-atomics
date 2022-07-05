use std::{
    sync::atomic::{AtomicI32, Ordering::Relaxed},
    thread,
};

static X: AtomicI32 = AtomicI32::new(0);
static Y: AtomicI32 = AtomicI32::new(0);

fn main() {
    X.store(10, Relaxed);
    Y.store(100, Relaxed);
    assert_eq!(Y.load(Relaxed), 100);
    let t = thread::spawn(f);
    assert!(Y.fetch_add(50, Relaxed) >= 100); // could be 100 or 150
    assert_eq!(Y.fetch_add(50, Relaxed), 100); // could be 100 or 150
    X.store(20, Relaxed);
    t.join().unwrap();
    X.store(-30, Relaxed);
    assert_eq!(Y.load(Relaxed), 200);
}

fn f() {
    assert!(X.load(Relaxed) >= 10); // could be 10 or 20
    assert!(Y.fetch_add(50, Relaxed) >= 100); // could be 100 or 150
}
