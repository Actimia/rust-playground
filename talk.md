## Typesafe State Machines in Rust
 - Hello my name is...
   - I am new to Agreat...
 - State machines are a powerful concept for encoding real-world systems
 - Many systems are built on top of a state machine
   - Network connections (TCP)
   - Regular expressions
   - Messages/order systems (Draft, Sent, Read)
 - Naive implementations often have runtime overhead or are easy to misuse
 - It would be nice if we could let the compiler take care of it!
   - Preferrably with minimal runtime overhead and zero memory cost

### 10000-feet overview of Rust
 - Who here have heard of Rust? Who has written Rust?
 - Systems programming language with a focus on memory safety, a major source of security bugs
 - Modern ecosystem through Cargo, which is a build system, package manager, and much more
 - Powerful zero-cost abstractions enable fearless concurrency, even at very complex scales
 - The compiler is very strict
   - Much like functional languages like Haskell, code that compiles is always safe, and often correct
   - Memory safety is achieved through ownership of data, and a borrow checker that ensures references do not live longer than intended
 - Mutability is never implied, all mutable variables have to be declared with the `mut` keyword

### Our State Machine
 - Diagram
 - Toy model of a rocket launch
   - Actions can only be performed in certain states
     - Very important that the actions are carried out in the correct order
     - We can always hail the spacecraft on the radio
     - We can only launch when we are on the ground

### A Naive Implementation
 - State as an enum
   - Another nice feature of Rust is data fields on enum variants
 - Each function must check its preconditions, and panic at runtime if they are not met
   - We could use Result or Option here, but that is not the point of this example
   - Easy to miss preconditions, and `&mut` everywhere
   - Can anyone see the bug in the `dock()` function?
     - We do not check the state of the other Spacecraft
   - Can anyone see the bug int he `undock()` function?
     - We overwrite the state first, even if the state was not correct!

### Rust to the Rescue
 - Encode the state as a type parameter instead
   - The states are no longer values, but are instead types
   - We store the state, but most state structs are 0-sized and they will be completely erased at runtime
 - Rust allows us to specialize `impl`-blocks for different type parameters, grouping related code together
   - It is no longer possible to call a function on a different state, checked at runtime
 - Limitations and workarounds:
   - All transitions needs to be deterministic
   - Cannot simply store a list of spacecraft in a list, due to them being different types
   - Functions can be applied to more than one state via marker traits (`CanPerformScience`)
 - This is essentially dependency injection, but with state
