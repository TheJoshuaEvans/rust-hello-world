# Rust Hello World

Going through the [intro docs](https://doc.rust-lang.org/rust-by-example/index.html)

# Progress
- [X] 1.1. Comments
- [X] 1.2. Formatted Print
  - [X] 1.2.1. Debug
  - [X] 1.2.2. Display
  - [X] 1.2.3. Formatting
- [X] 2. Primitives
  - [X] 2.1. Literals and operators
  - [X] 2.2. Tuples
  - [X] 2.3. Arrays and Slices
- [X] 3. Custom Types
  - [X] 3.1. Structures
  - [ ] 3.2. Enums
  - [ ] 3.3. constants
- [ ] 4. Variable Bindings
  - [ ] 4.1. Mutability
  - [ ] 4.2. Scope and Shadowing
  - [ ] 4.3. Declare first
  - [ ] 4.4. Freezing
- [ ] 5. Types
  - [ ] 5.1. Casting
  - [ ] 5.2. Literals
  - [ ] 5.3. Inference
  - [ ] 5.4. Aliasing
- [ ] 6. Conversion
  - [ ] 6.1. From and Into
  - [ ] 6.2. TryFrom and TryInto
  - [ ] 6.3. To and from Strings
- [ ] 7. Expressions
- [ ] 8. Flow of Control
  - [ ] 8.1. if/else
  - [ ] 8.2. loop
    - [ ] 8.2.1. Nesting and labels
    - [ ] 8.2.2. Returning from loops
  - [ ] 8.3. while
  - [ ] 8.4. for and range
  - [ ] 8.5. match
    - [ ] 8.5.1. Destructuring
      - [ ] 8.5.1.1. tuples
      - [ ] 8.5.1.2. arrays/slices
      - [ ] 8.5.1.3. enums
      - [ ] 8.5.1.4. pointers/ref
      - [ ] 8.5.1.5. structs
    - [ ] 8.5.2. Guards
    - [ ] 8.5.3. Binding
  - [ ] 8.6. if let
  - [ ] 8.7. while let
- [ ] 9. Functions
  - [ ] 9.1. Methods
  - [ ] 9.2. Closures
    - [ ] 9.2.1. Capturing
    - [ ] 9.2.2. As input parameters
    - [ ] 9.2.3. Type anonymity
    - [ ] 9.2.4. Input functions
    - [ ] 9.2.5. As output parameters
    - [ ] 9.2.6. Examples in std
      - [ ] 9.2.6.1. Iterator::any
      - [ ] 9.2.6.2. Searching through iterators
  - [ ] 9.3. Higher Order Functions
  - [ ] 9.4. Diverging functions
- [ ] 10. Modules
  - [ ] 10.1. Visibility
  - [ ] 10.2. Struct visibility
  - [ ] 10.3. The use declaration
  - [ ] 10.4. super and self
  - [ ] 10.5. File hierarchy
- [ ] 11. Crates
  - [ ] 11.1. Creating a Library
  - [ ] 11.2. Using a Library
- [ ] 12. Cargo
  - [ ] 12.1. Dependencies
  - [ ] 12.2. Conventions
  - [ ] 12.3. Tests
  - [ ] 12.4. Build Scripts
- [ ] 13. Attributes
  - [ ] 13.1. dead_code
  - [ ] 13.2. Crates
  - [ ] 13.3. cfg
    - [ ] 13.3.1. Custom
- [ ] 14. Generics
  - [ ] 14.1. Functions
  - [ ] 14.2. Implementation
  - [ ] 14.3. Traits
  - [ ] 14.4. Bounds
    - [ ] 14.4.1. Testcase: empty bounds
  - [ ] 14.5. Multiple bounds
  - [ ] 14.6. Where clauses
  - [ ] 14.7. New Type Idiom
  - [ ] 14.8. Associated items
    - [ ] 14.8.1. The Problem
    - [ ] 14.8.2. Associated types
  - [ ] 14.9. Phantom type parameters
    - [ ] 14.9.1. Testcase: unit clarification
- [ ] 15. Scoping rules
  - [ ] 15.1. RAII
  - [ ] 15.2. Ownership and moves
    - [ ] 15.2.1. Mutability
    - [ ] 15.2.2. Partial moves
  - [ ] 15.3. Borrowing
    - [ ] 15.3.1. Mutability
    - [ ] 15.3.2. Aliasing
    - [ ] 15.3.3. The ref pattern
  - [ ] 15.4. Lifetimes
    - [ ] 15.4.1. Explicit annotation
    - [ ] 15.4.2. Functions
    - [ ] 15.4.3. Methods
    - [ ] 15.4.4. Structs
    - [ ] 15.4.5. Traits
    - [ ] 15.4.6. Bounds
    - [ ] 15.4.7. Coercion
    - [ ] 15.4.8. Static
    - [ ] 15.4.9. Elision
- [ ] 16. Traits
  - [ ] 16.1. Derive
  - [ ] 16.2. Returning Traits with dyn
  - [ ] 16.3. Operator Overloading
  - [ ] 16.4. Drop
  - [ ] 16.5. Iterators
  - [ ] 16.6. impl Trait
  - [ ] 16.7. Clone
  - [ ] 16.8. Supertraits
  - [ ] 16.9. Disambiguating overlapping traits
- [ ] 17. macro_rules!
  - [ ] 17.1. Syntax
    - [ ] 17.1.1. Designators
    - [ ] 17.1.2. Overload
    - [ ] 17.1.3. Repeat
  - [ ] 17.2. DRY (Don't Repeat Yourself)
  - [ ] 17.3. DSL (Domain Specific Languages)
  - [ ] 17.4. Variadics
- [ ] 18. Error handling
  - [ ] 18.1. panic
  - [ ] 18.2. abort & unwind
  - [ ] 18.3. Option & unwrap
  - [ ] 18.3.1. Unpacking options with ?
  - [ ] 18.3.2. Combinators: map
  - [ ] 18.3.3. Combinators: and_then
  - [ ] 18.3.4. Defaults: or, or_else, get_or_insert, 'get_or_insert_with`
  - [ ] 18.4. Result
  - [ ] 18.4.1. map for Result
  - [ ] 18.4.2. aliases for Result
  - [ ] 18.4.3. Early returns
  - [ ] 18.4.4. Introducing ?
  - [ ] 18.5. Multiple error types
  - [ ] 18.5.1. Pulling Results out of Options
  - [ ] 18.5.2. Defining an error type
  - [ ] 18.5.3. Boxing errors
  - [ ] 18.5.4. Other uses of ?
  - [ ] 18.5.5. Wrapping errors
  - [ ] 18.6. Iterating over Results
- [ ] 19. Std library types
  - [ ] 19.1. Box, stack and heap
  - [ ] 19.2. Vectors
  - [ ] 19.3. Strings
  - [ ] 19.4. Option
  - [ ] 19.5. Result
    - [ ] 19.5.1. ?
  - [ ] 19.6. panic!
  - [ ] 19.7. HashMap
    - [ ] 19.7.1. Alternate/custom key types
    - [ ] 19.7.2. HashSet
  - [ ] 19.8. Rc
  - [ ] 19.9. Arc
- [ ] 20. Std misc
  - [ ] 20.1. Threads
  - [ ] 20.1.1. Testcase: map-reduce
  - [ ] 20.2. Channels
  - [ ] 20.3. Path
  - [ ] 20.4. File I/O
    - [ ] 20.4.1. open
    - [ ] 20.4.2. create
    - [ ] 20.4.3. read lines
  - [ ] 20.5. Child processes
    - [ ] 20.5.1. Pipes
    - [ ] 20.5.2. Wait
  - [ ] 20.6. Filesystem Operations
  - [ ] 20.7. Program arguments
    - [ ] 20.7.1. Argument parsing
  - [ ] 20.8. Foreign Function Interface
- [ ] 21. Testing
  - [ ] 21.1. Unit testing
  - [ ] 21.2. Documentation testing
  - [ ] 21.3. Integration testing
  - [ ] 21.4. Dev-dependencies
- [ ] 22. Unsafe Operations
  - [ ] 22.1. Inline assembly
- [ ] 23. Compatibility
  - [ ] 23.1. Raw identifiers
- [ ] 24. Meta
  - [ ] 24.1. Documentation
  - [ ] 24.2. Playground
