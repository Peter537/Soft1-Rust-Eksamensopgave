## Question 4

Polymorphism: Traits and Enums in Rust allow for polymorphism.
Can you discuss a place in your project where you implemented polymorphic behavior?
How did this design choice benefit your project? How is the problem of inheritance avoided in Rust? (think about composition - use of structs and impl functions)

## Links

- https://medium.com/@ksandeeptech07/diamond-problem-in-oop-explained-672d136912c8

## How it's done in Rust

- Det er ikke muligt at have inheritance i Rust.
- Rust bruger traits og komposition for at undgå diamond problem og inheritance problemer.
- Med komposition gør det når man ændre et sted, så ændre det ikke i de andre steder, så det er godt for maintainability.
- Det er også godt for fleksibilitet fordi man kan mix og matche forskellige komponenter uden at skulle ændre i en base klasse.

### Compared to other languages

Java undgår diamond problemet ved at man kun kan arve fra én klasse

### My view

Jeg kan personligt bedre lide OOP, jeg kan ikke lide det med at man skal tilføje metoder til structs for at få dem til at virke, det er ikke så intuitivt som inheritance

## Code Snippets

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

## Other examples

- Eksempel på Komposition i Rust:

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

- Traits er lidt ligesom interfaces i fx Java, det kan indeholde metoder og kan implementeres af structs.

```rust
trait Drawable {
    fn draw(&self);
}

struct Circle;
struct Square;

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle");
    }
}

impl Drawable for Square {
    fn draw(&self) {
        println!("Drawing a square");
    }
}

... // og så kan man:

fn render(shape: &dyn Drawable) {
    shape.draw();
}
```

- Ligesom med Structs, så kan enums også have metoder og implementere traits.

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

- Applikations eksempel hvor man skal sende forskellige typer notifikationer:

Det her benytter også Open/Closed princip

```rust
trait Notifier {
    fn send(&self, message: &str);
}

struct Email;
struct SMS;
struct Push;

impl Notifier for Email {
    fn send(&self, message: &str) {
        println!("Sending Email: {}", message);
    }
}

impl Notifier for SMS {
    fn send(&self, message: &str) {
        println!("Sending SMS: {}", message);
    }
}

impl Notifier for Push {
    fn send(&self, message: &str) {
        println!("Sending Push Notification: {}", message);
    }
}

... // og så kan man:

fn notify_all(notifiers: Vec<&dyn Notifier>, message: &str) {
    for notifier in notifiers {
        notifier.send(message);
    }
}
```
