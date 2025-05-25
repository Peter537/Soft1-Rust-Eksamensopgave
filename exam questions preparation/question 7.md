## Question 7

Structs and Data Organization: Structs are fundamental for organizing data in Rust.
How did you decide when to use structs versus tuples or arrays in your project?
Discuss any instances where you used the derive attribute with structs or used impl to add behaviour.

## Links

## How it's done in Rust

- Ved at have Derive attributen gør det at vi får mindre boilerplate kode og det er nemmere at ændre structs i fremtiden.
- Structs er mere læselige og nemmere at forstå end tuples og arrays når det skal bruges til komplekse data strukturer, hvor tuples og arrays er bedre til simple data strukturer fordi de er hurtigere at skrive og mere kompakte.
- Structs er custom data typer hvor man kan navngive flere relaterede felter.
  - Det er godt at bruge structs når man vil gruppere relaterede data sammen og give dem meningsfulde navne. Man vil have metoder/traits for at give dem adfærd. Man vil have selvdokumenterende kode.
- Tuples er en samling af værdier af forskellige typer, og de kan bruges til at repræsentere en enkelt værdi med flere komponenter.
  - Tuples er gode til at repræsentere en enkelt værdi med flere komponenter, og de er hurtigere at skrive end structs. De er også gode til at returnere flere værdier fra en funktion.
- Arrays er en samling af værdier af samme type, og de kan bruges til at repræsentere en liste af værdier.
  - Arrays er gode til at repræsentere en liste af værdier af samme type, og de er hurtigere at skrive end structs. De er også gode til at repræsentere en fast størrelse liste af værdier.
  - Det er også godt at bruge hvis man skal bruge indexing.
- Impl er en måde at tilføje metoder og funktioner til structs

### Compared to other languages

Man kan vel lidt sammenligne en struct i Rust med en record i Java ift. det bare er til at præsentere data, og man så gør det til en klasse ligesom i Java ved at tilføje metoder til den. Selvfølgelig i Rust kan man dog godt ændre på dataen i en Struct, hvorimod i Java er det en record, så den er immutable.

I C# kan man navngive felter i tuples, så de er mere læselige end Rusts tuples.

### My view

Jeg kan meget godt lide i C# at man kan navngive felter i tuples, det gør dem mere læselige.

## Code Snippets

1. Model-Structs bruges til at repræsentere komplekse entiteter i vores applikation

`src/model/driver.rs` : linje 1 - 10

```rust
pub struct Driver {
    pub id: u16,
    ...
    pub image_path: String,
}
```

2. Vi brugte Arrays og Tuples til at repræsentere simple data strukturer, som f.eks. en liste af værdier eller en samling af værdier.

`src/database/race.rs` : linje 48 - 56

Her samler vi data `(u16, f32)` med et driver_id og en lap tid

```rust
fn calculate_driver_total_times(driver_lap_times: &[(u16, Vec<f32>)]) -> Vec<(u16, f32)> {
    let mut driver_total_times = Vec::new();
    for (driver_id, laps) in driver_lap_times {
        let total_time: f32 = laps.iter().sum();
        driver_total_times.push((*driver_id, total_time));
    }
    driver_total_times.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    driver_total_times
}
```

3. Vi bruger Derive attribute til at generere standard implementeringer for vores structs

`src/ui/mod.rs` : linje 29 - 37

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

4. Vi bruger array til at præsentere en liste af billeder der skal downloades

`src/util/appdata.rs` : linje 115 - 126

```rust
const TEAMS: [&str; 10] = [
    "alpine.png",
    ...
    "williams.png",
];
```

5. Vi bruger `impl` til at tilføje en `lifecycle` metode til vores Struct, fordi det skal fungere som en controller til vores Druid UI's loading screen

`src/ui/loading_screen.rs` : linje 9 - 41

```rust
pub fn build_screen() -> impl Widget<AppState> {
    Flex::column()
        ...
        .controller(LoadingController)
}

struct LoadingController;

impl<W: Widget<AppState>> Controller<AppState, W> for LoadingController {
    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &AppState,
        env: &Env,
    ) {
        ...
    }
}
```

## Other examples
