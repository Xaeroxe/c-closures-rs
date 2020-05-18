//! # Purpose
//!
//! This crate is for producing Rust closures that can cross an FFI boundary.
//! It provides support for any function signature, assuming all of the types
//! in it have valid representations in C/C++ and Rust.
//!
//! [Here's an example.](https://github.com/Xaeroxe/c-closures-rs/tree/master/example)
//!
//! # Safety concerns
//!
//! Creating a `*Closure` by itself can not cause undefined behavior, however when the resulting
//! structure is used in C/C++ it can still trigger undefined behavior. `*Closure` should never be
//! an argument to a safe function, nor should it be a public member of any structures passed into a safe function.
//! Please write your own safe wrappers that incorporate the `*Closure` types internally.
//!
//! # Usage in C/C++
//!
//! To use this with a C/C++ library you'll need to include the header provided in the repo,
//! `rust_closures.h`. Then you can accept the relevant `*Closure` type anywhere that you need to
//! accept arbitrary Rust code.
//!
//! # Limitations
//!
//! This cannot be used to transfer ownership of allocated memory across FFI boundaries, as this crate cannot reasonably guarantee
//! both sides are using the same memory allocator, or dispose of the types in the same way. If such transfer
//! is required, you should copy the data into a new allocation, on the side of the FFI boundary it needs to live
//! on. The major exception to this is types with the `Copy` marker trait, which are trivially cloned and require
//! no disposal instructions.

use std::{
    io::{BufWriter, Write},
    path::PathBuf,
    process::{Command, Stdio},
};

use quote::{format_ident, quote, ToTokens};
use syn::{parse2, parse_str, File, FnArg, ForeignItem, Item, ReturnType, Signature, Type};

/// Provides the path containing `rust_closures.h`.
/// You'll need to include this path to compile any C/C++ code making use of this crate's `Closure` types.
pub fn c_closure_header_include_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

const SPECIAL_FN_SUFFIX: &str = "_closure_call";

struct ClosureDefinition {
    name: String,
    signature: Signature,
}

/// Accepts a blob of auto generated rust code binding to a C/C++ library, probably from `bindgen`,
/// analyzes it searching for instances of `Closure` definitions. When it finds them, it
/// enhances the definition with additional functions that allow passing in a rust closure
/// with a matching signature for the `Closure` definition. Outputs the initial blob,
/// with the accompanying enhancements. This attempts to `rustfmt` the output, but if that fails
/// will instead output rust code on a single line. That can make your error messages really ugly looking.
pub fn enhance_closure_bindings(rust_code: &str) -> String {
    let mut tree = parse_str::<File>(rust_code).unwrap();
    let mut new_enhancements = vec![];
    for tree in tree.items.iter_mut() {
        new_enhancements.extend(call_recurse(tree, &mut |item| {
            let mut enhance = vec![];
            if let Item::ForeignMod(foreigners) = item {
                for foreign_item in &mut foreigners.items {
                    if let ForeignItem::Fn(function) = foreign_item {
                        let function_name = function.sig.ident.to_string();
                        if function_name.ends_with(SPECIAL_FN_SUFFIX) {
                            let closure_name = (&function_name
                                [0..(function_name.len() - SPECIAL_FN_SUFFIX.len())])
                                .to_string();
                            enhance.push(ClosureDefinition {
                                name: closure_name,
                                signature: function.sig.clone(),
                            });
                        }
                    }
                }
            }
            enhance.iter().flat_map(gen_closure_fns).collect()
        }));
    }
    tree.items.extend(new_enhancements);
    let tokenified_source = tree.to_token_stream().to_string();
    if let Ok(mut rust_fmt_process) = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        {
            if let Some(mut input) = rust_fmt_process.stdin.as_mut().map(BufWriter::new) {
                let _ = input.write_all(tokenified_source.as_bytes());
            }
        }
        rust_fmt_process
            .wait_with_output()
            .map_err(|_| ())
            .and_then(|o| {
                if o.status.success() {
                    String::from_utf8(o.stdout).map_err(|_| ())
                } else {
                    Err(())
                }
            })
            .unwrap_or(tokenified_source)
    } else {
        tokenified_source
    }
}

fn call_recurse<F: FnMut(&mut Item) -> Vec<Item>>(item: &mut Item, f: &mut F) -> Vec<Item> {
    let mut enhancements = vec![];
    enhancements.extend(f(item));
    if let Item::Mod(mmod) = item {
        if let Some(t) = mmod.content.as_mut() {
            let new_enhancements =
                t.1.iter_mut()
                    .flat_map(|item| call_recurse(item, f))
                    .collect::<Vec<_>>();
            t.1.extend(new_enhancements);
        }
    }
    enhancements
}

