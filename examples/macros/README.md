# Macros in Rust

## Debug a Syntactic macro

This is a nightly feature

```rust
#!feature(trace_macros)]
trace_macros!(true);
```

We can also expand the expansion of a macro using

`rustc -Z unstable-options --pretty expanded <file-name>.rs`

## Procedural macro


