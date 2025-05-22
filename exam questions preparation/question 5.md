## Question 5

Memory Management: Memory management is handled differently in Rust compared to other languages like C++, C# or Java.
Describe how Rust’s memory management principles affected the way you structured your project.
Include examples of how you used Box, Rc, or RefCell in managing heap data.

### Links

### Code Snippets

1. Vi bruger Mutex som en smart pointer til at gemme heap data safely på flere tråde.

`src/database/connection.rs` : linje 7

```rust
static CONNECTION: Mutex<Option<Connection>> = Mutex::new(None);
```

2. Atomic types er brugt til lock-free data. Det er en smart pointer der kan bruges til at gemme data på flere tråde uden at bruge Mutex.

`src/database/mod.rs` : linje 12

```rust
static GAME_NUMBER: AtomicU16 = AtomicU16::new(0);
```

### Additional Information

- I Rust, så når en variabel går ud af scope, så bliver hukommelsen automatisk frigivet. Det er ikke nødvendigt at bruge `delete` som i C++ eller `Dispose` i C#.
- Smart pointers som `Box`, `Rc`, `Arc` og `Mutex` bliver brugt til heap-allokeret data og shared ownership.
