## Question 4

Polymorphism: Traits and Enums in Rust allow for polymorphism.
Can you discuss a place in your project where you implemented polymorphic behavior?
How did this design choice benefit your project? How is the problem of inheritance avoided in Rust? (think about composition - use of structs and impl functions) 

### Links

- https://medium.com/@ksandeeptech07/diamond-problem-in-oop-explained-672d136912c8

### Code Snippets

1. Vi bruger Enums til at repræsentere forskellige skærme i vores UI, og så bruger vi pattern-matching til at håndtere dem.

`src/ui/mod.rs` : linje 40 - 52 og linje 78 - 100

```rust
pub enum Screen {
    Loading,
    ...
    RaceScheduleScreen,
}
```

```rust
match screen {
    Screen::Loading => Box::new(loading_screen::build_screen()),
    ...
    Screen::RaceScheduleScreen => {
        Box::new(with_navbar(race_schedule_screen::build_screen()))
    }
}
```

2. Alle UI builder functions returnerer en `impl Widget<AppState>` type, hvilket gør det muligt at bruge polymorfisme til at håndtere forskellige UI komponenter.

`src/ui/main_screen.rs` : linje 10

```rust
pub fn build_screen() -> impl Widget<AppState> {
    ...
}
```

3. Vi undgår inheritance ved at bruge Composition til de sider der skal have en navbar.

`src/ui/mod.rs` : linje 71 - 76 og 81 - 83

```rust
fn with_navbar(inner: impl Widget<AppState> + 'static) -> impl Widget<AppState> {
    Flex::column()
        .with_child(build_navbar())
        .with_spacer(10.0)
        .with_flex_child(inner, 1.0)
}
```

```rust
Screen::TeamScreen { team_id } => {
    Box::new(with_navbar(team_screen::build_screen(team_id)))
}
```

### Additional Information

- Det er ikke muligt at have inheritance i Rust.
- Rust bruger traits og komposition for at undgå diamond problem og inheritance problemer.
