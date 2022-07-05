use std::{
    ptr,
    sync::atomic::{
        AtomicPtr,
        Ordering::{AcqRel, Acquire},
    },
};

#[derive(Debug)]
struct Data {
    id: usize,
}

fn main() {
    let data = get_data();
    println!("data = {}", data.id);
}

fn get_data() -> &'static Data {
    static PTR: AtomicPtr<Data> = AtomicPtr::new(ptr::null_mut());

    let mut p = PTR.load(Acquire);

    if p.is_null() {
        p = Box::into_raw(Box::new(generate_data()));

        if let Err(e) =
            PTR.compare_exchange(ptr::null_mut(), p, AcqRel, Acquire)
        {
            // Safety: p comes from Box::into_raw right above,
            // and wasn't share with any other thread
            drop(unsafe { Box::from_raw(p) });
            p = e;
        }
    }

    // Safety: p is not null and points to an initialized value
    unsafe { &*p }
}

fn generate_data() -> Data {
    Data { id: 0 }
}
