fn main() {
    let a = [123, 456, 789];
    let index = 3;

    match index {
        0 => x(),
        1 => y(),
        _ => z(index),
    }

    let b = unsafe { a.get_unchecked(index) };
    dbg!(b);
}

fn x() {}
fn y() {}
fn z(_index: usize) {}
