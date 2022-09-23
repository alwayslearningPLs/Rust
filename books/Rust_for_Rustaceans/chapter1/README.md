# Basic Rust

## Memory

Programs have access to:

- stack
- heap
- registers
- text segments
- memory-mapped registers
- memory-mapped files
- nonvolatile RAM (perhaps)

### Memory terminology

What is the difference between *values*, *variables*, and *pointers*?

A *value* in Rust is the combination of a *type* and an *element* of that type's domain of values.

A value can be turned into a *sequence of bytes* using its type's *representation*.

eg. `6` in the type `u8` is an instance of the mathematical integer `6`, and its memory representation in bytes would be `0x06`.

A value is stored in a *place*, which is the Rust terminology for "a location that can hold a value". This place can be on the *stack*, on the *heap*, or in a number of other locations.

A *pointer* is a value that holds the address of a *region of memory*, so the pointer points to a place.

### Variables in depth

high-level models and low-level models

#### High level model

It is useful when thinking about code at the level of *lifetimes* and *borrows*.

The compiler can check that at any given point in your program, all flows that can exist in parallel with each other are compatible.

For example, there cannot be two parallel flows with *mutable access* to a value. Nor can there be a flow that borrows a value while there is no flow that owns the value.

```rust
let mut x;
// this is illegal because of the last statement.
// We are creating a flow from x, trying to borrow the value of that variable while there is no flow that owns that value.
// assert_eq!(x, 42); 

x = 42;             //1

let y = &x;         //2

x = 43;             //3

assert_eq!(*y, x);  //4
```

The flow from 1-3 is *exclusive* (&mut) and the flow from 1-4 is *shared* (&).

You cannot have an *exclusive* and a *shared* use of a value at the same time. (3-4)

#### Low level model

It is good when you are reasoning about *unsafe code* and *raw pointes*.

### Memory regions

The three most important regions for the purposes of writing Rust code are the *stack*, the *heap* and *static memory*.

#### Stack

The stack is a segment of memory that your program uses as a scratch space for function calls.

Each time a function is called, a contiguous chunk of memory called a *frame* is allocated at the top of the stack.

#### Heap

The heap is a pool of memory that isn't tied to the current call stack of the program.

If you forget to deallocate heap memory, it will stick around forever, and your application will eventually eat up all the memory on your machine.
This is called *leaking memory* and it is something that you want to avoid. However, there are some cases where you explicitly want to *leak memory*.
For example, say you have a *read-only* configuration that the entire program should be able to access. You can allocate that on the heap and explicitly
leak it with `Box::leak` to get a `'static` reference to it.

#### Static

It is really a catch-all term for several closely related regions located in the fie your program is compiled into.

Values in static memory live for the entire execution of your program.

Your program's static memory contains the program's binary code, which is usually mapped as *read-only*.

You will encounter the `'static` lifetime much more often than you will encounter truly static memory. This is because `'static` often shows up in trait bounds on type parameters. A bound like `T: 'static` indicates that the type parameter T is able to live for however long we keep it around for, up to and including the remaining execution of the program.

## Ownership

Rust's memory model centers on the idea that all values have a single *owner* - that is, exactly one location (usually a scope) is reponsible for ultimately deallocating each value.

A variable that holds a reference to another variable does not own that other variable, so the values isn't dropped when the variable drops.

The rules for the order in which to drop variable that go out of scope are fairly simple:

- Variables (including function arguments) are dropped in reverse order.
- Nested values are dropped in source-code order. (structs, arrays, tuples)

Why in reverse order? Because if you declare a String and after that a HashTable containing that String, if we remove the String first, when the HashTable goes to remove the String that you have inserted, it will fail. Because of the lifetime rules of Rust, we cannot assign a variable to another one, without initializing it.

## Borrowing and lifetimes

Rust allows the owner of the value to lend out that value to others, without giving up ownership, through references. *References* are pointers that come with an additional contract for how they can be used.

### Shared references

A *shared reference*, `&T`, is, as the name implies, a pointer that may be shared. Any number of others references my exists to the same value, and each shared reference is *Copy*, so you can trivially make more of them.

You cannot modify the value a shared reference points to, nor can you cast a shared reference to a mutable one.

### Mutable references



