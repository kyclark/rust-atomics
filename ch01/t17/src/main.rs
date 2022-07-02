use std::cell::Cell;

fn main() {
    let vals = Cell::new(vec![1, 2, 3]);
    f(&vals);
}

fn f(v: &Cell<Vec<i32>>) {
    let mut v2 = v.take(); // Replaces the contents with an empty vec
    v2.push(1);
    v.set(v2); // put the modified vec back
}
