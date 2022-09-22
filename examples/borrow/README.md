# Borrow checker and ownership

The compiler guarantee memory safety without an external garbage collector.

By definition, every resource has a _lifetime_ and an _owner_ associated with it, which
operates under the rules:

- Each resource has exactly _one owner_ at any point in time.
  + By default, the owner is the variable that created that resource
  + and its lifetime is the lifetime of the enclosing scope.
  + Others can borrow or copy the resource if they need to.
- When the owner's scope has finished executing, all resources owner by it will be dropped. This is statically computed by the compiler, which then produces machine code accordingly.

