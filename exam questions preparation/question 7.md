## Question 7

Structs and Data Organization: Structs are fundamental for organizing data in Rust.
How did you decide when to use structs versus tuples or arrays in your project?
Discuss any instances where you used the derive attribute with structs or used impl to add behaviour.

### Links

### Code Snippets

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

### Additional Information

- Ved at have Derive attributen gør det at vi får mindre boilerplate kode og det er nemmere at ændre structs i fremtiden.
- Structs er mere læselige og nemmere at forstå end tuples og arrays når det skal bruges til komplekse data strukturer, hvor tuples og arrays er bedre til simple data strukturer fordi de er hurtigere at skrive og mere kompakte.
