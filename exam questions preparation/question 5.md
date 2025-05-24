## Question 5

Memory Management: Memory management is handled differently in Rust compared to other languages like C++, C# or Java.
Describe how Rust’s memory management principles affected the way you structured your project.
Include examples of how you used Box, Rc, or RefCell in managing heap data.

### Links

## How it's done in Rust

- I Rust, hver value har en ejer, så når den variabel går ud af scope, så bliver hukommelsen automatisk frigivet.
- Smart pointers som `Box`, `Rc`, `Arc` og `Mutex` bliver brugt til heap-allokeret data og shared ownership.
- Heap allocation sker til store og rekursive data strukturer. Stack allocation er hurtigere og bruges til små data strukturer.
- `Box` bruges til heap-allokeret data med single ownership. Det der sker med Box er at den bare laver plads i heapen, og så kan man putte data ind i den, så man er ikke begrænset af plads, fx hvis vi laver som i vores UI, hvor vi har forskellige screens som kan have forskellige størrelser.

### Compared to other languages

Det er ikke nødvendigt at bruge `delete` som i C++ eller `Dispose` i C#, eller Garbage Collector som i Java / C#.

### My view

Jeg synes det er meget smart at Rust har det sådan at en value har en ejer, og at hukommelsen bliver frigivet automatisk når den går ud af scope.

## Code Snippets

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

3. Vi bruger Box til at returnerer Screens fra vores UI lambda function, fordi vi ikke ved om de har samme størrelse i compile-time.

`src/ui/mod.rs` : linje 67 - 103

```rust
pub fn build_ui() -> impl druid::Widget<AppState> {
    ViewSwitcher::new(
        |data: &AppState, _env| (data.current_screen.clone(), data.game_number.clone()),
        |(screen, _game_number), _data, _env| -> Box<dyn druid::Widget<AppState>> {
            ...

            match screen {
                Screen::Loading => Box::new(loading_screen::build_screen()),
                ...
                Screen::RaceScheduleScreen => {
                    Box::new(with_navbar(race_schedule_screen::build_screen()))
                }
            }
        },
    )
}
```

## Other examples

Ift. rekursive data strukturer, så kan Rust ikke have rekursive datatyper uden at bruge `Box`, fordi størrelsen af datatypen skal være kendt ved compile time. Ved at wrappe dem i en `Box`, så får de en kendt størrelse.

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
}
```

- `Rc` bruges til shared ownership af data i en enkelt tråd. Det er en smart pointer der holder styr på hvor mange referencer der er til dataen, og når der ikke er flere referencer, så bliver hukommelsen frigivet.

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

- `RefCell` bruges til interior mutability, som gør det muligt at ændre data selvom det er immutable. Det er en smart pointer der gør det muligt at ændre data uden at flytte ejerskabet.

Nogle steder i koden kan det være at compile-time cheks kan være for restriktive.

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(vec![1, 2, 3]);

    data.borrow_mut().push(4);

    println!("Data: {:?}", data.borrow());
}
```
