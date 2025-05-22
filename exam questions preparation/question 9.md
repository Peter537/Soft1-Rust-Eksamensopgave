## Question 9

Concurrency in Rust: Rust provides several tools to handle concurrency, such as threads, Arc, and Mutex.
Discuss how you utilized these tools in your project.
What challenges did you encounter while implementing concurrency, and how did Rust’s safety guarantees affect your solutions?

### Links

### Code Snippets

1. Vi bruger Mutex til at sikre at kun én tråd kan få adgang til database connectionen ad gangen.

`src/database/connection.rs` : linje 7

```rust
static CONNECTION: Mutex<Option<Connection>> = Mutex::new(None);
```

2. AtomicU16 for GameNumber til at sikre at kun én tråd kan ændre værdien ad gangen.

`src/database/mod.rs` : linje 12 - 17

```rust
static GAME_NUMBER: AtomicU16 = AtomicU16::new(0);

pub fn set_game_number(number: u16) {
    GAME_NUMBER.store(number, Ordering::SeqCst);
    delete_connection();
}
```

3. Vi bruger Rayon (Rust crate) til at parallelisere downloads af billeder fra GitHub.

`src/util/appdata.rs` : linje 162 - 165

```rust
let results: Vec<Result<(), Box<dyn Error + Send>>> = downloads
    .into_par_iter()
    .map(|(url, dest)| download_file(&url, &dest))
    .collect();
```

4. Vi bruger thread spawning til at køre downloads i baggrunden.

`src/ui/loading_screen.rs` : linje 29 - 37

```rust
let sink = ctx.get_external_handle();
thread::spawn(move || match create_files_if_not_exist() {
    Ok(_) => sink.submit_command(SET_SCREEN, Main, Target::Auto),
    Err(e) => sink.submit_command(
        SHOW_ERROR,
        format!("Failed to create files: {}", e),
        Target::Auto,
    ),
});
```

### Additional Information

- Rust's ownership model og borrowing rules hjælper med at undgå data races og sikre thread safety.
- Mutex og Arc er brugt til at dele data mellem tråde, mens Atomic types som AtomicU16 og AtomicBool bliver brugt til at sikre at data kan ændres sikkert uden at bruge locks.
