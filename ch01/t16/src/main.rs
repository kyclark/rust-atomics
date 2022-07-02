use std::cell::Cell;

fn main() {
    let a = Cell::new(1);
    let b = Cell::new(2);
    f(&a, &b);
}

fn f(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();
    if before != after {
        println!("Hey!"); // never happens
    }
}
