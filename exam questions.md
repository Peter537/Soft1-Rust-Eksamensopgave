
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

Vi bruger pattern matching i vores projekt til at forenkle kompleks kontrolflow, især når vi skal vælge hvilken UI der skal vises baseret på applikationens tilstand. Dette gør koden mere læsbar og effektiv sammenlignet med traditionelle `if-else` eller `switch` statements.

#### Eksempel på Pattern Matching i `build_ui`

I `mod.rs` bruger vi pattern matching til at vælge hvilken skærm der skal vises baseret på den aktuelle `Screen` i `AppState`:

```rust
pub fn build_ui() -> impl druid::Widget<AppState> {
    ViewSwitcher::new(
        |data: &AppState, _env| (data.current_screen.clone(), data.game_number.clone()),
        |(screen, _game_number), _data, _env| -> Box<dyn druid::Widget<AppState>> {
            match screen {
                Screen::Main => Box::new(main_screen::build_screen()),
                Screen::TeamScreen { team_id } => Box::new(with_navbar(team_screen::build_screen(team_id))),
                Screen::ChooseTeam => Box::new(choose_team_screen::build_screen()),
                Screen::MainGameScreen => Box::new(with_navbar(main_game_screen::build_screen())),
                Screen::RaceScreen { race_id } => Box::new(with_navbar(race_screen::build_screen(*race_id))),
                Screen::Leaderboard => Box::new(with_navbar(leaderboard_screen::build_screen())),
                Screen::TeamListScreen => Box::new(with_navbar(team_list_screen::build_screen())),
                Screen::DriverScreen { driver_id } => Box::new(with_navbar(driver_screen::build_screen(driver_id))),
                Screen::DriverListScreen => Box::new(with_navbar(driver_list_screen::build_screen())),
                Screen::RaceScheduleScreen => Box::new(with_navbar(race_schedule_screen::build_screen())),
            }
        },
    )
}
```

Her matcher vi på `Screen`-enum'en, som repræsenterer de forskellige skærme i applikationen. Hver variant af `Screen` er forbundet med en specifik funktion, der bygger den tilsvarende UI.

#### Fordele ved Pattern Matching

1. **Læsbarhed**: Koden er nem at læse og forstå, da hver `Screen`-variant er klart defineret med sin tilhørende handling.
2. **Sikkerhed**: Rust sikrer, at alle mulige varianter af `Screen` bliver håndteret. Hvis en ny variant tilføjes, vil kompilatoren give en fejl, hvis den ikke bliver matchet.
3. **Effektivitet**: Pattern matching er mere kompakt og mindre fejlbehæftet end en række `if-else` statements.

#### Sammenligning med Switch Statements

I andre sprog som C++ eller Java kunne vi have brugt en `switch` statement til at opnå lignende funktionalitet. Men Rusts pattern matching er mere kraftfuldt, da det kan håndtere komplekse strukturer og ikke kun simple værdier. For eksempel kan vi matche på strukturer som `Screen::RaceScreen { race_id }` og samtidig udtrække værdien af `race_id`.

#### Opsummering

Pattern matching i Rust gør det muligt at skrive mere kompakt, læsbar og sikker kode. I vores projekt har det været en essentiel del af at håndtere kontrolflowet for UI-rendering, og det har gjort det nemt at tilføje nye skærme uden at introducere fejl.

### 7. Structs and Data Organization

#### 7.1 Structs

Structs bliver i vores projekt brugt til at repræsentere komplekse dataentiteter som `AppState`, `Screen` og alle database-modeller. Eksempelvis bruges følgende struct til at repræsentere en lap i et race:

```rust
pub struct Lap {
    pub id: i32,
    pub race_driver_result_id: i32,
    pub lap_time_ms: i32,
    pub lap_number: i32,
}
```

Derudover bruges structs også til at håndtere applikationens tilstand. For eksempel bruges `AppState` til at holde styr på nuværende skærm, valgte hold og andre vigtige data:

```rust
#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub current_screen: Screen,
    pub game_number: String,
    pub selected_team: Option<String>,
    pub current_date: String,
    pub last_race_update_time: String,
    pub show_modal: bool,
}
```

Her bruger vi `#[derive(Clone, Data, Lens)]` til automatisk at implementere vigtige traits, som er nødvendige for integration med `druid` frameworket. `Clone` gør det muligt at kopiere data, `Data` bruges til at spore ændringer i tilstanden, og `Lens` gør det muligt at binde specifikke felter til UI-komponenter.

Vi bruger også `impl` til at tilføje funktionalitet til structs. For eksempel har vi implementeret en `Default` funktion til `AppState`, som initialiserer standardværdier:

```rust
impl Default for AppState {
    fn default() -> Self {
        AppState {
            current_screen: Screen::Main,
            game_number: String::new(),
            selected_team: None,
            show_modal: false,
            current_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap().to_string(),
            last_race_update_time: Utc::now().to_string(),
        }
    }
}
```

Dette gør det nemt at oprette en ny `AppState` med foruddefinerede værdier.

---

#### 7.2 Tuples/Arrays

Vi bruger tuples og arrays til mere simple funktioner i vores projekt, hvor data kun skal grupperes midlertidigt eller ikke kræver navngivne felter. For eksempel bruges tuples til at returnere flere værdier fra en funktion eller til at repræsentere rækker af data i UI-komponenter:

```rust
let driver_data = vec![
    ("Lewis Hamilton", 44, "Mercedes"),
    ("Max Verstappen", 1, "Red Bull"),
];
```

Arrays bruges, når vi arbejder med en fast størrelse af data, som ikke ændrer sig under runtime. For eksempel kan vi bruge arrays til at definere faste værdier, der bruges i UI eller beregninger.

