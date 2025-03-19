SDP 25-03-20

## Typesafe State Machines in Rust
 - Hello my name is...
   - I am new to Agreat...


### 10000-feet overview of Rust
 - Who here have heard of Rust? Who has written Rust?
 - Systems programming language with a focus on memory safety, a major source of security bugs
 - Modern ecosystem through Cargo, which is a build system, package manager, and much more
 - Powerful zero-cost abstractions enable fearless concurrency, even at very complex scales
 - The compiler is very strict
   - Memory safety is achiev∑ed through ownership of data, and a borrow checker that ensures references do not live longer than intended
   - Mutability is never implied, all mutable variables have to be declared with the `mut` keyword
   - Much like functional languages like Haskell, code that compiles is always safe, and often correct
 - I will also highlight some of the nicer features of Rust in the code, which will be available afterwards

#### The Borrow Checker
 - Underpins much of what we are going to look at today
 - There are three different bindings
 - owned, ref, mut ref
 - Some versions are automatically coerced, but only when it is 100% safe to do so

### State Machines
 - State machines are a powerful concept for encoding real-world systems
 - Many systems are built on top of a state machine, or can be easily modelled as such
   - Network connections
   - Regular expressions
   - Messages/order systems
   - Turn based gamesß
   - Traffic lights
   - Scene management in games
 - Naive implementations often have runtime overhead or are easy to misuse
 - It would be nice if we could let the compiler take care of it!
   - Preferrably with minimal runtime overhead and zero memory cost

### Our State Machine
 - Toy model of a rocket launch
   - Could be extended with many more states (like re-entry), but that is not the point
 - Very important that the actions are only carried out in the correct state
   - We can only launch when we are on the ground
   - We can always hail the spacecraft on the radio

### A Naive Implementation
 - State is stored as an enum value
   - Another nice feature of Rust are data fields on enum variants
 - Each function must check its preconditions, and panic at runtime if they are not met
   - We could use Result or Option here, but that is not the point of this example
   - Easy to miss preconditions, and `&mut` everywhere
 - Can anyone see the bug in the `dock()` function?
   - We do not check the state of the other Spacecraft
 - Can anyone see the bug in the `undock()` function?
   - We overwrite the state first, even if the state was not correct!

### Rust to the Rescue
 - Encode the state as a type parameter instead
   - The states are no longer values, but are instead types
   - We store the state
     - Most state structs are 0-sized and they will be completely erased at runtime
     - If all states were 0-sized, one could use a `PhantomData`
 - Rust allows specialized `impl`-blocks for different type parameters, grouping related code together
   - It is no longer possible to call a function on a different state, checked at runtime
   - This is also understood by IDEs, which doesn't even suggest invalid functions
   - Functions can be applied to more than one state via marker traits (`CanPerformScience`)

### Limitations and workarounds:
 - No undeterministic state transitions
 - Slightly more code, but it is a linear overhead that would disappear with longer method bodies
 - No shared references to the state machine, due to state transitions requiring ownership
   - This also means you cannot store a list of all spacecraft, due to them being different types
 - The executable becomes slightly bigger, as each specialization is generated and optimized independently

### Advanced concepts:
 - So where do we go from here?
 - The example is written with synchronous code, but the concept can be applied in async contexts as well
 - Also possible to have multiple orthogonal states, enabling even more complicated machines to be modeled
   - However, this makes the code considerably more complicated to write
   - Would not use more than two state type parameters
 - This is essentially compile-time dependency injection, but with state
