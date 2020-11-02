#ifndef RUST_CLOSURES_H
#define RUST_CLOSURES_H

// These macro definitions are not considered ideal, if you can think of a better way to do this, the author (Jacob Kiesel) would love to know.

#define _EVERY_OTHER0() 
#define _EVERY_OTHER1(_0)
#define _EVERY_OTHER2(_0, _1) ,  _1
#define _EVERY_OTHER4(_0, _1, _2, _3) ,  _1, _3
#define _EVERY_OTHER6(_0, _1, _2, _3, _4, _5) ,  _1, _3, _5
#define _EVERY_OTHER8(_0, _1, _2, _3, _4, _5, _6, _7) ,  _1, _3, _5, _7
#define _EVERY_OTHER10(_0, _1, _2, _3, _4, _5, _6, _7, _8, _9) ,  _1, _3, _5, _7, _9
#define _EVERY_OTHER12(_0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11) ,  _1, _3, _5, _7, _9, _11
#define _EVERY_OTHER14(_0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13) ,  _1, _3, _5, _7, _9, _11, _13
#define _EVERY_OTHER16(_0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15) ,  _1, _3, _5, _7, _9, _11, _13, _15
#define _EVERY_OTHER18(_0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17) ,  _1, _3, _5, _7, _9, _11, _13, _15, _17
#define _EVERY_OTHER20(_0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19) ,  _1, _3, _5, _7, _9, _11, _13, _15, _17, _19
#define _EVERY_OTHER22(_0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21) ,  _1, _3, _5, _7, _9, _11, _13, _15, _17, _19, _21
#define _EVERY_OTHER24(_0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23) ,  _1, _3, _5, _7, _9, _11, _13, _15, _17, _19, _21, _23
#define _EVERY_OTHER26(_0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25) ,  _1, _3, _5, _7, _9, _11, _13, _15, _17, _19, _21, _23, _25
#define _EVERY_OTHER28(_0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27) ,  _1, _3, _5, _7, _9, _11, _13, _15, _17, _19, _21, _23, _25, _27
#define _EVERY_OTHER30(_0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29) ,  _1, _3, _5, _7, _9, _11, _13, _15, _17, _19, _21, _23, _25, _27, _29
#define _EVERY_OTHER32(_0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31) ,  _1, _3, _5, _7, _9, _11, _13, _15, _17, _19, _21, _23, _25, _27, _29, _31

#define _ARGIFY0() 
#define _ARGIFY1(_0)
#define _ARGIFY2(_0, _1) ,  _0 _1
#define _ARGIFY4(_0, _1, _2, _3) ,  _0 _1, _2 _3
#define _ARGIFY6(_0, _1, _2, _3, _4, _5) ,  _0 _1, _2 _3, _4 _5
#define _ARGIFY8(_0, _1, _2, _3, _4, _5, _6, _7) ,  _0 _1, _2 _3, _4 _5, _6 _7
#define _ARGIFY10(_0, _1, _2, _3, _4, _5, _6, _7, _8, _9) ,  _0 _1, _2 _3, _4 _5, _6 _7, _8 _9
#define _ARGIFY12(_0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11) ,  _0 _1, _2 _3, _4 _5, _6 _7, _8 _9, _10 _11
#define _ARGIFY14(_0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13) ,  _0 _1, _2 _3, _4 _5, _6 _7, _8 _9, _10 _11, _12 _13
#define _ARGIFY16(_0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15) ,  _0 _1, _2 _3, _4 _5, _6 _7, _8 _9, _10 _11, _12 _13, _14 _15
#define _ARGIFY18(_0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17) ,  _0 _1, _2 _3, _4 _5, _6 _7, _8 _9, _10 _11, _12 _13, _14 _15, _16 _17
#define _ARGIFY20(_0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19) ,  _0 _1, _2 _3, _4 _5, _6 _7, _8 _9, _10 _11, _12 _13, _14 _15, _16 _17, _18 _19
#define _ARGIFY22(_0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21) ,  _0 _1, _2 _3, _4 _5, _6 _7, _8 _9, _10 _11, _12 _13, _14 _15, _16 _17, _18 _19, _20 _21
#define _ARGIFY24(_0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23) ,  _0 _1, _2 _3, _4 _5, _6 _7, _8 _9, _10 _11, _12 _13, _14 _15, _16 _17, _18 _19, _20 _21, _22 _23
#define _ARGIFY26(_0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25) ,  _0 _1, _2 _3, _4 _5, _6 _7, _8 _9, _10 _11, _12 _13, _14 _15, _16 _17, _18 _19, _20 _21, _22 _23, _24 _25
#define _ARGIFY28(_0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27) ,  _0 _1, _2 _3, _4 _5, _6 _7, _8 _9, _10 _11, _12 _13, _14 _15, _16 _17, _18 _19, _20 _21, _22 _23, _24 _25, _26 _27
#define _ARGIFY30(_0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29) ,  _0 _1, _2 _3, _4 _5, _6 _7, _8 _9, _10 _11, _12 _13, _14 _15, _16 _17, _18 _19, _20 _21, _22 _23, _24 _25, _26 _27, _28 _29
#define _ARGIFY32(_0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31) ,  _0 _1, _2 _3, _4 _5, _6 _7, _8 _9, _10 _11, _12 _13, _14 _15, _16 _17, _18 _19, _20 _21, _22 _23, _24 _25, _26 _27, _28 _29, _30 _31

