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
- Det er smartere at bruge Enums i tilfælde hvor mange kender alle mulige værdier ved compile time
- Det er bedre at bruge Traits i tilfælde hvor nye implementeringer kan tilføjes senere, uden man skal ændre i eksisterende kode. -- De fungerer lidt ligesom interfaces i Java.
- Der bliver brugt enums som Option og Result for at håndtere polymorfisme

### Compared to other languages

Java undgår diamond problemet ved at man kun kan arve fra én klasse

### My view

Jeg kan personligt bedre lide OOP, jeg kan ikke lide det med at man skal tilføje metoder til structs for at få dem til at virke, det er ikke så intuitivt som inheritance

Synes det havde været nemmere at forstå hvis nu man havde haft en Screen klasse i Java som alle screens ville extende fra, og så kunne man nemmere se det var en Screen, i stedet for her er det bare en Enum, og så skal man vide nede i en metode på pattern matching at det er en Screen.

## Code Snippets

1. Vi bruger Enums til at repræsentere forskellige skærme i vores UI, og så bruger vi pattern-matching til at håndtere dem.

Her er det kendt på compile tid hvilke screens der er, så det er bedre at bruge Enums i stedet for Traits.

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

Det med at vi har `'static` her er at vi kræver der ikke er noget fra `inner` må have noget som lever længere end `with_navbar`, altså at det kan gå out-of-scope.

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

4. Eksempel med Option og Result for at håndtere polymorfisme med Connection

Vi starter med at sætte en connection til None hvor vi bruger den som optional, fordi så kan vi så have når get_connection bliver kaldt, så kan vi tjekke om der allerede er en connection, og hvis ikke så opretter vi en ny.

Vi returnerer en Result for at håndtere fejl, hvis der ikke er et spil nummer eller hvis der er en fejl i at åbne databasen.

`src/database/connection.rs` : linje 7 og 25 - 45

```rust
static CONNECTION: Mutex<Option<Connection>> = Mutex::new(None);

...

pub fn get_connection() -> Result<ConnectionGuard, String> {
    let mut conn_guard = CONNECTION
        .lock()
        .map_err(|_| "Failed to lock connection mutex".to_string())?;

    let game_number = super::GAME_NUMBER.load(Ordering::SeqCst);
    if game_number == 0 {
        return Err("Game number is not set".to_string());
    }

    if conn_guard.is_none() {
        let db_file = format!("Career_{}.db", game_number);
        let db_path = get_game_saves_path().join(db_file);
        let conn =
            Connection::open(db_path).map_err(|e| format!("Failed to open database: {}", e))?;

        *conn_guard = Some(conn);
    }

    Ok(ConnectionGuard(conn_guard))
}
```

5. Vi bruger `dyn` i ViewSwitcher fordi vi gerne vil have en trait object, som kan repræsentere forskellige typer af widgets, der implementerer `Widget<AppState>`. Dette gør det muligt at returnere forskellige UI komponenter fra vores `build_ui` funktion.

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
