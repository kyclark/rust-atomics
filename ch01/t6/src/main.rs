use std::thread;

fn main() {
    // let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));
    let x: [i32; 3] = *Box::leak(Box::new([1, 2, 3]));

    thread::spawn(move || dbg!(x));
    thread::spawn(move || dbg!(x));
}
