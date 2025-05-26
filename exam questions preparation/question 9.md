## Question 9

Concurrency in Rust: Rust provides several tools to handle concurrency, such as threads, Arc, and Mutex.
Discuss how you utilized these tools in your project.
What challenges did you encounter while implementing concurrency, and how did Rust’s safety guarantees affect your solutions?

## Links

## How it's done in Rust

- Rust’s ownership and borrowing rules prevent data races and ensure thread safety at compile time.
- `Mutex` and `Arc` are used to safely share and mutate data across threads; atomic types like `AtomicU16` allow lock-free, thread-safe updates.
- Deadlocks can occur if threads wait on each other’s locks—Rust enforces safety but does not prevent deadlocks, so careful design is needed.
- If a thread panics while holding a `Mutex`, the lock becomes “poisoned”; handling this is essential to avoid crashes.
- `std::thread::scope` ensures all spawned threads complete before exiting scope, preventing certain concurrency bugs.

---

- `Rc<T>`: Single-threaded reference counting for shared ownership.
- `Arc<T>`: Thread-safe reference counting for sharing data between threads.
- `Mutex<T>`: Ensures only one thread accesses data at a time.
- `RefCell<T>`: Allows interior mutability in single-threaded contexts.

### Compared to other languages


- **Python:**
    - Concurrency via `threading`, `multiprocessing`, and `asyncio`.
    - GIL prevents true parallelism for CPU-bound tasks; threads mainly for I/O.
    - Shared data needs manual locking (`threading.Lock`); no compile-time safety.
    - Data races and subtle bugs possible; issues found at runtime.

- **Java:**
    - Native thread support (`Thread`, `Runnable`), synchronization (`synchronized`, `ReentrantLock`, `AtomicInteger`).
    - Memory model and GC help, but thread safety is developer's responsibility.
    - Higher-level tools: `ExecutorService`, `CompletableFuture`.
    - Concurrency bugs (data races, deadlocks) possible; detected at runtime.

- **Rust:**
    - Enforces thread safety at compile time via ownership and borrowing.
    - `Arc<T>`, `Mutex<T>` require `Send`/`Sync` traits for thread sharing.
    - Many concurrency bugs prevented before running code.
    - More effort up front, but safer and more predictable concurrency.

### My view
We dont have a lot of experience with concurrency in Rust, and thus we havent implemented it in our project. However, we have learned about the tools available in Rust for handling concurrency, such as `Mutex`, `Arc`, and atomic types like `AtomicU16`. 

We still implemented Mutex despite not needing it in our project, as we wanted to learn how to use it and how it works. We also learned about the challenges of concurrency, such as deadlocks. Our project already managed most of its connections one at a time, but by implementing Mutex we ensure that only one transaction can access the database connection at a time, which is a good practice for future-proofing our code. It is however not super necessary in our case.

## Code Snippets

1. Vi bruger Mutex til at sikre at kun én tråd kan få adgang til database connectionen ad gangen.

`src/database/connection.rs` : linje 7 og 25-28

```rust
static CONNECTION: Mutex<Option<Connection>> = Mutex::new(None);
```

```rust
pub fn get_connection() -> Result<ConnectionGuard, String> {
    let mut conn_guard = CONNECTION
        .lock()
        .map_err(|_| "Failed to lock connection mutex".to_string())?;

    ...
}
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


## Example of scope-managed mutex

>The inner scope is where all database operations are performed and the mutex connection is freed right after the database portions are finished, the rest is basic error handling and and return.

```rust
pub fn get_top_driver_standings(limit: Option<u8>) -> Vec<Vec<String>> {
    let rows_data = {
        let conn = get_connection().unwrap();
        let final_query = /* ... */;
        let mut stmt = conn.prepare(&final_query).unwrap();
        stmt.query_map([], |row| {
            Ok(vec![
                row.get::<_, String>(0)?,
                row.get::<_, i32>(1)?.to_string(),
            ])
        })
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
    }; // closing the scope here releases the mutex lock from get_connection and immediately frees the connection

    let mut standings = Vec::new();
    for (position, mut row_vec) in rows_data.into_iter().enumerate() {
        row_vec.insert(0, (position + 1).to_string());
        standings.push(row_vec);
    }
    standings
}
```	

## Non-manaaged mutex example

```rust
pub fn get_top_driver_standings(limit: Option<u8>) -> Vec<Vec<String>> {
    let conn = get_connection().unwrap();
    let final_query = /* ... */;
    let mut stmt = conn.prepare(&final_query).unwrap();
    let rows = stmt
        .query_map([], |row| {
            Ok(vec![
                row.get::<_, String>(0)?,
                row.get::<_, i32>(1)?.to_string(),
            ])
        })
        .unwrap(); // There is no scope-ending here which means the get_connection mutex lock stays until this function fully finishes. It could have ended sooner if it was managed as in the previous example.
    let mut standings = Vec::new();
    let mut position = 1;
    for row in rows {
        if let Ok(mut row_vec) = row {
            row_vec.insert(0, position.to_string());
            standings.push(row_vec);
            position += 1;
        }
    }
    standings
}
```