---

#### Opsummering

Structs bruges til at repræsentere komplekse og navngivne data, især når vi arbejder med databasen eller applikationens tilstand. Derive-attributter som `Clone`, `Data` og `Lens` gør det nemt at integrere structs med `druid` frameworket. Tuples og arrays bruges til enklere og midlertidige grupperinger af data, hvor navngivne felter ikke er nødvendige. Denne tilgang sikrer, at vores data er organiseret og let at arbejde med i hele projektet.


### 8. Module System and Code Organization

Vi har en `mod.rs` fil i hver mappe, som er entry-pointet for den mappe. Inde i den fil vælger vi så hvad som andre mapper skal have adgang til (ved `pub mod`), og hvad skal være privat for mappen (ved `mod`).

--

Vi bruger pub og mod til at gøre metoder, objekter og strukturer 'accessible' fra forskellige filer i vores projekt. Eksempelvis laver vi i hver underfil af vores UI en "build_screen" funktion som offentliggøres gennem "pub fn" og dernæst bruges i AppState til at rendere nuværende skærm "conditionally":

```
pub fn build_screen() -> impl Widget<AppState> {
    Flex::column().with_spacer(20.0).with_child(make_table(
        vec![
            "Name".to_string(),
            "Racing Number".to_string(),
            "Rating".to_string(),
            "Country".to_string(),
            "Team".to_string(),
        ],
        get_driver_data(),
        vec![(0, goto_driver()), (4, goto_team())],
    ))
}
```

Endvidere har hvert folder (modul) hvert sit **mod.rs** hvor vi gør delene til offentlige moduler, ligesom der gøres her med vores goto og table komponenenter:

```
pub mod goto;
pub mod table;
```

"pub mod" bruges til at offtenliggøre filer og funktioner, men når de allerede er offentlige kan man gøre som vi har gjort i mod.rs i "ui" folderen; hvilket er udelukkende at bruge de offentliggjorte entiteter uden at behøve at gøre dem public igen.

```
mod choose_team_screen;
mod main_game_screen;
mod main_screen;
mod race_screen;

mod driver_list_screen;
mod driver_screen;
mod leaderboard_screen;
mod race_schedule_screen;
mod team_list_screen;
mod team_screen;
```




### 9. Concurrency in Rust

Vi bruger Rusts concurrency model til at sikre trådsikker adgang til delte ressourcer og undgå datarace. I vores projekt er concurrency primært implementeret gennem brug af `Mutex` til at håndtere delte ressourcer, især i forbindelse med databaseforbindelser.

#### Shared State med `Mutex`
Vi bruger `Mutex` til at sikre, at kun én tråd ad gangen kan få adgang til den delte databaseforbindelse. Dette forhindrer datarace og sikrer, at vores applikation forbliver stabil. Eksempelvis i `src/database/connection.rs` er vores databaseforbindelse pakket ind i en `Mutex`:

```rust
static CONNECTION: Mutex<Option<Connection>> = Mutex::new(None);
```

For at få adgang til forbindelsen bruger vi `ConnectionGuard`, som sikrer eksklusiv adgang ved hjælp af `MutexGuard`:

```rust
pub struct ConnectionGuard(MutexGuard<'static, Option<Connection>>);

impl Deref for ConnectionGuard {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        (*self.0).as_ref().unwrap()
    }
}

impl DerefMut for ConnectionGuard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        (*self.0).as_mut().unwrap()
    }
}
```

Dette design sikrer, at databaseforbindelsen kan deles sikkert mellem forskellige dele af applikationen uden risiko for samtidige adgangsproblemer.

#### Databaseoperationer
Concurrency håndteres også indirekte gennem transaktioner, som grupperer flere databaseoperationer i en enkelt atomisk enhed. Eksempelvis i `src/database/race.rs`:

```rust
let mut conn = get_connection().unwrap();
let tx = conn.transaction().unwrap();

// Udfør flere operationer inden for transaktionen
let mut stmt_race_driver_results = tx.prepare(
    "INSERT INTO race_driver_results (...) VALUES (...)"
).unwrap();

let mut stmt_laps = tx.prepare(
    "INSERT INTO laps (...) VALUES (...)"
).unwrap();

// Commit transaktionen
tx.commit().unwrap();
```

Dette sikrer, at alle operationer enten udføres fuldt ud eller slet ikke, hvilket forhindrer inkonsistens i databasen.

#### Hvorfor vi ikke bruger Threads, Channels eller Async/Await
I vores projekt bruger vi ikke eksplicitte tråde (`std::thread`), kanaler (`std::sync::mpsc`) eller asynkron programmering (`async/await`), da `druid` frameworket kører på en enkelttrådet event loop. Dette gør det nemmere at håndtere UI-rendering og state management uden behov for avanceret concurrency.

Hvis vi i fremtiden skulle tilføje baggrundsopgaver som datahentning eller tunge beregninger, kunne vi integrere disse værktøjer for at forbedre performance og responsivitet. Eksempelvis:
- **Threads** kunne bruges til at udføre tunge beregninger i baggrunden.
- **Channels** kunne bruges til kommunikation mellem tråde.
- **Async/Await** kunne bruges til ikke-blokerende I/O-operationer.

#### Opsummering
I dette projekt er concurrency primært implementeret gennem brug af `Mutex` til delte ressourcer og transaktioner til databaseoperationer. Selvom vi ikke bruger tråde, kanaler eller async/await, sikrer Rusts sikkerhedsgarantier, at vores nuværende model er robust og fri for datarace. Hvis der opstår behov for yderligere concurrency i fremtiden, kan vi nemt udvide projektet med disse værktøjer.

