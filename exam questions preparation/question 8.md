## Question 8

Module System and Code Organization: Rust’s module system aids in organizing large codebases.
How did you utilize this system to structure your project’s code?
Explain your use of pub, mod, and other visibility qualifiers to manage encapsulation and modularity.

## Links

## How it's done in Rust

- Vi har opdelt vores kode i moduler for at gøre det lettere at finde og vedligeholde koden.
- Man bruger `pub` til at kontrollere synligheden af hvad man vil have skal blive vist til offentligheden.
- Når alt bliver privat fra starten, så gør det man kommer til at tænke mere over strukturen af ens kode ift. hvad der skal være offentligt og hvad der ikke skal.
- Man kan bruge `super` til at gå op til parent modulet

### Compared to other languages

I Java og C# er ting public by default.

### My view

Jeg synes det er meget godt at Rust har det sådan at alt er privat som standard, fordi det gør man skal tænke mere over strukturen af ens kode ift. hvad der skal være offentligt og hvad der ikke skal.

## Code Snippets

1. Vi bruger mod.rs til at organisere vores moduler ift. visbility.

`src/database/mod.rs` : linje 4 - 10

```rust
pub mod circuit;
pub mod config;
mod connection;
pub mod country;
pub mod driver;
pub mod race;
pub mod teams;
```

Connection er privat, siden den kun skal bruges internt i database modulen.

2. I vores model modul er filerne private, og vi eksponerer kun structs, det gør det lettere at holde styr på hvad der er offentligt og hvad der ikke er, samt at undgå navnekonflikter. Det gør det også nemt at finde de structs man skal bruge i andre moduler, fordi man ikke skal finde hvilken fil de ligger i.

`src/model/mod.rs` : linje 1 - 20

```rust
mod circuit;
...

...
pub use team::TeamBase;
```

## Other examples

- Det er godt at re-export for at få en cleaner API (ligesom vi har gjort i vores model modul). Andet eksempel kunne være:

```rust
// lib.rs
mod utils;
pub use utils::math::add;

// så kan andre:
fn main() {
    let sum = add(5, 7);
    println!("Sum: {}", sum);
}
```

- Eksempel på Visibility modifiers:

`pub` er den mest almindelige synlighed, og den gør et element tilgængeligt for alle moduler.
`pub(crate)` gør et element tilgængeligt for hele crate'en, hvilket er nyttigt for at eksponere funktionalitet internt uden at gøre det til en del af den offentlige API.
`pub(super)` gør et element synligt for sin overordnede modul, hvilket er nyttigt til at dele funktionalitet mellem søskende moduler.
`pub(in path)` begrænser synligheden til en specifik modulsti, hvilket giver finjusteret kontrol over, hvor et element er tilgængeligt.

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
