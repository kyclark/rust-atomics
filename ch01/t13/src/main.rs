fn main() {
    let a = 1;
    let mut b = 2;
    f(&a, &mut b);
}

fn f(a: &i32, b: &mut i32) {
    let before = *a;
    *b += 1;
    let after = *a;
    if before != after {
        println!("Hey!"); // never happens
    }
}
