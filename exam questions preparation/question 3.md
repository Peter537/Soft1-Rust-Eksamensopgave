## Question 3

Strings and String Handling in Rust: In Rust, the handling of strings is different to languages like Java C# - especially about mutability and ownership. Discuss how you managed strings in your project. What challenges did you face with String and &str types?
Provide examples of where you optimised string usage for performance or memory efficiency (you could discuss the use of `&’static str` or `[A]rc<str>` where lots of cloning is needed)

### Links

### Code Snippets

1. Vi bruger `&'static str` ved at definere en konstant string, der ikke skal ændres i løbet af programmet.

`/src/util/appdata.rs` : linje 10

```rust
const REPO: &str = "https://raw.githubusercontent.com/Peter537/Soft1-Rust-Eksamensopgave/main";
```

*Det er vigtigt at bemærke, selvom vi kun har sat den til at være en `&str`, så er den stadig en `&'static str`, fordi den er defineret som en konstant og derfor ikke kan ændres i løbet af programmet.

2. Vi bruger String i structs for data som bliver loaded i runtime og kan ændres

`/src/model/driver.rs` : linje 1 - 10

```rust
pub struct Driver {
    pub first_name: String,
    ...
    pub image_path: String,
}
```

3. Vores database metoder bruger `&str` (string slice) for effektivitet og for at undgå unødvendig kloning af data.

`src/database/driver.rs` : linje 100 - 110

```rust
pub fn get_driver_id_by_fullname(full_name: &str) -> Option<u16> {
    ...
}
```

### Additional Information

- `String` er heap-allocated og kan ændres, mens `&str` er en reference til en string og er ikke heap-allocated. Det betyder at `String` kan ændres i runtime, mens `&str` ikke kan. `&str` er et borrowed view af en string, og det er derfor hurtigere at bruge i mange tilfælde.
- `&'static str` er en konstant string der lever i hele programmets livscyklus. Det er en reference til en string, der er hardkodet ind i programmet og derfor ikke kan ændres.
- Kloning af strings kan være dyrt i hukommelse og tid, så det er vigtigt at bruge `&str` når det er muligt for at undgå unødvendig kloning.
