use std::rc::Rc;
use std::thread;

fn main() {
    let a = Rc::new(123);
    thread::spawn(move || {
        dbg!(a);
    });
}
