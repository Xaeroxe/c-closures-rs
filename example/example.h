#include "rust_closures.h" 

CLOSURE_DEF(IntInt, int, int, p1)
CLOSURE_DEF(IntIntInt, int, int, p1, int, p2)
CLOSURE_DEF_VOID_RET(VoidInt, int, p1)
CLOSURE_DEF(IntVoid, int, void)
CLOSURE_DEF(IntVoidClosureFactory, IntVoidClosure, void)

int maybe_call(VoidVoidClosure *closure);