fn main() {
    let a = [123, 456, 789];
    let index = 10; // This is wrong
    let b = unsafe { a.get_unchecked(index) }; // What will happen?
    dbg!(b);
}
