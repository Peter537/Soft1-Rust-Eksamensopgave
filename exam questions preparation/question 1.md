# Question 1

Error Handling and Enums: How have you used enums to manage errors in your project?
Discuss any patterns you implemented, such as the Result or Option types, and how they contributed to your application’s robustness (Option data type [ Some() and None()] and the Result type [OK() or Err()])

## Links

## How it's done in Rust

- Pattern matching på Result/Option er bedre end `unwrap()` fordi det ikke vil lave en panic (som gør programmet crasher).
- Man kan bruge en ? operator til at simplificere error propagration
- Man bruger None / Err for at indikere at der ikke er nogen værdi tilgængelig.
- Ved at man har Error handling (Result/Option) så gør det at programmøren bliver tvunget til at tænke over hvad der kan gå galt og hvordan
- Det gør koden lettere at læse og vedligeholde fordi man ved hvordan metoder bruges.

### Compared to other languages

For at lave Error Propagation ligesom vores `download_file` i Java, så ville man i stedet for have metoden være `void` og så have en `throws` i metodesignaturen, som ville gøre brugeren skulle benytte sig af try-catch blokke for at håndtere fejl.

I Java, der er en `Optional` klasse som er meget ligesom Rusts `Option` enum.

Sprog som Python er meget mere lenient omkring error handling, så man skal være meget omhyggelig med at håndtere fejl, ellers kan de ske under runtime og gøre at programmet crasher.

### My view

Til store projekter, så kan jeg godt lide at Rust er så striks omkring error handling, fordi det gør at man sikre sit program mod at crashe.

Men til mindre hyggeprojekter, hvor man bare vil have noget hurtigt op at køre, så kan det godt være lidt irriterende at skulle skrive så meget boilerplate kode for at håndtere fejl, som man godt ved ikke bør ske. Der synes jeg dog det er godt at bruge `unwrap()` i stedet for at skulle skrive en masse kode for at håndtere det.

## Code Snippets

1. Vi bruger Result for Error håndtering i file download

`/src/util/file.rs` : linje 7 - 12

```rust
pub fn download_file(url: &str, dest: &PathBuf) -> Result<(), Box<dyn Error + Send>> {
    let mut response = get(url).map_err(|e| Box::new(e) as Box<dyn Error + Send>)?;
    let mut file = File::create(dest).map_err(|e| Box::new(e) as Box<dyn Error + Send>)?;
    copy(&mut response, &mut file).map_err(|e| Box::new(e) as Box<dyn Error + Send>)?;
    Ok(())
}
```

2. Vi bruger Option for at håndtere null værdier i vores database

`/src/database/driver.rs` : linje 5 - 27

```rust
pub fn get_driver_by_id(id: &u16) -> Option<Driver> {
    ...
    match row {
        Ok(driver) => Some(driver),
        Err(_) => None,
    }
}
```

3. Viser hvordan man konverterer et Result fra en Mutex lock til en custom error

`/src/database/connection.rs` : linje 26 - 28

```rust
let mut conn_guard = CONNECTION
    .lock()
    .map_err(|_| "Failed to lock connection mutex".to_string())?;
```

4. Her hvis en column ikke findes i databasen ift. Ok(Circuit), så bliver der error håndteret med Option, fordi Row er en Result type.

`src/database/circuit.rs` : linje 4 - 23

```rust
pub fn get_circuit_by_id(circuit_id: &u16) -> Option<Circuit> {
    let conn = get_connection().unwrap();
    let mut stmt = conn.prepare(
        "SELECT name, fk_country_id, city, length_km, lap_amount, image_circuit FROM circuits WHERE id = ?"
    ).unwrap();
    let row = stmt.query_row([circuit_id], |row| {
        Ok(Circuit {
            name: row.get(0)?,
            ...
            image_path: row.get(5)?,
        })
    });
    match row {
        Ok(circuit) => Some(circuit),
        Err(_) => None,
    }
}
```

## Other examples

1. `Result` enumen kan enten returnerer Ok(T) eller Err(E). Det er godt til hvis noget kan fejle, ex. en fil download eller database connection.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

2. `Option` er en enum som er optional og kan være enten Some(T) eller None. Det er godt til at håndtere værdier som kan være null.

```rust
enum Option<T> {
    Some(T),
    None,
}
```