// Accept any number of args >= N, but expand to just the Nth one. In this case,
// we have settled on 33 as N. We could pick a different number by adjusting
// the count of throwaway args before N. Note that this macro is preceded by
// an underscore--it's an implementation detail, not something we expect people
// to call directly.
#define _GET_NTH_ARG(_1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, _31, _32, N, ...) N

#define _EVERY_OTHER(...) _GET_NTH_ARG(__VA_ARGS__, _EVERY_OTHER32, _EVERY_OTHER31, _EVERY_OTHER30, _EVERY_OTHER29, _EVERY_OTHER28, _EVERY_OTHER27, _EVERY_OTHER26, _EVERY_OTHER25, _EVERY_OTHER24, _EVERY_OTHER23, _EVERY_OTHER22, _EVERY_OTHER21, _EVERY_OTHER20, _EVERY_OTHER19, _EVERY_OTHER18, _EVERY_OTHER17, _EVERY_OTHER16, _EVERY_OTHER15, _EVERY_OTHER14, _EVERY_OTHER13, _EVERY_OTHER12, _EVERY_OTHER11, _EVERY_OTHER10, _EVERY_OTHER9, _EVERY_OTHER8, _EVERY_OTHER7, _EVERY_OTHER6, _EVERY_OTHER5, _EVERY_OTHER4, _EVERY_OTHER3, _EVERY_OTHER2, _EVERY_OTHER1, _EVERY_OTHER0)(__VA_ARGS__)

#define _ARGIFY(...) _GET_NTH_ARG(__VA_ARGS__, _ARGIFY32, _ARGIFY31, _ARGIFY30, _ARGIFY29, _ARGIFY28, _ARGIFY27, _ARGIFY26, _ARGIFY25, _ARGIFY24, _ARGIFY23, _ARGIFY22, _ARGIFY21, _ARGIFY20, _ARGIFY19, _ARGIFY18, _ARGIFY17, _ARGIFY16, _ARGIFY15, _ARGIFY14, _ARGIFY13, _ARGIFY12, _ARGIFY11, _ARGIFY10, _ARGIFY9, _ARGIFY8, _ARGIFY7, _ARGIFY6, _ARGIFY5, _ARGIFY4, _ARGIFY3, _ARGIFY2, _ARGIFY1, _ARGIFY0)(__VA_ARGS__)

