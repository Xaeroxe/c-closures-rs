/// A general purpose closure type defined in C code which can be created in Rust.
typedef struct Closure {
  /// Directions to call the contained closure
  void *(*function)(void * const data, void * const arg);
  /// Rust user data for this closure.
  void * const data;
  /// The data pointer may require personalized delete instructions, we can
  /// access those here.
  void (*delete_data)(void *data);
  /// The value returned by this function may require personalized delete
  /// instructions, we can access those here.
  void (*delete_ret)(void *ret);
} Closure;

/// Calls the inner code. The return value of this may have come from
/// Rust, meaning you can not free it. However it must be freed. When
/// you're done with the return value, pass it back to Rust with
/// CleanReturnValue so that the memory isn't leaked. If you won't be
/// using the return value, instead call CallWithNoReturn.
void *closure_call(Closure * const self, void * const arg);

/// Cleans up the value returned by calling this Closure. Do not attempt
/// to free the returned pointer yourself.
void closure_release_return_value(Closure * const self, void *ret);

/// Calls the inner code and cleans up the returned value, if any.
void closure_call_with_no_return(Closure * const self, void * const arg);

/// Release data associated with this closure, must be called when done with
/// Closure to avoid memory leaking.
void closure_release(Closure * const self);