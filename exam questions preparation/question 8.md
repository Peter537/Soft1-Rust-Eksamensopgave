## Question 8

Module System and Code Organization: Rust’s module system aids in organizing large codebases.
How did you utilize this system to structure your project’s code?
Explain your use of pub, mod, and other visibility qualifiers to manage encapsulation and modularity.

## Links

## How it's done in Rust

- In Rust, a `crate` is a compilation unit—either a binary or a library—with its root in `main.rs` (for applications) or `lib.rs` (for libraries).
- We organized our code into modules to improve maintainability and navigation.
- The `pub` keyword controls visibility, making items public as needed, while everything is private by default, encouraging thoughtful API design.
- The `mod` keyword defines a module, and `super` allows access to the parent module.
- `pub use` enables re-exporting structs and functions, simplifying the public API and avoiding naming conflicts.
- Private methods can be accessed in tests using the `#[cfg(test)]` attribute, provided tests are in the same or a dedicated test module.

### Compared to other languages
In Java, `super` refers to the superclass, while in Rust, `super` refers to the parent module.

Java uses package-private visibility by default. The four visibility levels in Java are:

- `public`: Accessible everywhere.
- `protected`: Accessible within the same package and by subclasses (even in other packages).
- *package-private* (no modifier): Accessible only within the same package.
- `private`: Accessible only within the same class.

### My view

I feel like i most often try to make things private by default when i code as i dont need most tools more than in one place. This is especially the case for OOP. With that said i do enjoy how Rust has most things private by default, and you have to explicitly make them public if you want to use them in other modules. This makes it easier to keep track of what is public and what is private, and it helps avoid accidental naming conflicts.

## Code Snippets

1. **Organizing Modules with `mod.rs` and Visibility**

In [`src/database/mod.rs`](../src/database/mod.rs ), we define which submodules are public or private. For example:

```rust
pub mod circuit;
pub mod config;
mod connection;
pub mod country;
pub mod driver;
pub mod race;
pub mod teams;
```

Here, `connection` is private because it is only used internally within the `database` module. The other modules are public so they can be accessed from outside.

2. **Re-exporting Structs for a Clean API**

In our `model` module, files are private by default, and we only re-export the necessary structs. This keeps the API clean and avoids naming conflicts. For example, in [`src/model/mod.rs`](../src/model/mod.rs):

```rust
mod circuit;
// ...
pub use team::TeamBase;
```

This way, other modules can use `TeamBase` directly, without needing to know which file it comes from.

3. **Using `pub(super)` for UI Components**

For UI components, we could have used `pub(super)` to make components visible only to their parent module, since they are only relevant within the UI context. This would help encapsulate them and prevent accidental usage elsewhere. For example, instead of:

```rust
pub mod goto;
pub mod table;
```

We could use:

```rust
pub(super) mod goto;
pub(super) mod table;
```

4. **Re-exporting for a Cleaner API**

Re-exporting items can simplify your public API. For example:

```rust
// lib.rs
mod utils;
pub use utils::math::add;

// Usage in other files:
fn main() {
    let sum = add(5, 7);
    println!("Sum: {}", sum);
}
```

5. **Visibility Modifiers in Rust**

- `pub`: Public everywhere.
- `pub(crate)`: Visible within the entire crate.
- `pub(super)`: Visible to the parent module.
- `pub(in path)`: Visible only within a specific module path.

Example:

```rust
mod outer {
    pub(crate) fn crate_visible_function() {
        println!("Visible within the crate.");
    }

    pub mod inner {
        pub(super) fn parent_visible_function() {
            println!("Visible to the parent module.");
        }

        pub(in crate::outer) fn specific_path_visible_function() {
            println!("Visible within crate::outer.");
        }
    }
}
```
