#include "rust_closures.h"
#include "example.h"
#include <stdio.h>
#include <stdlib.h>

int maybe_call(VoidVoidClosure *closure) {
    if (rand() % 2 == 1) {
        VoidVoid_closure_call(closure);
        return 1;
    }
    return 0;
}