use std::thread;

fn main() {
    let X: [i32; 3] = [1, 2, 3];
    thread::spawn(|| dbg!(&X));
    thread::spawn(|| dbg!(&X));
}