fn gen_closure_fns(
    &ClosureDefinition {
        ref name,
        ref signature,
    }: &ClosureDefinition,
) -> Vec<Item> {
    let closure_name = format_ident!("{}Closure", name);
    let release_name = format_ident!("{}_closure_release", name);
    let args = signature
        .inputs
        .iter()
        .skip(1)
        .map(|arg| match arg {
            FnArg::Typed(pat_type) => (*pat_type.ty).clone(),
            _ => unreachable!("Functions passed into here should never have a self reference."),
        })
        .map(|a| a.to_token_stream())
        .collect::<Vec<_>>();
    let arg_idents = (0..args.len())
        .map(|i| format_ident!("_p{}", i))
        .collect::<Vec<_>>();
    let arg_ident_pairs = args
        .iter()
        .zip(arg_idents.iter())
        .map(|(arg, ident)| quote!(#ident: #arg))
        .collect::<Vec<_>>();
    let (has_return_value, return_type) = match &signature.output {
        ReturnType::Default => (false, Type::Verbatim(quote!(()))),
        ReturnType::Type(_, ref ty) => (true, (**ty).clone()),
    };
    let delete_ret = if has_return_value {
        quote!(delete_ret: Some(Self::drop_me::<#return_type>),)
    } else {
        quote!()
    };

    #[cfg(feature = "no_std")]
    let std_or_core = quote!(core);

    #[cfg(not(feature = "no_std"))]
    let std_or_core = quote!(std);

    #[cfg(feature = "no_std")]
    let std_or_alloc = quote!(alloc);

    #[cfg(not(feature = "no_std"))]
    let std_or_alloc = quote!(std);

    #[cfg(feature = "no_std")]
    let abort_or_zeroed = quote!(::core::mem::zeroed());

    #[cfg(not(feature = "no_std"))]
    let abort_or_zeroed = quote! {
        eprintln!("Function marked as single-use was called more than once, the closure will not be called as that would segfault. Aborting.");
        ::std::process::abort()
    };

    let noop = if has_return_value {
        quote!()
    } else {
        quote! {
            /// Constructs a new instance of this class that when called does nothing.
            pub fn new_noop() -> Self {
                Self::fn_not_mut(|#(#arg_idents),*| ())
            }
        }
    };
    vec![
        // primary fn block
        parse2(
            quote! {
                impl #closure_name {

                    unsafe extern "C" fn f_wrapper<F>(f: *mut ::#std_or_core::ffi::c_void, #(#arg_ident_pairs),*) -> #return_type
                    where
                        F: FnMut(#(#args),*) -> #return_type,
                    {
                        let f = &mut *(f as *mut F);
                        f(#(#arg_idents),*)
                    }

                    unsafe extern "C" fn drop_my_box<T>(t: *mut ::#std_or_core::ffi::c_void) {
                        // Drop is implicit
                        ::#std_or_alloc::boxed::Box::<T>::from_raw(t as *mut T);
                    }

                    unsafe extern "C" fn drop_me<T>(_t: T) {
                        // Drop is implicit
                    }

                    /// Transform an FnMut Rust closure into a structure you can pass into a C/C++ library.
                    ///
                    /// This structure currently assumes it will never be called in multiple threads
                    /// simultaneously. If that guarantee cannot be upheld, then you should instead use `fn_not_mut`.
                    pub fn fn_mut<Function>(f: Function) -> Self
                    where
                        Function: FnMut(#(#args),*) -> #return_type,
                    {
                        Self {
                            data: ::#std_or_alloc::boxed::Box::into_raw(::#std_or_alloc::boxed::Box::new(f)) as *mut ::#std_or_core::ffi::c_void,
                            function: Some(Self::f_wrapper::<Function>),
                            delete_data: Some(Self::drop_my_box::<Function>),
                            #delete_ret
                        }
                    }

                    /// Transform an Fn Rust closure into a structure you can pass into a C/C++ library.
                    ///
                    /// This structure is safe to use in multiple threads simultaneously. If your usage is single
                    /// threaded, consider `fn_mut` instead as it permits more robust closures.
                    pub fn fn_not_mut<Function>(f: Function) -> Self
                    where
                        Function: Fn(#(#args),*) -> #return_type,
                    {
                        Self {
                            data: ::#std_or_alloc::boxed::Box::into_raw(::#std_or_alloc::boxed::Box::new(f)) as *mut ::#std_or_core::ffi::c_void,
                            function: Some(Self::f_wrapper::<Function>),
                            delete_data: Some(Self::drop_my_box::<Function>),
                            #delete_ret
                        }
                    }

                    /// Transform an FnOnce Rust closure into a structure you can pass into a C/C++ library.
                    ///
                    /// This structure assumes it will only ever be called once. If you attempt to call it more than once
                    /// the program will abort. If the `no_std` feature is enabled, instead you'll received zeroed memory.
                    pub fn fn_once<Function>(f: Function) -> Self
                    where
                        Function: FnOnce(#(#args),*) -> #return_type,
                    {
                        let mut f = Some(f);
                        Self::fn_mut(move |#(#arg_idents),*| match f.take() {
                            Some(f) => f(#(#arg_idents),*),
                            None => {
                                #abort_or_zeroed
                            }
                        })
                    }

                    #noop
                }
            }
        ).unwrap(),
        // drop block
        parse2(
            quote! {
                impl Drop for #closure_name {
                    fn drop(&mut self) {
                        unsafe {
                            #release_name(self)
                        }
                    }
                }
            }
        ).unwrap()
    ]
}
