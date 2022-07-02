use std::cell::RefCell;

fn main() {
    let vals = RefCell::new(vec![1, 2, 3]);
    f(&vals);
    dbg!(vals);
}

fn f(v: &RefCell<Vec<i32>>) {
    v.borrow_mut().push(1);
}
