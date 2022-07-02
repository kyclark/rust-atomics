use std::rc::Rc;
use std::thread;

fn main() {
    let a = Rc::new([1, 2, 3]);
    let b = a.clone();
    thread::spawn(move || dbg!(b));
}
