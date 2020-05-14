#[allow(dead_code, non_snake_case, non_camel_case_types)]
mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

fn main() {
    let mut x = 0;
    let mut c = ffi::VoidVoidClosure::fn_mut(move || {
        x += 1;
        println!("I've been called {} times", x);
    });
    for i in 1..=30 {
        println!("Considered calling closure {} times", i);
        unsafe {
            ffi::maybe_call(&mut c);
        }
    }
    let mut c_int = ffi::IntIntClosure::fn_mut(move |x| x * 2);
    println!("5 * 2 is {}", unsafe {
        ffi::IntInt_closure_call(&mut c_int, 5)
    });
    let mut c_int_int = ffi::IntIntIntClosure::fn_mut(move |x, y| x - y);
    println!("4 - 3 is {}", unsafe {
        ffi::IntIntInt_closure_call(&mut c_int_int, 4, 3)
    });
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::ffi::*;

    #[test]
    fn fn_not_mut() {
        let y = 4;
        let mut closure = IntIntClosure::fn_not_mut(move |x: i32| x + x + y);
        unsafe {
            let ret = IntInt_closure_call(&mut closure, 2);
            assert_eq!(ret, 8);
            IntInt_closure_release_return_value(&mut closure, ret);
            IntInt_closure_release(&mut closure);
        }
    }

    #[test]
    fn fn_mut() {
        let mut y = 4;
        let mut closure = IntIntClosure::fn_mut(move |x: i32| {
            y *= 2;
            x + x + y
        });
        unsafe {
            let ret = IntInt_closure_call(&mut closure, 2);
            assert_eq!(ret, 12);
            IntInt_closure_release_return_value(&mut closure, ret);

            let ret = IntInt_closure_call(&mut closure, 2);
            assert_eq!(ret, 20);
            IntInt_closure_release_return_value(&mut closure, ret);
            IntInt_closure_release(&mut closure);
        }
    }

    #[test]
    fn fn_once() {
        let mut y = 4;
        let mut closure = IntIntClosure::fn_once(move |x: i32| {
            y *= 2;
            x + x + y
        });
        unsafe {
            let ret = IntInt_closure_call(&mut closure, 2);
            assert_eq!(ret, 12);
            IntInt_closure_release_return_value(&mut closure, ret);

            // I'd love to verify that a subsequent call aborts, but it's non-trivial
            // to put that into a test suite. We'll address this if it ever becomes a problem
            // that this testing isn't done.
            IntInt_closure_release(&mut closure);
        }
    }

    #[test]
    fn fn_drop_test() {
        let value = Arc::new(());
        let value_clone = value.clone();
        let mut closure =
            VoidVoidClosure::fn_not_mut(move || println!("{}", Arc::strong_count(&value_clone)));
        unsafe {
            assert_eq!(Arc::strong_count(&value), 2);
            VoidVoid_closure_release(&mut closure);
            assert_eq!(Arc::strong_count(&value), 1);
        }
    }

    #[test]
    fn fn_noop() {
        let mut closure = VoidVoidClosure::new_noop();
        unsafe {
            VoidVoid_closure_call(&mut closure);
        }
    }
}
