## Game Logic

1. Kalender / Next Action
2. Race logic
3. Screens
4. DB Logic
5. Load/Create Game

## Screens

### Home Screen

1. Create new Save Game => Choose Team Screen
2. Load existing Save Game => Main Game Screen
3. Delete Save Game

### Choose Team Screen

1. Select a Team from a list of available teams
2. Create Game Save => Main Game Screen

### Main Game Screen

1. 'Next Turn' button to advance the game

### Race Screen

1. Display result (if race is done)

### Driver Screen

1. Display driver ratings
2. Display season stats (team, position, points, races, etc.)
3. Display career stats (team, position, points, etc.)

### Team Screen

1. Display season stats (drivers, position, points, races, etc.)
2. Display career stats (drivers, position, points, etc.)

### Leaderboard Screen

1. Display the current leaderboards (individual & team rankings)

### Season Overview Screen

1. Display the Races and Winners
2. Display top 3(?) of leaderboard (individual & team rankings)

---

## Exam Questions

### 1 Error Handling and Enums

Ift. Result<>, så kan vi bruge det til når der kan ske fejl, ex. ved `/util/image_loader.rs` hvis billede-lokationen ikke eksisterer.

Ift. Option<>, så kan vi bruge det i database-metoderne hvis nu der ikke er nogen data i databasen.

Alle steder med `.expect()` skal vi fjerne og fejlhåndtere ordentligt.

---

Der er 5 (ish) forskellige muligheder til at vi kan håndtere errors:

1. '.unwrap()'
2. '.expect()'
3. 'match / if let'
4. '.unwrap_or() / .unwrap_or_else()'
5. '?'

Både `unwrap()` og `expect()` er ikke gode til at håndtere errors i vores tilfælde, da de begge vil kalde `panic!()` hvis der sker en fejl, hvilket får programmet til at crashe. Det ville have givet mening, at brug hvis man havde lavet et cli program, såsom git hvor alt bare skal stoppe, hvis der er fejl, men ikke i vores tilfælde, da vi har UI og skal håndtere det, på en måde der kan informere brugeren om hvad der er sket af fejl.

Ved at bruge `match` eller `if let`, kan vi have fuld kontrol over hvad der skal gøres i tilfælde af fejl og success (for bøde Result og Option).

Der er 2 (muligvis flere) variationer af `unwrap`: `.unwrap_or()` / `.unwrap_or_else()`, disse giver os muligheden for at have en default værdi (fallback safety), hvis der sker en fejl. Det her er en mindre udgave af `match` og `if let`.

`?` er en shorthand for `match`, som gør at vi kan skrive det på en mere kompakt måde. Det er dog kun muligt at bruge `?` hvis metoden returnerer en `Result` eller `Option`. Det er derfor ikke muligt at bruge det i alle situationer.


Alle steder med `.expect()` skal vi fjerne og fejlhåndtere ordentligt.

### 2. Ownership and Borrowing Concepts

Anvend `&` og `&mut` til at låne variabler i stedet for at tage ejerskab af dem.

`.clone()` kan være være dårligt for performance, hvis der er store datamængder, da det vil lave en kopi af dataene og ikke bare låne dem.

I rust kan man borrowe det samme flere gang: &T, &T, &T. Eller bare kun én gang: &mut T. Dog ikke sammen, da det ville være at man prøver at læse og skrive til noget på samme tid (ish). Dette kaldes for `Data Race`, her skal man passe på at man ikke kalder det `Race condition`, selvom det lidt er det sammen, men ikke helt.
- Race Condition: "Logisk fejl pga. timing, fx to tråde konkurrerer om rækkefølge. Kan ske uden memory-fejl."
- Data Race: "En type race condition, hvor simultan adgang til data kan give memory corruption."

`*`: bruges til at dereferencere en reference eller pointer, altså til at få adgang til den værdi, som referencen peger på, ikke til at tage ejerskab af den.

`move` keyword: Tag ejerskab af alle de variabler udenfor clousuren (lamda), som du bruger inden i den.

Ex. i `choose_team_screen.rs`:
```rust
[...]
// Balance columns manually
if left_count <= right_count {
    left_column.add_child(driver_column);
    left_column.add_spacer(10.0);

    left_column.add_child(Button::new("Select").on_click({
        let short_name = short_name.clone(); // clone needed because short_name is &String
        move |ctx, data: &mut AppState, _env| {
            println!("Selected team: {}", short_name);
            data.selected_team = Some(short_name.clone());
            ctx.request_update();
        }
    }));

    left_column.add_spacer(15.0);
    left_count += 1;
} else {
    right_column.add_child(driver_column);
    right_column.add_spacer(10.0);

    right_column.add_child(Button::new("Select").on_click({
        let short_name = short_name.clone();
        move |ctx, data: &mut AppState, _env| {
            println!("Selected team: {}", short_name);
            data.selected_team = Some(short_name.clone());
            ctx.request_update();
        }
    }));
    right_column.add_spacer(15.0);
    right_count += 1;
}
[...]
```

Her bruger vi `move` keywordet til at tage ejerskab af `short_name` variablen, så vi kan bruge den inde i clousuren. Det er nødvendigt at da vi skal kunne gemme bruge valgt senere, hvis ikke der anvendes `move` keywordet, da det oprindelige scope hvor short_name blev defineret ville være ud af scope.


### 3. Strings and String Handling in Rust

`String`: En growable, heap-allocated string type. Det er en dynamisk størrelse og kan ændres i runtime.
`&Str`: En reference til en string, som ikke ejer den. Det er en immutable reference og kan ikke ændres.

