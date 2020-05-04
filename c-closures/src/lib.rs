//! # Purpose
//!
//! This crate is for producing Rust closures that can cross an FFI boundary with no generic types.
//! It provides support for any single argument signature, along with any return type, assuming
//! both have valid representations in C/C++ and Rust.
//!
//! [Here's an example.](https://github.com/Xaeroxe/c-closures-rs/tree/master/example)
//!
//! # Safety concerns
//!
//! Creating a `Closure` by itself can not cause undefined behavior, however the resulting
//! structure is extremely dangerous. The C/C++ code may not validate arguments
//! passed are of the correct type, which could lead to memory corruption and
//! segfaulting. `Closure` should never be an argument to a safe function, nor should it be
//! a public member of any structures passed into a safe function.
//!
//! # Usage in C/C++
//!
//! To use this with a C/C++ library you'll need to include the header provided in the repo,
//! `rust_closures.h`, then link to the assembly produced by `rust_closures.c`. If the C/C++ code
//! is being linked into a Rust binary depending on this crate, then you don't need to worry about
//! linking to `rust_closures.c`. Then you can accept the `Closure` type anywhere that you need to
//! accept arbitrary Rust code.
//!
//! # Limitations
//!
//! `Closure` can currently only accept a single argument, this can be worked around by making that argument
//! a C/C++ class/struct containing multiple fields. Additionally it is strongly recommended that all types
//! in the closure signature have a valid representation in C/C++ and Rust. Fat pointers are a common gotcha
//! in this respect, remember slices and string slices are not a single pointer value.
//!
//! This cannot be used to transfer ownership across FFI boundaries, as this crate cannot reasonably guarantee
//! both sides are using the same memory allocator, or dispose of the types in the same way. If such transfer
//! is required, you should copy the data into a new allocation, on the side of the FFI boundary it needs to live
//! on. The major exception to this is types with the `Copy` marker trait, which are trivially cloned and require
//! no disposal instructions.
//!
//! In order to achieve this in such a general manner this crate leans heavily on heap allocations. Arguments,
//! and return types are treated as data of arbitrary unknown length. If such heap allocations are unacceptable
//! for your use case, consider authoring a similar structure with specific known types and involving no indirection.
//!

#![allow(non_snake_case)]

use std::{ffi::c_void, mem::size_of, process::abort, ptr::null_mut};

use backtrace::Backtrace;
use log::error;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Send + Sync impl for Closure intentionally omitted. It's possible to create versions of these
// for which such an impl is correct, and it may even prove to be desirable, but this version is
// not provably Send + Sync as the closures it wraps can capture anything.

impl Closure {
    /// Transform an FnMut Rust closure into a structure you can pass into a C/C++ library.
    ///
    /// This structure currently assumes it will never be called in multiple threads
    /// simultaneously. If that guarantee cannot be upheld, then you should instead use `fn_not_mut`.
    ///
    /// ```
    /// # use c_closures::Closure;
    /// let mut y = 5;
    /// let _f = Closure::fn_mut(move |x: &i32| {
    ///     y *= 2;
    ///     *x * 2
    /// });
    /// ```
    pub fn fn_mut<Arg, Return, Function>(f: Function) -> Self
    where
        Arg: FromClosureArgPointer,
        Function: FnMut(Arg) -> Return + Send + 'static,
    {
        Self {
            data: Box::into_raw(Box::new(f)) as *mut c_void,
            function: Some(f_wrapper::<Arg, Return, Function>),
            delete_data: Some(delete_me::<Function>),
            delete_ret: Some(delete_me::<Return>),
        }
    }

    /// Transform an Fn Rust closure into a structure you can pass into a C/C++ library.
    ///
    /// This structure is safe to use in multiple threads simultaneously. If your usage is single
    /// threaded, consider `fn_mut` instead as it permits more robust closures.
    ///
    /// ```
    /// # use c_closures::Closure;
    /// let y = 5;
    /// let _f = Closure::fn_not_mut(move |x: &i32| {
    ///     *x * y
    /// });
    /// ```
    pub fn fn_not_mut<Arg, Return, Function>(f: Function) -> Self
    where
        Arg: FromClosureArgPointer,
        Function: Fn(Arg) -> Return + Send + 'static,
    {
        Self {
            data: Box::into_raw(Box::new(f)) as *mut c_void,
            function: Some(f_wrapper::<Arg, Return, Function>),
            delete_data: Some(delete_me::<Function>),
            delete_ret: Some(delete_me::<Return>),
        }
    }

