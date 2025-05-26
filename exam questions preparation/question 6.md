## Question 6

Pattern Matching and Control Flow: Pattern matching is a powerful feature in Rust.
How did you use pattern matching to simplify complex control flow in your project?
Provide examples of how this was used to help your code be more efficient and/or readable.
What do you think of the pattern matching instead of using switch type statements?

## Links

## How it's done in Rust

- Pattern matching is more readable than switch statements and makes it easier to handle multiple cases at once.
    - Pattern matching requires you to handle all possible cases, whereas switch statements in other languages may allow you to skip cases.
    - You can destructure data directly within pattern matching.
    - Match guards let you add conditions to patterns, e.g., perform one action if a value is greater than 0 and another if it is less.
- Using `match` allows the Rust compiler to optimize control flow, for example by generating jump tables, making it faster than using if-else chains.

### Compared to other languages

In languages like Java, C#, or JavaScript, switch statements are often used for control flow. However, they have limitations:
- They do not allow destructuring of data.
- They do not support match guards, which can lead to more verbose and less readable code.
- Switch statements can lead to fall-through behavior, where cases are not explicitly handled, potentially causing bugs.

### My view

I find Rust's pattern matching to be a powerful and expressive feature that enhances code readability and maintainability. It allows for clear handling of different cases, especially when dealing with enums or complex data structures. The ability to destructure data directly in the match arms makes it easier to work with nested data without additional boilerplate code.

## Code Snippets

1. We use pattern matching on the `Screen` enum to handle which screen should be displayed in the UI.

Here we also use destructuring of data in pattern matching, because we want to extract, for example, which `team_id` should be shown in the team screen.

File: [`src/ui/mod.rs`](../src/ui/mod.rs)
```rust
match screen {
    Screen::Loading => Box::new(loading_screen::build_screen()),
    Screen::Main => Box::new(main_screen::build_screen()),
    Screen::TeamScreen { team_id } => {
        Box::new(with_navbar(team_screen::build_screen(team_id)))
    }
    ...
    Screen::RaceScheduleScreen => {
        Box::new(with_navbar(race_schedule_screen::build_screen()))
    }
}
```

2. We use `match` on a row in the database methods to handle whether we received an `Ok` or `Err` value.

File: [`src/database/driver.rs`](../src/database/driver.rs)
```rust
match row {
    Ok(driver) => Some(driver),
    Err(_) => None,
}
```
3. We use pattern matching when there may not be data in the database to display in the UI.

File: [`src/ui/main_game_screen.rs`] (lines 37-42)
```rust
let next_race_day: String = match get_next_race() {
    Some(race) => NaiveDate::parse_from_str(&race.date, "%Y-%m-%d")
        .unwrap()
        .to_string(),
    None => no_races_left_string.clone(),
};
```

4. We use `if let` to show a label in the UI if a team is selected.

This is useful when you only care about one case. If you need to handle all cases, `match` is better.

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

- Example of pattern matching for error handling:

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

- Example of destructuring data in pattern matching:

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

- Example of pattern matching with a match guard:

```rust
let number = Some(4);

match number {
    Some(x) if x % 2 == 0 => println!("Even number: {}", x),
    Some(x) => println!("Odd number: {}", x),
    None => println!("No number"),
}
```