Vi skal anvende `String` når vi skal ændre på en string, og `&str` når vi kun skal læse den. Samt tænke over, at vi undgår at clone hvis en funktion returnerer &str.

`&'static str`: er en reference til en string, der lever i hele programmets lifetime (som er 'static). Det betyder, at stringen er kendt på compile time og kan ikke ændres, da den er immutabel.

`Arc<str>`: står for "Atomic Reference Counting", og det bruges til at dele ejerskab af data, som er immutable (ikke kan ændres), mellem flere tråde i et thread safe environment.


### 4. Polymorphism
1. Polymorphism med Traits:
En trait i Rust definerer et fælles interface, som forskellige typer kan implementere. Det er lidt ligesom et interface i andre sprog som Java eller C#. Når en type implementerer et trait, lover den at tilbyde de funktioner, der er defineret af det trait.

```rust
trait Animal {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

fn make_speak(animal: &dyn Animal) {
    animal.speak();
}

fn main() {
    let dog = Dog;
    let cat = Cat;
    
    make_speak(&dog); // Woof!
    make_speak(&cat); // Meow!
}
```

2. Polymorphism med Enums:
En enum i Rust er en type, der kan være en af flere mulige varianter. Hver variant kan have forskellige datatyper. Dette kan bruges til at modellere polymorfisme, hvor du kan have forskellige typer, men stadig kunne arbejde med dem på samme måde.

```rust
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle(width, height) => width * height,
        }
    }
}

fn main() {
    let circle = Shape::Circle(2.0);
    let rectangle = Shape::Rectangle(2.0, 3.0);
    
    println!("Circle area: {}", circle.area());
    println!("Rectangle area: {}", rectangle.area());
}
```


Vi kunne have brugt polymorphism ift. til vores forskellige screens, det var noget vi overvejede i starten, da hver side skulle have en `build_screen` metode, dog indså vi at nogle sider skulle have parametre og andre ikke, så vi endte med at have gentagende kode i stedet.

### 5. Memory Management

`Rust`: Hukommelsen styres gennem ownership og borrowing, hvilket forhindrer hukommelseslækager og datarace uden garbage collection.

`C++`: Hukommelsen håndteres manuelt, og programmøren er ansvarlig for at allokere og frigive hukommelsen.

`C#` og `Java`: Brug af garbage collectors til automatisk hukommelsesstyring, hvilket gør det nemmere at undgå hukommelseslækager, men kan medføre performance costs.

Dog har Rust og C# adgang til unsafe code, via `unsafe` keyword, hvilket gør det muligt at omgå safety mechanisms og direkte manipulere hukommelsen.


Smart pointers: `Box`, `Rc`, `RefCell`: https://chatgpt.com/c/68074d44-5b34-8012-92a9-51126a405169

`Box`:
Box er en smart pointer, der tager ansvar for at allokere hukommelse på heap'en. Normalt i Rust er data på stacken, men med en Box kan du allokere data på heap'en og stadig beholde Rusts ejendom og livstidssikring.
- Hvad gør den?

    Box flytter ejerskabet af data til heapen, hvilket betyder, at du kan opbevare store mængder data uden at bruge for meget stack-plads. Når du bruger en Box, er det kun muligt at have én ejer af de data, der er allokeret i heapen.

`Rc`:
Rc står for Reference Counted og er en smart pointer, der gør det muligt at dele ejerskabet af data mellem flere forskellige steder i koden. Dette sker ved hjælp af reference-tælling: Rc holder styr på, hvor mange referencer der er til den data, den peger på. Når ingen referencer er tilbage, bliver hukommelsen frigivet.

- Hvad gør den?

    Rc tillader, at flere variable kan eje data samtidig. Dette betyder, at du kan dele ejerskab af data mellem flere steder i koden, men det kræver, at dataene er uforanderlige (immutable). Hvis du har behov for at dele data på tværs af funktioner og har flere ejere, kan du bruge Rc.

`RefCell`:
RefCell giver en form for indirekte mutabilitet. Normalt kræver Rust, at data kun kan ændres, hvis de er mutable og har en entydig ejer. RefCell giver dig mulighed for at ændre data, selvom de er immutabelt lånt, ved at kontrollere adgang på runtime i stedet for compile time. Det gør brug af en køretidstidens tjek af låse og referencer.

- Hvad gør den?
    
    RefCell tillader, at data ændres, selvom de ikke er mutable direkte, ved at bruge intern reference-tælling og låsning. Det betyder, at du kan ændre data gennem en immutable reference til RefCell, men kun én aktiv lås vil være tilladt ad gangen, og forsøg på at låse data flere gange samtidig vil føre til en runtime-fejl.


### 6. Pattern Matching and Control Flow

Vi bruger f.eks. pattern-matching i mod.rs' `build_ui` for at vælge hvilken UI der skal vises.

### 7. Structs and Data Organization

Vi bruger Structs når vi henter noget ud fra databasen.

Vi bruger Tuples/Arrays når det er små-ting inde i koden som skal flyttes rundt på.

### 8. Module System and Code Organization

In each directory, we have a file `mod.rs` which is the entry point for that directory.

Vi har en `mod.rs` fil i hver mappe, som er entry-pointet for den mappe. Inde i den fil vælger vi så hvad som andre mapper skal have adgang til (ved `pub mod`), og hvad skal være privat for mappen (ved `mod`).

### 9. Concurrency in Rust

Vi bruger Mutex i database-connection håndteringen.