    /// Transform an FnOnce Rust closure into a structure you can pass into a C/C++ library.
    ///
    /// This structure assumes it will only ever be called once. If you attempt to call it more than once
    /// the return value will be zeroed memory. If the return type does not consider zeroed memory to be a valid
    /// representation, then usage of the return type in this instance may result in undefined behavior.
    ///
    /// ```
    /// # use c_closures::Closure;
    /// let values = vec![String::from("1"), String::from("2"), String::from("3")];
    /// let _f = Closure::fn_once(move |_: ()| {
    ///     for item in &values {
    ///         println!("Item: {}", item);
    ///     }
    ///     // Probably not how this would actually be used, just to demonstrate that we can.
    ///     std::mem::drop(values);
    /// });
    /// ```
    pub fn fn_once<Arg, Return, Function>(f: Function) -> Self
    where
        Arg: FromClosureArgPointer,
        Function: FnOnce(Arg) -> Return + Send + 'static,
    {
        let mut f = Some(f);
        Self::fn_mut(move |arg| match f.take() {
            Some(f) => f(arg),
            None => {
                error!("Function marked as single-use was called more than once, the closure will not be called as that would segfault. Aborting.");
                abort()
            }
        })
    }

    /// Constructs a new instance of this class that when called does nothing. It provides all
    /// possible signatures simultaneously, excluding those with a return value, because the
    /// `Closure` machinery will do nothing with it.
    pub fn new_noop() -> Self {
        Self {
            data: null_mut(),
            function: None,
            delete_data: None,
            delete_ret: None,
        }
    }

    /// Similar to the `rebind_closure` macro, except this operates on immutable references instead.
    pub fn rebind_closure_ref<C: ClosureMarkerTrait>(&self) -> &C {
        // size_of here is a const fn, so this branch will be optimized out of existence.
        if size_of::<C>() != size_of::<Self>() {
            panic!("rebind_closure_ref external definition is not the same size as internal definition. \
            `ClosureMarkerTrait` is probably implemented incorrectly. This also might be a bug in c-closures.")
        } else {
            unsafe { &*(self as *const Self as *const C) }
        }
    }

    /// Similar to the `rebind_closure` macro, except this operates on mutable references instead.
    pub fn rebind_closure_mut<C: ClosureMarkerTrait>(&mut self) -> &mut C {
        // size_of here is a const fn, so this branch will be optimized out of existence.
        if size_of::<C>() != size_of::<Self>() {
            panic!("rebind_closure_mut external definition is not the same size as internal definition. \
            `ClosureMarkerTrait` is probably implemented incorrectly. This also might be a bug in c-closures.")
        } else {
            unsafe { &mut *(self as *mut Self as *mut C) }
        }
    }
}

// In the unlikely event a `Closure` is released while on the Rust side, we need to dispose of it correctly.
impl Drop for Closure {
    fn drop(&mut self) {
        unsafe {
            closure_release(self);
        }
    }
}

unsafe extern "C" fn f_wrapper<Arg, Return, Function>(f: *mut c_void, a: *mut c_void) -> *mut c_void
where
    Arg: FromClosureArgPointer,
    Function: FnMut(Arg) -> Return + Send + 'static,
{
    let f = &mut *(f as *mut Function);
    let arg = if a.is_null() && size_of::<Arg>() > 0 {
        error!(
        "Unexpected null argument received in Closure, the closure will not be called as that would segfault.\n{:?}",
        Backtrace::new()
      );
        None
    } else {
        Some(Arg::from_arg_ptr(a))
    };

    arg.map(|arg| Box::into_raw(Box::new(f(arg))) as *mut c_void)
        .unwrap_or(null_mut())
}

unsafe extern "C" fn delete_me<T>(t: *mut c_void) {
    // The box takes back ownership, and is then dropped, preventing a leak of the closure data.
    Box::from_raw(t as *mut T);
}

/// This trait identifies instances of the `Closure` type from `rust_closures.h`. In Rust land, there will be
/// multiple instances of this type that we need to be able to cast from one to another. This trait helps us
/// determine which of these casts are safe. To implement this use `BindgenBuilderExt::c_closures_enhancements`
/// from `c-closures-build` on your `bindgen::Builder`.
pub trait ClosureMarkerTrait {}

impl ClosureMarkerTrait for Closure {}

/// Provides a general purpose way to deref a structure from a C void pointer. Auto implemented for `Copy` types.
pub trait FromClosureArgPointer {
    /// # Safety
    ///
    /// Incorrect implementations of this trait may lead to undefined behavior. If you're trying to read out a
    /// pointer type, then the pointer passed to this trait is a pointer to your pointer, not the pointer itself.
    unsafe fn from_arg_ptr(ptr: *const c_void) -> Self;
}

impl<T: Copy> FromClosureArgPointer for T {
    unsafe fn from_arg_ptr(ptr: *const c_void) -> Self {
        *(ptr as *const T)
    }
}

