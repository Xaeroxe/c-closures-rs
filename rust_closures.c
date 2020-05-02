#include "rust_closures.h"

void *closure_call(Closure * const self, void * const arg) {
  if (self->function != 0) {
    return (self->function)(self->data, arg);
  }
  return 0;
}

void closure_release_return_value(Closure * const self, void *ret) {
  if (ret != 0 && self->delete_ret != 0) {
    (self->delete_ret)(ret);
  }
}

void closure_call_with_no_return(Closure * const self, void * const arg) {
  closure_release_return_value(self, closure_call(self, arg));
}

void closure_release(Closure * const self) {
  if (self->delete_data != 0) {
    (self->delete_data)(self->data);
  }
}