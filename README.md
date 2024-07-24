# Compiler-1
just me learning to write a compiler, planning to make it a 'c like' lang (but worse) 

### Motivation
I like c, but i also hate c

### its actually a transpiler to c
I don't know assembly or how to codegen (YET!!!), so it just transpiles to c for now.

### Implemented (kinda):
  - int main()
  - program returns
  - addition
  - mutable variable declaration (i32 only)

### Plans for the future:
  - remove deps [clap]
  - other functions
  - function calls
  - function args
  - conditionals
  - for-loops
  - variable scopes
  - enfore compile time const 
  - native types
    - bool
    - char
    - string
    - `html` like php
    - i8,i16,i64
    - u8,u16,u32,u64
    - f32, f64
    - safe ptrs
  - structs
  - enums
  - syscalls

### Big motivations but maybe too difficult 
  - convert c transpiler to assembly compiler (llvm ir or raw arm)
  - pattern matching would be cool
  - iterators built in ? 
  - traits but simpler
  - pipe operator
  - borrow checker (for mutable stuff)
  - lifetimes (if ptrs are impl)
