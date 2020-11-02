#include "rust_closures.h" 

CLOSURE_DEF_HEAD(IntInt, int, Int, int, p1)
CLOSURE_DEF_HEAD(IntIntInt, int, Int, int, p1, int, p2)
CLOSURE_DEF_VOID_RET_HEAD(VoidInt, int, p1)
CLOSURE_DEF_VOID_RET_HEAD(VoidVoid, void)
CLOSURE_DEF_HEAD(IntVoid, int, Int, void)
CLOSURE_DEF_HEAD(IntVoidClosureFactory, IntVoidClosure, IntVoidClosure, void)

int maybe_call(VoidVoidClosure *closure);
