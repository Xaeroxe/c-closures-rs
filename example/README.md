## Why do I need this?

[Rust closures](https://doc.rust-lang.org/book/ch13-01-closures.html) are a very
useful construct that enables functional programming and functions as first
class data. There's just one problem. They have a hard time being represented or
called in C/C++ land. This crate solves that! Now you can define snippets of
code in Rust that capture their environment and execute them in C/C++.

## Why not just use a function pointer?

[Rust function pointers](https://doc.rust-lang.org/reference/types/function-pointer.html)
are neat, and indeed this crate is powered by several. However a function
pointer is missing capabilities that a `*Closure` has, namely the ability to
capture variables from the outer scope. Additionally, as of this writing (Rust
1.43.1) references to non-capturing Rust closures don't coerce to a C compatible
function pointer.

## C Macros in this crate

First off it's important to understand the two macros provided by
`rust_closures.h` which you'll use in the C/C++ code to define `*Closure`
signatures. The first is `CLOSURE_DEF` , which is used to define `*Closure`
types with a non-void return type. It's usage is as such

``` C
CLOSURE_DEF(<SIGNATURE NAME>, <RETURN TYPE>, <RETURN TYPE NAME>, <TYPE OF FIRST CLOSURE ARG>, <NAME OF FIRST CLOSURE ARG>, <..ADDITIONAL ARGS>)
```

The first three parameters to this macro are required, subsequent arguments are optional.

The return type name is used as the prefix of the function to drop the return value. It should only contain characters legal in a C function name.

The second macro this crate provides is `CLOSURE_DEF_VOID_RET` which is useful
for `*Closure` types you don't actually need anything returned from. It's usage
is as such

``` C
CLOSURE_DEF_VOID_RET(<SIGNATURE NAME>, <TYPE OF FIRST CLOSURE ARG>, <NAME OF FIRST CLOSURE ARG>, <..ADDITIONAL ARGS>)
```

The first parameter to this macro is required, subsequent arguments are
optional. That being said, if you're using this macro without any input
parameters, consider instead using `VoidVoidClosure` which is defined in
`rust_closures.h`.

## Okay, what do I get for it?

Here's the expansion of the macro for a simple signature.

``` C
CLOSURE_DEF(IntInt, int, Int, int, p1)
```

``` C
typedef struct IntIntClosure
{
	int(*function)(void *data _ARGIFY(int, p1));
	void *data;
	void(*delete_data)(void *data);
} IntIntClosure;

int IntInt_closure_call(IntIntClosure *const self _ARGIFY(int, p1))
{
	return (self->function)(self->data _EVERY_OTHER(int, p1));
}

void Int_release_rust_return_value(int ret); // Defined in Rust later.

void IntInt_closure_call_with_no_return(IntIntClosure *const self _ARGIFY(int, p1))
{
	Int_release_rust_return_value(IntInt_closure_call(self _EVERY_OTHER(int, p1)));
}

void IntInt_closure_release(IntIntClosure *const self)
{
	if (self->delete_data != 0 && self->data != 0)
	{
		(self->delete_data)(self->data);
		self->delete_data = 0;
		self->data = 0;
	}
}
```

So for this expansion you get a struct, called `IntIntClosure`, and four
functions, three of which are prefixed with the name `IntInt`. One function,
`*_release_rust_return_value` is left undefined. Rust will define it later.
If you instead use `CLOSURE_DEF_VOID_RET` then `*_release_rust_return_value`
and `*_closure_call_with_no_return` are omitted as those functions are
extraneous for a `void` return type.

Creation of the struct is handled in Rust, we'll get to that later. Once you
have the struct it contains handles to Rust data, and depending on your
signature, may generate handles to Rust data when called. These handles need to
be cleaned up when we're done with them. When you're done with the `*Closure`
type, you'll need to call `*_closure_release` on it, and when you're done with
the return value you'll need to call `*_release_rust_return_value` on
that.<sup>1</sup> Do not attempt to delete Rust memory allocations with anything
other than these functions, Rust may not be using the same memory allocator as
C, meaning they have no common language for memory allocation operations.

<sup>1. Types which have a Rust `std::marker::Copy` implementation don't
necessarily need to be released, but if there's any doubt in your mind about
whether such an implementation exists, you should assume it does not exist.</sup>

Alright great so now we have a type we can expose in our C function signatures
and types that represent arbitrary Rust closures, and we have facilities to
call that code, and clean up after ourselves as needed.

## How do I create a `*Closure` in Rust?

After you've generated/written your bindings to the C code you might find them
lacking when it comes to creating these types. That's where `c-closures-build`
comes in! It provides two functions of use to you here.

### Build Functions

* `enhance_closure_bindings` - The powerhouse of the operation. This function
  analyzes Rust code, finds `*Closure` definitions, and enhances them with a few
  construction functions for use in Rust.

* `c_closure_header_include_dir` - This function provides a path containing
  `rust_closures.h` , which is useful when compiling the C/C++ code from a
  `build.rs` script. If I were altering a `bindgen` 0.53 generator with this
  function I'd do it like so:
  `.clang_arg(format!("-I{}", c_closure_header_include_dir().display()))`.

### Construction Functions

Here's the different construction functions provided by our enhancements.

* `fn_mut` - Accepts an `FnMut` Rust closure and transforms it into an instance
  of this `*Closure` type. If you're sure this will never be called by multiple
  threads in C/C++ and you need to call it multiple times this is the function
  you should prefer.

* `fn_not_mut` - Sorry, `fn` is a keyword in Rust, so we get this silly name.
  This accepts an `Fn` Rust closure. This is the preferred function to use if
  the C/C++ code may be calling it from multiple threads.

* `fn_once` - This accepts an `FnOnce` Rust closure, it is the most powerful and
  most dangerous construction function. It places very few restrictions on what
  the Rust closure can capture, and what it does with that information. However
  if called more than once, this `*Closure` will cause your program to `abort()`.

* `new_noop` - Only available for types defined with `CLOSURE_DEF_VOID_RET`.
  This `*Closure` will when called, do nothing.

## Conclusion

Thanks for reading this! You should now be equipped to read the example, so just
start clicking around in it. As you've seen, this crate is powered almost
entirely by code generation. It provides very few definitions of its own. That
means this crate might be a bit difficult to use at times. If you run into
problems using it let me know on the issue tracker, I'm happy to help, even if
you don't think you've found a bug in this crate.
