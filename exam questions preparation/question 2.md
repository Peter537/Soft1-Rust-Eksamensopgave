# Question 2

Ownership and Borrowing Concepts: Explain the concept of ownership and borrowing in Rust.
How did these features influence the design and functionality of your project?
Provide examples of how managing ownership and borrowing helped improve your code’s safety or performance. What are some of the common problems you encountered using the Rust Ownership and Borrowing model?

## Links

## How it's done in Rust

- Primitive typer, der implementerer `Copy`-traiten (f.eks. `u16`, `f32`, `bool`, `char`), bliver ikke flyttet, når de bruges i en funktion; i stedet bliver deres værdi kopieret. Derfor kan variablen stadig anvendes senere i den oprindelige funktion, selv efter at være blevet brugt som argument i et funktionskald.
- Borrow checker gør så data races og dangling references ikke kan ske.
  - Ift data races (hvis to tråde prøver at ændre data på samme tid) så er det kun muligt at have en mutable reference til data ad gangen. Man kan heller ikke have en mutable og en immutable reference til data på samme tid.
- Nogle gange bliver man nødt til at clone data for at gøre borrow checker glad (?)
- Borrowing og ownership gør det klart hvem der ejer data og hvem der låner det. Det gør det lettere at forstå livscyklussen for data i programmet.
- Man bruger kun `move` hvis man skal have inde i en closure/lambda hvis der er ting inden i som skal have ownership af data ude fra
- Rust lifetimes hjælper med at sikre at referencer ikke lever længere end de data de refererer til, hvilket forhindrer dangling references.
  - Man kan bruge `<'a>` på en funktion/struct for at angive at den har en lifetime parameter, som kan bruges til at specificere hvor længe referencerne lever. Så bliver de brugt ved at sige fx `x: &'a str` for at sige at `x` er en reference til en string som lever i samme lifetime som `'a`.
- Rust tillader ikke Null referencer, hvilket forhindrer Null pointer exceptions. I stedet bruger Rust `Option`-typen til at repræsentere værdier, der kan være til stede eller fraværende.

### Compared to other languages

Det betyder man ikke behøver have en Garbage Collector, som i Java og C#, så der er ikke noget Runtime overhead. Man skal heller ikke sørge for selv at frigive hukommelse, som i C/C++.

Hvis man kører multi-threaded kode, så sikrer man sig stadig at ens variabler har de værdier som de skal have, hvor sprog som JavaScript og Python kan have problemer med det, fordi de ikke har et striks ownership system.

### My view

Jeg kan godt lide at det er klart hvem det er som ejer data, men dog skal man lige sætte sig lidt mere ind i det, fordi et eksempel fra vores race.rs med `driver_lap_times`, der har vi bare været vant til fra Java at man parser data ind (hvor det så er en pointer til data), men her er det meget mere eksplicit at man skal bruge `&` for at låne data, så skal man lige huske.

Det har gjort mig mere opmærksom på hvordan det er data bevæger sig rundt i programmet, og det er helt klart noget som jeg kommer til at tænke mere på i fremtiden, også når jeg skriver i andre sprog.

## Code Snippets

1. Vi bruger `move` for at flytte ejerskab ind i en closure for senere brug. Den her bruger også Mutable borrow ved at data skal opdateres.

\*Vi flytter ejerskab af `career_id` ind i closuren.

`/src/ui/main_screen.rs` : linje 53 - 70

```rust
let career_id = career_number;

column = column.with_child(Button::new(label.clone()).on_click(
    move |ctx, data: &mut AppState, _env| {
        data.game_number = career_id.to_string();
        set_game_number(career_id);
        if has_selected_team() == false {
            data.current_screen = ChooseTeam;
        } else {
            let selected_team = get_selected_team(&career_id.to_string());
            data.selected_team = selected_team;
            data.current_screen = MainGameScreen;
        }

        data.show_modal = false;
        ctx.request_update();
    },
));
```

2. Vi bruger `&` for at en funktion kan borrow en variabel uden at flytte ejerskabet

Hvis vi ikke havde brugt `&driver_lap_times` (men bare `driver_lap_times`), så ville ejerskabet være flyttet ind i funktionen `calculate_driver_total_times`, og så ville den ikke kunne bruges i `create_driver_results` funktionen, fordi ejerskabet nu er flyttet til `calculate_driver_total_times`.

`/src/backend/race.rs` : linje 20 - 25

```rust
let driver_lap_times =
    generate_driver_lap_times(&drivers, circuit.lap_amount, circuit.length_km);

let driver_total_times = calculate_driver_total_times(&driver_lap_times);

let driver_results = create_driver_results(&driver_total_times, &driver_lap_times);
```

3. Vi bruger `String` i structs loaded fra databasen, fordi det vil være for komplekst at holde styr på lifetimes strings i structs.

`src/model/driver.rs` : linje 1 - 10

```rust
pub struct Driver {
    pub id: u16,
    pub first_name: String,
    pub last_name: String,
    pub rating: u8,
    pub country_id: u8,
    pub date_of_birth: String,
    pub racing_number: u8,
    pub image_path: String,
}
```

4. Vi bruger `{}`-scope lige efter `let tx = conn.transaction().unwrap();` fordi vi skal droppe vores prepared statements inden vi kan commit vores transaction. Det er fordi prepared statements borrower `tx`, så vi kan ikke commit før de er droppet

`src/database/races.rs` : linje 45 - 90

```rust
pub fn save_driver_results(
    ...
) {
    let mut conn = get_connection().unwrap();
    let tx = conn.transaction().unwrap();
    {
        let mut stmt_race_driver_results = tx
            .prepare(
                r#"INSERT INTO race_driver_results (
                fk_season_schedule_id, fk_driver_id, fk_team_id, placement, points, status
            ) VALUES (?, ?, ?, ?, ?, ?)"#,
            )
            .unwrap();
        let mut stmt_laps = tx
            .prepare(
                r#"INSERT INTO laps (
                fk_race_driver_result_id, lap_time_ms, lap_number
            ) VALUES (?, ?, ?)"#,
            )
            .unwrap();
        ...
    }
    tx.commit().unwrap();
}
```

## Other examples

- Eksempel på hvordan ejerskab og flytning fungerer i Rust:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // Ownership of the string is moved to s2

    // println!("{}", s1); // Error: s1 is no longer valid
    println!("{}", s2);
}
```

- Eksempel på hvordan mutable borrow fungerer i Rust:

```rust
fn main() {
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2); // Allowed: multiple immutable references
}
```

```rust
fn main() {
    let mut s = String::from("hello");

    let r = &mut s;
    r.push_str(", world!");
    println!("{}", r); // Allowed: single mutable reference
}
```
