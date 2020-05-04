#include "rust_closures.h"
#include "example.h"
#include <stdio.h>
#include <stdlib.h>

int maybe_call(Closure *closure) {
    if (rand() % 2 == 1) {
        closure_call_with_no_return(closure, 0);
        return 1;
    }
    return 0;
}