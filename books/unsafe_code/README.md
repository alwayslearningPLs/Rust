# Unsafe code in Rust

The `unsafe` keyword has two uses:

- To declare the existence of contracts the compiler can't check.
- To declare that a programmer has checked that these contracts have been upheld.

You can use `unsafe` to indicate the existence of unchecked contracts on _functions_ and _trait declarations_.

Some examples:

- `slice::get_unchecked`, which performs unchecked indexing, allowing memory safety to be freely violated.
- `mem::transmute` reinterprets some value as having a give type, bypassing type safety in arbitraty ways

## Reference

- https://doc.rust-lang.org/nomicon/safe-unsafe-meaning.html