#define CLOSURE_DEF(definition_name, return_type, return_type_name, ...)  \
const char ____xpaQBSrQUbNWjnzsGvEgOEjbtPAGJISUDgCbJiUyQWnbqEkYesdTqYoJaKYcHsdRRlZNLPYPCoWBkDZefGQwCilbNJIIsNBeLkKs##definition_name = 0;\
/* A user defined closure type from C code which can be created in Rust. */ \
typedef struct definition_name##Closure { \
  /* Directions to call the contained closure */ \
  return_type (*function)(void * data _ARGIFY(__VA_ARGS__)); \
  /* Rust user data for this closure. */ \
  void * data; \
  /* The data pointer may require personalized delete instructions, we can \
  access those here. */ \
  void (*delete_data)(void *data); \
} definition_name##Closure; \
\
/* Calls the inner code. The return value of this may have come from \
 Rust, meaning you can not free it. However it must be freed. When \
 you're done with the return value, pass it back to Rust with \
 `<closure_name>_closure_release_return_value` so that the memory isn't leaked. If you won't be \
 using the return value, instead call `<closure_name>_closure_call_with_no_return`. */ \
return_type definition_name##_closure_call(definition_name##Closure * const self _ARGIFY(__VA_ARGS__))  {\
  return (self->function)(self->data _EVERY_OTHER(__VA_ARGS__)); \
} \
\
/* Cleans up the value returned by calling a Rust Closure. Do not attempt \
to free the returned value yourself. */ \
void return_type_name##_release_rust_return_value(return_type ret);\
\
/* Calls the inner code and cleans up the returned value, if any. */ \
void definition_name##_closure_call_with_no_return(definition_name##Closure * const self _ARGIFY(__VA_ARGS__)) { \
  return_type_name##_release_rust_return_value(definition_name##_closure_call(self _EVERY_OTHER(__VA_ARGS__))); \
} \
\
/* Release data associated with this closure, must be called when done with \
 Closure to avoid memory leaking. */ \
void definition_name##_closure_release(definition_name##Closure * const self) { \
  if (self->delete_data != 0 && self->data != 0) { \
    (self->delete_data)(self->data); \
    self->delete_data = 0; \
    self->data = 0; \
  } \
}

#define CLOSURE_DEF_VOID_RET(definition_name, ...)  \
const char ____xpaQBSrQUbNWjnzsGvEgOEjbtPAGJISUDgCbJiUyQWnbqEkYesdTqYoJaKYcHsdRRlZNLPYPCoWBkDZefGQwCilbNJIIsNBeLkKs_##definition_name = 0;\
/* A user defined closure type from C code which can be created in Rust. */ \
typedef struct definition_name##Closure { \
  /* Directions to call the contained closure */ \
  void (*function)(void * data _ARGIFY(__VA_ARGS__)); \
  /* Rust user data for this closure. */ \
  void * data; \
  /* The data pointer may require personalized delete instructions, we can \
  access those here. */ \
  void (*delete_data)(void *data); \
} definition_name##Closure; \
\
/* Calls the inner code. The return value of this may have come from \
 Rust, meaning you can not free it. However it must be freed. When \
 you're done with the return value, pass it back to Rust with \
 `<closure_name>_closure_release_return_value` so that the memory isn't leaked. If you won't be \
 using the return value, instead call `<closure_name>_closure_call_with_no_return`. */ \
void definition_name##_closure_call(definition_name##Closure * const self _ARGIFY(__VA_ARGS__))  {\
  (self->function)(self->data _EVERY_OTHER(__VA_ARGS__)); \
} \
\
/* Release data associated with this closure, must be called when done with \
 Closure to avoid memory leaking. */ \
void definition_name##_closure_release(definition_name##Closure * const self) { \
  if (self->delete_data != 0 && self->data != 0) { \
    (self->delete_data)(self->data); \
    self->delete_data = 0; \
    self->data = 0; \
  } \
}


CLOSURE_DEF_VOID_RET(VoidVoid, void)
#endif
