# Unsafe code in Rust

The `unsafe` keyword has two uses:

- To declare the existence of contracts the compiler can't check.
- To declare that a programmer has checked that these contracts have been upheld.

You can use `unsafe` to indicate the existence of unchecked contracts on _functions_ and _trait declarations_.

Some examples:

- `slice::get_unchecked`, which performs unchecked indexing, allowing memory safety to be freely violated.
- `mem::transmute` reinterprets some value as having a give type, bypassing type safety in arbitraty ways

## Unsafe superpowers

The unsafe superpowers are:

- Dereference a raw pointer.
- Call an unsafe function or method.
- Access or modify a mutable static variable.
- Implement an unsafe trait.
- Access fields of `union`s.

### Dereferencing a Raw Pointer

```rust
fn main() {
  let _ = dangling_pointer();
}

fn dangling_pointer() -> &String {
  return &String::from("Hello world");
}
```

As with references, _raw pointers_ can be _immutable_ or _mutable_ and are written as `*const T` and `*mut T`, respectively. `*` doesn't mean _dereference_, it's part of the type name.

Differences from references and smart pointers, raw pointers:

- Are allowed to ignore the _borrowing rules_ by having both immutable and mutable pointers or multiple mutable pointers to the same location.
- Aren't guaranteed to point to a valid memory.
- Are allowed to be null.
- Don't implement any automatic cleanup.

Example:

```rust
fn main() {
  let mut num = 5;

  let r1: *const i32 = &num as *const i32;
  let r2: *mut i32 = &mut num as *mut i32;
}
```

### Call an unsafe function or method

See the code inside [raw pointer file](./src/raw_pointer.rs) 

```rust
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        ( slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

### Using extern Functions to Call external code

Sometimes we need to call code from another language, for this Rust has the keyword `extern` that facilitates the creation and use of a _Foreign Function Interface_(FFI).

```rust
extern "C" {
  fn abs(input: i32) -> i32;
}

fn main() {
  unsafe {
    println!("Absolute value of -3 according to C is: {}", abs(-3));
  }
}
```

Here, the `"C"` part defines which _Application Binary Interface_(ABI) the external function uses: the ABI defines how to call the function at the assembly level.

```rust
// Mangling is when the compiler changes the name of the function for another one, which is not human readable, so if we want to extern
// functions from Rust to other languages, we need to use these features.
#[no_mangle]
pub extern "C" fn call_from_c() {
  println!("Just called a Rust function from C!");
}
```

### Accessing or Modifying a Mutable Static Variable

When dealing with _global variables_, it can be ownership problematic, if two threads try to access the same mutable global variable, it can cause _data races_.

All the static variable belongs to the `static` lifetime. Their value are always inside the same fixed memory allocation. We can only update a mutable static variable inside an unsafe block.

### Implementing an Unsafe trait

TODO
 
## Reference

- https://doc.rust-lang.org/nomicon/safe-unsafe-meaning.html
