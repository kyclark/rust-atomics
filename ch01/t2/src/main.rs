use std::thread;

fn main() {
    let numbers = vec![1, 2, 3];

    thread::spawn(|| {
        for n in numbers {
            println!("{n}");
        }
    });
}
