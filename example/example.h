#include "rust_closures.h" 

CLOSURE_DEF(IntInt, int, int, p1)
CLOSURE_DEF(IntIntInt, int, int, p1, int, p2)
CLOSURE_DEF_VOID_RET(VoidInt, int, p1)

int maybe_call(VoidVoidClosure *closure);