# rust-cheat-sheet-code

## Cheat sheet for Rust in a module

I thought code in a cheat sheet file on-line named 'LGR Cheat Sheet' would be nice in one module rather than a Word document or PDF.

## Credits

i.e. Let's Get Rusty Version 1.0.7 from these person(s) https://www.youtube.com/c/LetsGetRusty

## Dependencies

None. Added missing glue code so all example cheat sheet Rust code compiles and runs.

## 79 Functions Covering

### Basic Types & Variables
- Boolean
- Unsigned integers
- Signed integers
- Floating point numbers
- Platform specific integers
- Unicode Char String
- Tuple
- Array & Slice
- `HashMap`{:.rust}
- `struct`{:.rust}
- `enum`{:.rust}
- Constant
- `static`{:.rust} variable
- Mutability
- Shadowing
- Type alias

### Control Flow
- `if`{:.rust} and `if let`{:.rust}
- loop
- Nested loops & labels
- Returning from loops
- while and while let
- for loop
- match

### References, Ownership, and Borrowing
- Creating references
- Copy, Move, and Clone
- Ownership and functions

### Pattern Matching
- Basics
- Destructuring
- Ignoring values
- Match guards
- @ bindings

### Iterators
- Usage
- Implementing the Iterator trait

### Error Handling
- Throw unrecoverable error
- Option enum
- Result enum
- ? operator

### Combinators
- .map
- .and_then

### Multiple error types
- Define custom error type
- Boxing errors

### Iterating over errors
- Ignore failed items with filter_map()
- Fail the entire operation with collect()
- Collect all valid values and failures with partition()

### Generics, Traits, and Lifetimes
- Using generics
- Defining traits
- Default implementations with Derive
- Trait bounds
- impl trait
- Trait objects
- Operator overloading
- Supertraits
- Lifetimes in function signatures
- Lifetimes in struct definitions
- Static lifetimes

### Functions, Function Pointers & Closures
- Associated functions and methods
- Function pointers
- Creating closures
- Returning closures
- Closure traits
  - FnOnce
  - FnMut
  - Fn
- Store closure in struct
- Function that accepts closure or function pointer

### Pointers
- References
- Raw pointers

### Smart pointers

- Box<T>
- Rc<T>
- Ref<T>, RefMut<T>, and RefCell<T>
- Multiple owners of mutable data
