#include "rust_closures.h" 

CLOSURE_DEF(IntInt, int, Int, int, p1)
CLOSURE_DEF(IntIntInt, int, Int, int, p1, int, p2)
CLOSURE_DEF_VOID_RET(VoidInt, int, p1)
CLOSURE_DEF(IntVoid, int, Int, void)
CLOSURE_DEF(IntVoidClosureFactory, IntVoidClosure, IntVoidClosure, void)

int maybe_call(VoidVoidClosure *closure);