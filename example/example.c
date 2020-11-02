#include "rust_closures.h"
#include "example.h"
#include <stdio.h>
#include <stdlib.h>

CLOSURE_DEF(IntInt, int, Int, int, p1)
CLOSURE_DEF(IntIntInt, int, Int, int, p1, int, p2)
CLOSURE_DEF_VOID_RET(VoidInt, int, p1)
CLOSURE_DEF_VOID_RET(VoidVoid, void)
CLOSURE_DEF(IntVoid, int, Int, void)
CLOSURE_DEF(IntVoidClosureFactory, IntVoidClosure, IntVoidClosure, void)


int maybe_call(VoidVoidClosure *closure) {
    if (rand() % 2 == 1) {
        VoidVoid_closure_call(closure);
        return 1;
    }
    return 0;
}
