## Question 1

Error Handling and Enums: How have you used enums to manage errors in your project? 
Discuss any patterns you implemented, such as the Result or Option types, and how they contributed to your application’s robustness (Option data type [ Some() and None()] and the Result type [OK() or Err()])

### Links

### Code Snippets

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

### Additional Information

- Pattern matching på Result/Option er bedre end `unwrap()` fordi det ikke vil lave en panic (som gør programmet crasher).
- Man kan bruge en ? operator til at simplificere error propagration - (det er ligesom throws i Java)
- Man bruger None / Err for at indikere at der ikke er nogen værdi tilgængelig.
