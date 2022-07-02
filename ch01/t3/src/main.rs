use std::thread;

fn main() {
    // let numbers = Vec::from_iter(0..=1000);
    let numbers: Vec<usize> = vec![];

    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.into_iter().sum::<usize>();
        sum / len
    });

    let average = t.join().unwrap();

    println!("average: {average}");
}
