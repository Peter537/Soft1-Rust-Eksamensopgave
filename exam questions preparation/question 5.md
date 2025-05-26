## Question 5

Memory Management: Memory management is handled differently in Rust compared to other languages like C++, C# or Java.
Describe how Rust’s memory management principles affected the way you structured your project.
Include examples of how you used Box, Rc, or RefCell in managing heap data.

## How it's done in Rust

- In Rust, each value has a single owner, and memory is automatically freed when the owner goes out of scope. Manual memory management is not needed.
- Smart pointers like `Box`, `Rc`, `Arc`, and `Mutex` are used for heap allocation and shared ownership.
- Heap allocation is used for large or recursive data structures, while the stack is used for small, fast data.
- `Box` provides heap allocation with single ownership, useful for storing data of unknown or varying size at compile time, such as different UI screens.
- Developers don’t manually free memory; Rust’s compiler and runtime handle memory management safely and efficiently.

Common smart pointers in Rust:

- `Box`: Single ownership, heap allocation, and automatic cleanup. Used for large or recursive data structures, such as UI screens or linked lists.
- `Rc`: Reference counting for shared ownership in single-threaded code. Useful when multiple parts of your program need access to the same data without taking ownership.
- `Arc`: Like `Rc`, but thread-safe for shared ownership across threads. Often used in multi-threaded applications where data needs to be shared safely.
- `Mutex`: Provides thread-safe, mutable access to data with locking, as shown in the `CONNECTION` example. Ensures only one thread can access the data at a time.
- `RefCell`: Enables interior mutability in single-threaded code, allowing you to mutate data even when it is otherwise immutable, with borrow checking enforced at runtime.


### Compared to other languages

In languages like C++ or C#, memory management is often manual or relies on garbage collection. Rust's ownership model eliminates the need for manual memory management and reduces the risk of memory leaks and dangling pointers.

Rust does a far better job at ensuring that memory is freed when it is no longer needed, and it does so without the overhead of garbage collection.
> Like Steven once said; "Trust the Rust compiler, it knows what it's doing."
* No manual cleanup is needed.

The only cleanup i would do is perhaps using scopes to manually drop an item for the purpose of freeing its lock-state when it is a Mutex, but that is not necessary in most cases.

### My view

^

## Code Snippets

1. We use `Mutex` as a smart pointer to safely store heap data across multiple threads.

File: [`src/database/connection.rs`](../src/database/connection.rs)
```rust
static CONNECTION: Mutex<Option<Connection>> = Mutex::new(None);
```

2. Atomic types are used for lock-free data sharing between threads. Here, `AtomicU16` allows us to update data across threads without a `Mutex`.

`src/database/mod.rs` : line 12

```rust
static GAME_NUMBER: AtomicU16 = AtomicU16::new(0);
```

3. `Box` is used to return UI screens from our lambda function, since their sizes may differ at compile time. We use dynamic dispatch with a trait object (`Box<dyn Widget<AppState>>`) to return the correct screen type.

File: [`src/ui/mod.rs`](../src/ui/mod.rs)
```rust
pub fn build_ui() -> impl druid::Widget<AppState> {
    ViewSwitcher::new(
        |data: &AppState, _env| (data.current_screen.clone(), data.game_number.clone()),
        |(screen, _game_number), _data, _env| -> Box<dyn druid::Widget<AppState>> {
            match screen {
                Screen::Loading => Box::new(loading_screen::build_screen()),
                // ...
                Screen::RaceScheduleScreen => {
                    Box::new(with_navbar(race_schedule_screen::build_screen()))
                }
            }
        },
    )
}
```

## Other Examples

- `Rc` is used for shared ownership of data in a single thread. It keeps track of the number of references, and memory is freed when there are no more references.

```rust
use std::rc::Rc;

fn main() {
    let data = Rc::new(vec![1, 2, 3]);
    let data_clone = Rc::clone(&data);

    println!("Original: {:?}", data);
    println!("Clone: {:?}", data_clone);
    println!("Reference count: {}", Rc::strong_count(&data));
}
```

- `RefCell` enables interior mutability, allowing you to mutate data even when it is otherwise immutable. This is useful when compile-time checks are too restrictive.

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(vec![1, 2, 3]);
    data.borrow_mut().push(4);
    println!("Data: {:?}", data.borrow());
}
```
