# Compiler 
## c like lang (but worse) 

### Motivation
I like c, but i also hate c
---

- I dont know assembly or how to codegen, so it just transpiles to c for now.

- Implemented (kinda):
  - program return
  - const variable declaration
    - i32
    - bool
    - char
  - addition
  - functions

- Plans for the future:
  - function calls
  - conditionals
  - for-loops
  - scopes
  - enfore compile time mutability
  - types
    - bool
    - char
    - string
    - i8,i16,i64
    - u8,u16,u32,u64
    - f32, f64
    - ptrs ????
  - structs
  - enums
  - syscalls

- big motivations but maybe too difficult 
  - convert c transpiler to assembly compiler (llvm or raw)
  - pattern matching
  - iterators but simpler
  - traits but simpler
  - pipe operator
  - borrow checker (for mutable stuff)
  - lifetimes (if ptrs are impl)
