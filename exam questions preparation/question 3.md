## Question 3

Strings and String Handling in Rust: In Rust, the handling of strings is different to languages like Java C# - especially about mutability and ownership. Discuss how you managed strings in your project. What challenges did you face with String and &str types?
Provide examples of where you optimised string usage for performance or memory efficiency (you could discuss the use of `&’static str` or `[A]rc<str>` where lots of cloning is needed)

## How it's done in Rust

- `String` is heap-allocated and mutable, while `&str` is an immutable reference to a string, usually not heap-allocated. `String` can be changed at runtime; `&str` cannot. Using `&str` is often faster and avoids unnecessary cloning as its stored in the stack.
- `&'static str` is a reference to a string literal that lives for the entire duration of the program and cannot be changed.
- Cloning strings is expensive in terms of memory and performance, so prefer `&str` when possible.
- Methods like `split()` and `trim()` on a `String` return `&str` slices, as the result is always the same size or smaller. Methods like `replace()` may return a larger string, so they return a new `String`.

### Compared to other languages
- In both Java and C#, strings are immutable; modifying a string creates a new heap allocation, which can be costly for memory and performance.
- Rust's `String` is also heap-allocated and growable; creating or modifying a `String` may cause new allocations or reallocations.
- Rust provides `&str`, a lightweight, read-only reference to a string slice, usually stored on the stack.
- Java lacks a direct equivalent to string slices.
- C# offers `ReadOnlySpan<char>` for read-only views into strings or arrays, but it is less integrated and less commonly used than Rust's `&str`.

### My view

Rusts string management works similar to Java and C# in that it uses heap-allocated strings, but i love the slice-system that Rust introduces as it lets me look back at text management and wonder "does this even get changed?" and delve into "Should this be a slice instead perhaps?" which is a great way to optimize memory usage and performance.
its fun and not very hard to understand

String is used for unprecise sizes and changable text.
- `&str` is used for precise, read-only text that does not change or sizes that are guaranteed smaller than the original.

## Code Snippets

1. We use a static string for the repository URL, which is a good use case for `&'static str` since it does not change and is known at compile time.

File: [`src/util/appdata.rs`](../src/util/appdata.rs)
```rust
const REPO: &str = "https://raw.githubusercontent.com/Peter537/Soft1-Rust-Eksamensopgave/main";
```

2. We use `String` in structs for runtime data from the database, as ownership is simpler and avoids complex lifetimes. `&str` would require the data to outlive the struct, which is hard to guarantee.

File: [`src/model/driver.rs`](../src/model/driver.rs)
```rust
pub struct Driver {
    pub first_name: String,
    ...
    pub image_path: String,
}
```

File: [`src\ui\mod.rs`](../sc/ui/mod.rs)
```rust
pub selected_team: Option<String> // option but still a string which can change multiple times during runtime. Thus it needs to have a changable size and manage its own memory.
```

3. Our database methods use `&str` (string slice) for efficiency and to avoid unnecessary cloning of data.

File: [`src\database\driver.rs`](../src/database/driver.rs)
```rust
pub fn get_driver_id_by_fullname(full_name: &str) -> Option<u16> {
    ...
}
```