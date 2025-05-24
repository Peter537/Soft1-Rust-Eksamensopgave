## Question 9

Concurrency in Rust: Rust provides several tools to handle concurrency, such as threads, Arc, and Mutex.
Discuss how you utilized these tools in your project.
What challenges did you encounter while implementing concurrency, and how did Rust’s safety guarantees affect your solutions?

## Links

## How it's done in Rust

- Rust's ownership model og borrowing rules hjælper med at undgå data races og sikre thread safety.
- Mutex og Arc er brugt til at dele data mellem tråde, mens Atomic types som AtomicU16 og AtomicBool bliver brugt til at sikre at data kan ændres sikkert uden at bruge locks.
- Hvis flere tråder venter på hinanden for at release en lock, kan det føre til deadlocks. Det er vigtigt at designe tråd-sikker kode for at undgå dette, fordi Rust ikke undgår deadlocks automatisk.
- Hvis en tråd bliver paniced mens den holder på en lock, så vil det sige at Mutex bliver "poisoned", og det vil sige at andre tråde ikke kan få adgang til den lock før den bliver "unpoisoned". Det er vigtigt at håndtere dette i din kode for at undgå at programmet crasher.

### Compared to other languages

Sprog som Java og C# er lidt ligesom Rust med deres udfordringer i deadlocks, men hvis en tråd har en unormal trådafslutning, så vil Rust have man selv laver explicit fejlhåndtering, hvorimod Java og C# automatisk frigiver låsen og overlader datakonsistent til programmøren.

Det vil sige at Rust's tilgang er mere sikker, men kompleks, mens Java og C# er simplere, men kræver ekstra opmærksomhed på data.

### My view

## Code Snippets

1. Vi bruger Mutex til at sikre at kun én tråd kan få adgang til database connectionen ad gangen.

`src/database/connection.rs` : linje 7

```rust
static CONNECTION: Mutex<Option<Connection>> = Mutex::new(None);
```

.. TILFØJ DET MED LOCKED CONNECTION i get_connection

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

4. Vi bruger thread spawning til at køre downloads i baggrunden, mens vi viser en loading screen.

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

## Other examples

- Simpelt eksempel på Rust standard library `std::thread` til at oprette tråde:

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from a new thread!");
    });

    // Wait for the thread to finish
    handle.join().unwrap();
    println!("Main thread finished.");
}
```

- Eksempel på multi-threaded counter:

Hver tråd vil inkrementere tælleren 10 gange, og vi bruger Arc og Mutex til at dele tælleren mellem trådene.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", *counter.lock().unwrap());
}
```
