## Question 6

Pattern Matching and Control Flow: Pattern matching is a powerful feature in Rust.
How did you use pattern matching to simplify complex control flow in your project?
Provide examples of how this was used to help your code be more efficient and/or readable.
What do you think of the pattern matching instead of using switch type statements?

## Links

## How it's done in Rust

- Pattern matching er mere læseligt end switch statements, og det er nemmere at håndtere flere cases på en gang.
  - Pattern matching gør så man skal håndtere alle mulige cases, hvor i andre sprog med switch-cases kan man godt undlade at håndtere en case.
  - Man kan destructure data inde i pattern matching
  - Man kan match guard i pattern matching, ex noget skal ske hvis en værdi er større end 0, og noget andet skal ske hvis den er mindre end 0.

### Compared to other languages

Andre sprog som Java og C# har switch statements, men de er ikke så fleksible som Rusts pattern matching, ex. de kan ikke destructure data eller have match guards.

### My view

Jeg kan godt lide den måde med at man kan både destructure data og have match guards i pattern matching, det gør det mere læseligt og nemmere at håndtere flere cases på en gang. Jeg synes også det er bedre end switch statements, fordi det er mere fleksibelt og kan håndtere flere forskellige typer data.

## Code Snippets

1. Vi bruger pattern-matching på Screen-enum til at håndtere hvilken skærm skal vises i UI.

`src/ui/mod.rs` : linje 78 - 100

```rust
match screen {
    Screen::Loading => Box::new(loading_screen::build_screen()),
    ...
    Screen::RaceScheduleScreen => {
        Box::new(with_navbar(race_schedule_screen::build_screen()))
    }
}
```

2. Vi bruger match på en row i database metoderne til at håndtere om vi fik en Ok eller Err værdi.

`src/database/driver.rs` : linje 23-26

```rust
match row {
    Ok(driver) => Some(driver),
    Err(_) => None,
}
```

3. Vi bruger pattern-matching hvis der muligvis ikke er data i databasen hvor det skal vises i UI.

`src/ui/main_game_screen.rs` : linje 37 - 42

```rust
let next_race_day: String = match get_next_race() {
    Some(race) => NaiveDate::parse_from_str(&race.date, "%Y-%m-%d")
        .unwrap()
        .to_string(),
    None => no_races_left_string.clone(),
};
```

4. Vi bruger `if-let` til at vise en label i UI hvis der er valgt et team.

Det er noget som er smartere at gøre hvis man kun vil have der skal ske noget hvis én værdi skal gøres noget ved, så her ville det have været smartere at bruge `match` fordi alle cases skal håndteres.

```rust
let selected_team_label = Label::<AppState>::new(|data: &AppState, _env: &Env| {
    if let Some(ref team) = data.selected_team {
        team.to_string()
    } else {
        "No team selected".to_string()
    }
})
```

## Other examples

- Ekstra eksempel på pattern matching ift error handling:

```rust
fn divide(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        Err("Cannot divide by zero")
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
```

- Eksempel på destructuring data i pattern matching:

```rust
enum Message {
    Text(String),
    Image { url: String, caption: String },
    Video(String),
}

fn process_message(msg: Message) {
    match msg {
        Message::Text(content) => println!("Text: {}", content),
        Message::Image { url, caption } => println!("Image: {} with caption: {}", url, caption),
        Message::Video(url) => println!("Video: {}", url),
    }
}
```

- Eksempel på pattern matching med match guard:

```rust
let number = Some(4);

match number {
    Some(x) if x % 2 == 0 => println!("Even number: {}", x),
    Some(x) => println!("Odd number: {}", x),
    None => println!("No number"),
}
```
