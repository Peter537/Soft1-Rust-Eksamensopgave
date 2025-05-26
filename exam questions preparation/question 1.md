# Question 1

Error Handling and Enums: How have you used enums to manage errors in your project?
Discuss any patterns you implemented, such as the Result or Option types, and how they contributed to your application’s robustness (Option data type [ Some() and None()] and the Result type [OK() or Err()])

## How it's done in Rust

- Pattern matching:
    - `Result<T, E>:` Used for operations that can fail
    - `Option<T>:` Used for values that might be absent
- The ? operator can be used similar to Java (Throws) to propagate en error upwards

1. The `Result` enum can either return `Ok(T)` or `Err(E)`. It's useful for operations that can fail, such as file downloads or database connections.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

2. `Option` is an enum that represents an optional value and can be either `Some(T)` or `None`. It's useful for handling values that might be null.

```rust
enum Option<T> {
    Some(T),
    None,
}
```

### Compared to other languages

In **Rust**, error handling is explicit and enforced by the type system using `Result` and `Option` enums. This encourages developers to handle errors at compile time, reducing the risk of unhandled exceptions at runtime.

- **Java** uses exceptions for error handling. Methods can throw checked or unchecked exceptions, and callers can catch or propagate them. This can sometimes lead to missed error handling if exceptions are not properly caught.
- **Python** also uses exceptions, but all exceptions are unchecked. Errors are handled using `try`/`except` blocks, and if not caught, they propagate up and can crash the program.

**Key differences:**
- Rust’s approach makes error handling explicit and part of the function signature, while Java and Python rely on runtime exceptions.
- Rust’s `?` operator is similar to Java’s `throws` or Python’s `raise`, but it is checked at compile time.
- In Rust, the absence of a value is handled with `Option`, avoiding null pointer exceptions common in Java and Python.

**Example:**

| Language | Error Handling Example |
|----------|-----------------------|
| Rust     | `let file = File::open("foo.txt")?;` |
| Java     | `try { FileInputStream file = new FileInputStream("foo.txt"); } catch (IOException e) { ... }` |
| Python   | `try: file = open("foo.txt") except IOError as e: ...` |

Rust’s model leads to safer and more predictable code, as all possible errors must be considered by the developer.

## Code Snippets

1. Result is used to error handle the connection establishment

File: [`src\database\connection.rs`](../src/database/connection.rs)
```rust
pub fn get_connection() -> Result<ConnectionGuard, String> {
    let mut conn_guard = CONNECTION
        .lock()
        .map_err(|_| "Failed to lock connection mutex".to_string())?; // Propagates error if lock fails

    let game_number = super::GAME_NUMBER.load(Ordering::SeqCst);
    if game_number == 0 {
        return Err("Game number is not set".to_string()); // Explicitly returns an Err
    }

    if conn_guard.is_none() {
        let db_file = format!("Career_{}.db", game_number);
        let db_path = get_game_saves_path().join(db_file);
        let conn = Connection::open(db_path)
            .map_err(|e| format!("Failed to open database: {}", e))?; // Propagates error if open fails

        *conn_guard = Some(conn);
    }

    Ok(ConnectionGuard(conn_guard)) // Returns Ok on success [Implicit return at the end of a function]
}
```

2. Option is here used to handle the case where a driver may not be found.

File: [`src\database\driver.rs`](../src/database/driver.rs)
```rust
pub fn get_driver_by_id(id: &u16) -> Option<Driver> {
    ...
    match row {
        Ok(driver) => Some(driver),
        Err(_) => None, // Notice that this pattern matches that of an Option {Some(T) or None}
    }
}
```

3. Here we propegate the error from the database query using `?`, which is a shorthand for propagating errors in Rust. If the query fails, it will return an `Err` variant of the `Result` type.

File: [`src\database\circuit.rs`](../src/database/circuit.rs)
```rust
pub fn get_circuit_by_id(circuit_id: &u16) -> Option<Circuit> {
    let conn = get_connection().unwrap();
    let mut stmt = conn.prepare(
        "SELECT name, fk_country_id, city, length_km, lap_amount, image_circuit FROM circuits WHERE id = ?"
    ).unwrap();
    let row = stmt.query_row([circuit_id], |row| {
        Ok(Circuit {
            name: row.get(0)?,
            country_id: row.get(1)?,
            city: row.get(2)?,
            length_km: row.get(3)?, // '?' propegation
            lap_amount: row.get(4)?,
            image_path: row.get(5)?,
        }) // Any of these `row.get()` calls can fail, so we use `?` to propagate the error and return that to row.
    });
    match row {
        Ok(circuit) => Some(circuit),
        Err(_) => None,
    }
}
```

4. We use `.unwrap()` quite a lot in our code, and we are aware that this is a trade-off in terms of error handling because the program would crash if an error occurs. However, in principle, these errors should not happen, as they would typically be caused by user mistakes, such as if the database was deleted or similar situations.

* In some cases where our critical data is not present, the app cannot fully function and thus should combust.

File : [`src\ui\driver_screen.rs`](../src/ui/driver_screen.rs)
```rust
pub fn build_screen(driver_id: &u16) -> impl Widget<AppState> {
    let driver = get_driver_by_id(driver_id).unwrap(); // This returns an Option<Driver> and we assume it will work. 
    let driver_contract = get_driver_contract(driver_id).unwrap(); // if it doesnt, the app should stop.
    ...
}
```