## Question 6

Pattern Matching and Control Flow: Pattern matching is a powerful feature in Rust.
How did you use pattern matching to simplify complex control flow in your project?
Provide examples of how this was used to help your code be more efficient and/or readable.
What do you think of the pattern matching instead of using switch type statements?

### Links

### Code Snippets

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

### Additional Information

- Pattern matching gør så man skal håndtere alle mulige cases, hvor i andre sprog kan man godt undlade at håndtere en case.
- Pattern matching er mere læseligt end switch statements, og det er nemmere at håndtere flere cases på en gang.
