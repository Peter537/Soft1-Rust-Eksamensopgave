## Question 4

Polymorphism: Traits and Enums in Rust allow for polymorphism.
Can you discuss a place in your project where you implemented polymorphic behavior?
How did this design choice benefit your project? How is the problem of inheritance avoided in Rust? (think about composition - use of structs and impl functions)


## How it's done in Rust

- Rust does not support inheritance.
- Rust values composition over inheirtance.
- Composition improves maintainability and flexibility, as changes in one component do not affect others, and components can be mixed and matched without a base class.
- Enums are best when all possible variants are known at compile time.
- Traits are preferable when new implementations may be added later, similar to interfaces in Java.
- Enums like `Option` and `Result` are used for polymorphism in Rust.

### Compared to other languages

Java is a class-based language that uses inheritance to achieve polymorphism, allowing classes to extend other classes and implement interfaces. This can lead to complex hierarchies and tight coupling between components; and furthermore make relationships between components very explicit.

- Rust, on the other hand, uses enums and traits to achieve polymorphism without inheritance.
> Rust focuses on composition and traits, and not relationships and inheritance.

### My view

- Personally, I prefer the structure of objects in class-based inheritance systems like Java.
- Class inheritance makes relationships between components explicit and easy to understand—e.g., a `Screen` class extended by subscreens like `LoadingScreen`, `MainScreen`, etc.
- With Rust's enum and composition-based design, it works, but feels less intuitive and sometimes fragile, since relationships are not as clearly expressed.
- For example, I can't immediately tell that a `build_screen` function in `loading_screen.rs` represents a screen until I see it used in the `Screen` enum in `mod.rs`.
- In Java, the inheritance signature directly shows what something is, making the codebase easier to navigate and reason about.

## Code Snippets

1. **Enums for UI Screens**  
    Enums represent different screens in the UI, enabling compile-time knowledge of all variants. Pattern matching is used to handle each screen.

    ```rust
    pub enum Screen {
         Loading,
         // ...
         RaceScheduleScreen,
    }

    match screen {
         Screen::Loading => Box::new(loading_screen::build_screen()),
         // ...
         Screen::RaceScheduleScreen => {
              Box::new(with_navbar(race_schedule_screen::build_screen()))
         }
    }
    ```

2. **Trait Objects for Polymorphic Widgets**  
    All UI builder functions return `impl Widget<AppState>`, allowing different UI components to be handled polymorphically. The use of `dyn` in `ViewSwitcher` enables returning different widget types at runtime.

    ```rust
    pub fn build_screen() -> impl Widget<AppState> {
         // ...
    }

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

3. **Composition Instead of Inheritance**  
    Composition is used to add shared UI elements, such as a navbar, to screens. This avoids inheritance and keeps components modular.

    ```rust
    fn with_navbar(inner: impl Widget<AppState> + 'static) -> impl Widget<AppState> {
         Flex::column()
              .with_child(build_navbar())
              .with_spacer(10.0)
              .with_flex_child(inner, 1.0)
    }
    ```

4. **Option and Result for Flexible State Handling**  
    `Option` and `Result` are used to manage state and error handling, such as for database connections.

    ```rust
    static CONNECTION: Mutex<Option<Connection>> = Mutex::new(None);

    pub fn get_connection() -> Result<ConnectionGuard, String> {
         // ...
    }
    ```

These patterns demonstrate how Rust achieves polymorphism and flexible design without inheritance.

## Other examples

- ### Example of using Structs and Methods for composition similar to inheritance in Java:

```rust
struct Engine {
    horsepower: u32,
}

impl Engine {
    fn start(&self) {
        println!("Engine with {} HP started", self.horsepower);
    }
}

struct Car {
    engine: Engine,
    model: String,
}

impl Car {
    fn start(&self) {
        self.engine.start();
        println!("Car model {} is ready to go!", self.model);
    }
}
```

### Example of using Enums and Traits for polymorphism similar to inheritance in Java:

```rust
enum Shape {
    Circle,
    Square,
}

trait Drawable {
    fn draw(&self);
}

impl Drawable for Shape {
    fn draw(&self) {
        match self {
            Shape::Circle => println!("Drawing a circle"),
            Shape::Square => println!("Drawing a square"),
        }
    }
}
```
