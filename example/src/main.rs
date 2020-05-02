use c_closures::{rebind_closure_ref, Closure};

#[allow(dead_code, non_snake_case)]
mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

fn main() {
    let mut x = 0;
    let mut c = Closure::fn_mut(move |_: ()| {
        x += 1;
        println!("I've been called {} times", x);
    });
    let c = rebind_closure_ref!(ffi::Closure, &mut c);
    for i in 1..=30 {
        println!("Considered calling closure {} times", i);
        unsafe {
            ffi::maybe_call(c);
        }
    }
}
