# Rust stdlib

## System resources

- User space programs: Editor, terminal, ssh, firewall.
- Rust stdlib (Cross-platform API for _syscalls_).
- _libc_ (Platform-specific _syscall_ wrapper APIs).
- Kernel is the core module of OS (System call API interface).
  + _kernel_ and _user_ mode.
- System resources:
  + Memory.
  + File system.
  + CPU.
  + Network.
  + Other devices (Keyboard, mouse, monitor, disk drive).

## Exploring the Rust stdlib

- std is cross-platform.
- std is available to all Rust crates by default.
- std includes operations on standard Rust primitives. eg. `std::i8::MAX`
- It provides functionality for operations such as:
  + data manipulation
  + memory allocation
  + error handling
  + networking
  + I/O
  + concurrency
  + async I/O primitives
  + FFI (Foreign Function Interface).

The Rust stdlib is broadly organized as follows:

- Rust language primitives
- The _Core crate_: It acts as the link between the Rust language and the stdlib. The _core crate_ can be used independently, is not platform-specific, and does not have any links to OS sys libraries or other external libraries. You can instruct the compiler to compile without the Rust Stdlib and use the core crate instead (annotated with `#![no_std]`)
- The _Alloc crate_ contains all related to memory allocation for heap-allocated values. It includes :
  + _smart pointer types_ such as `Box`(`Box<T>`).
  + _reference-counted pointers_ (`Rc<T>`).
  + _atomically reference-counted pointers_ (`Arc<T>`)

## Rust stlib modules

- Concurrency:
  + env
  + sync
  + process
  + thread
- Memory management:
  + alloc
  + convert
  + ptr
  + borrow
  + default
  + rc
  + cell
  + mem
  + clone
  + pin
- file system:
  + fs
  + path
- Async:
  + future
  + task
- Data processing:
  + ascii
  + fmt
  + num
  + cmp
  + hash
  + ops
  + iter
- Error handling
  + error
  + panic
  + option
  + result
- Compiler:
  + hint
  + primitive
  + prelude
- ffi:
  + ffi
- networking:
  + net
- IO:
  + io
- os-specific:
  + os
- time-related:
  + time
- data types:
  + any
  + array
  + char
  + collections
  + marker
  + string
  + slice
  + str
  + vec
  + f32
  + f64
  + i8
  + u8
  + i16
  + u16
  + i32
  + u32
  + i64
  + u64
  + i128
  + u128
  + isize
  + usize