/// Rebinds a `Closure` from this crate to a `Closure` type defined externally.
/// If you use bindgen to make bindings to C/C++ functions accepting this `Closure` type then the bindings won't
/// be defined in terms of `c_closures`, instead your functions will want an instance of your own `Closure` definition.
/// This macro provides a convenient way to rebind them.
///
/// ```
/// use c_closures::{Closure, rebind_closure};
/// mod ffi {
///     // Import of bindgen generated closure here.
///     // i.e. include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
/// # #[repr(C)]
/// # pub struct Closure {
/// #  inner: [u8; std::mem::size_of::<c_closures::Closure>()],
/// # }
/// # impl c_closures::ClosureMarkerTrait for Closure {}
/// }
/// # fn main () {
/// // elsewhere
/// let c = rebind_closure!(ffi::Closure, Closure::fn_not_mut(|_: ()| 2 + 2));
/// # }
/// ```
#[macro_export]
macro_rules! rebind_closure {
    ($external_name:ty, $closure:expr) => {
        { // Additional scope added to prevent leaking of the fn definition.
            fn is_closure_type<C: $crate::ClosureMarkerTrait>() {}

            // size_of here is a const fn, so this branch will be optimized out of existence.
            if ::std::mem::size_of::<$external_name>() != ::std::mem::size_of::<$crate::Closure>() {
                panic!("rebind_ref! macro external definition is not the same size as internal definition. \
                `ClosureMarkerTrait` is probably implemented incorrectly.")
            } else {
                is_closure_type::<$external_name>(); // Intentionally creates a compiler error if the marker trait isn't implemented.
                unsafe {
                    ::std::mem::transmute::<$crate::Closure, $external_name>($closure)
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use std::{
        ffi::{CStr, CString},
        sync::Arc,
    };

    use super::*;

    #[test]
    fn fn_not_mut() {
        let y = 4;
        let mut closure = Closure::fn_not_mut(move |x: i32| x + x + y);
        unsafe {
            let ret = closure_call(&mut closure, &mut 2 as *mut i32 as _);
            assert_eq!(<i32 as FromClosureArgPointer>::from_arg_ptr(ret), 8);
            closure_release_return_value(&mut closure, ret);
            closure_release(&mut closure);
        }
    }

    #[test]
    fn fn_mut() {
        let mut y = 4;
        let mut closure = Closure::fn_mut(move |x: i32| {
            y *= 2;
            x + x + y
        });
        unsafe {
            let ret = closure_call(&mut closure, &mut 2 as *mut i32 as _);
            assert_eq!(<i32 as FromClosureArgPointer>::from_arg_ptr(ret), 12);
            closure_release_return_value(&mut closure, ret);

            let ret = closure_call(&mut closure, &mut 2 as *mut i32 as _);
            assert_eq!(<i32 as FromClosureArgPointer>::from_arg_ptr(ret), 20);
            closure_release_return_value(&mut closure, ret);
            closure_release(&mut closure);
        }
    }

    #[test]
    fn fn_once() {
        let mut y = 4;
        let mut closure = Closure::fn_once(move |x: i32| {
            y *= 2;
            x + x + y
        });
        unsafe {
            let ret = closure_call(&mut closure, &mut 2 as *mut i32 as _);
            assert_eq!(<i32 as FromClosureArgPointer>::from_arg_ptr(ret), 12);
            closure_release_return_value(&mut closure, ret);

            // I'd love to verify that a subsequent call aborts, but it's non-trivial
            // to put that into a test suite. We'll address this if it ever becomes a problem
            // that this testing isn't done.
            closure_release(&mut closure);
        }
    }

    #[test]
    fn fn_cstring() {
        let mut closure = Closure::fn_not_mut(|name: &CStr| {
            CString::new(format!("Hello {}", name.to_str().unwrap())).unwrap()
        });
        let my_name = CString::new("Jacob").unwrap();
        unsafe {
            let ret = closure_call(&mut closure, &mut my_name.as_c_str() as *mut &CStr as _);
            assert_eq!(
                (&mut *(ret as *mut CString)).clone().into_string().unwrap(),
                "Hello Jacob"
            );
            closure_release_return_value(&mut closure, ret);
            closure_release(&mut closure);
        }
    }

    #[test]
    fn fn_drop_test() {
        let value = Arc::new(());
        let value_clone = value.clone();
        let mut closure = Closure::fn_not_mut(move |_: ()| value_clone.clone());
        unsafe {
            let ret = closure_call(&mut closure, &mut () as *mut () as _);
            assert_eq!(Arc::strong_count(&value), 3);
            closure_release_return_value(&mut closure, ret);
            assert_eq!(Arc::strong_count(&value), 2);
            closure_release(&mut closure);
            assert_eq!(Arc::strong_count(&value), 1);
        }
    }

    struct NotAClosure;

    impl ClosureMarkerTrait for NotAClosure {}

    #[test]
    #[should_panic]
    fn bad_ref_usage() {
        let c = Closure::fn_not_mut(|_: ()| ());
        c.rebind_closure_ref::<NotAClosure>();
    }

    #[test]
    #[should_panic]
    fn bad_mut_usage() {
        let mut c = Closure::fn_not_mut(|_: ()| ());
        c.rebind_closure_mut::<NotAClosure>();
    }

    // Validates that calling this macro doesn't result in leaky definitions. Items
    // defined in the macro should not exist outside of it.
    #[test]
    fn two_in_scope() {
        rebind_closure!(Closure, Closure::new_noop());
        rebind_closure!(Closure, Closure::new_noop());
    }
}
