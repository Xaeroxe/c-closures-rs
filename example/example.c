// The include path is written this way in the example so I can avoid duplicating files,
// but you may want to keep a local copy of this header when using this yourself.
#include "../c-closures/rust_closures.h"
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