## Question 3

Strings and String Handling in Rust: In Rust, the handling of strings is different to languages like Java C# - especially about mutability and ownership. Discuss how you managed strings in your project. What challenges did you face with String and &str types?
Provide examples of where you optimised string usage for performance or memory efficiency (you could discuss the use of `&’static str` or `[A]rc<str>` where lots of cloning is needed)

## Links

## How it's done in Rust

- `String` er heap-allocated og kan ændres, mens `&str` er en reference til en string og er ikke heap-allocated. Det betyder at `String` kan ændres i runtime, mens `&str` ikke kan. `&str` er et borrowed view af en string, og det er derfor hurtigere at bruge i mange tilfælde.
- `&'static str` er en konstant string der lever i hele programmets livscyklus. Det er en reference til en string, der er hardkodet ind i programmet og derfor ikke kan ændres.
- Kloning af strings kan være dyrt i hukommelse og tid, så det er vigtigt at bruge `&str` når det er muligt for at undgå unødvendig kloning.
- Ting som `split()` og `trim()` på en String vil altid returnerer noget samme størrelse eller mindre, og derfor returnerer en `&str` i stedet for en `String`. Noget som `replace()` kan potentielt returnere en større string, og derfor returnerer den en `String` i stedet for en `&str`.

### Compared to other languages

I Java og C# er strings immutable, hvilket betyder at når en string ændres, så bliver der oprettet en ny string i heapen.

Så hvis man skal ændre en string i Java eller C#, så opretter man en ny string i heapen, og det kan være dyrt i hukommelse og tid.

Java har ikke nogen direkte ækvivalent til string slices, hvorimod C# har `ReadOnlySpan<char>` som er en reference til en string, der ikke kan ændres. Det er dog ikke så udbredt som `&str` i Rust.

### My view

Efter at have brugt Rust, så er jeg mere kommet i tanke om det med hvor ineffektivt det egentlig er at opdaterer String i fx Java, det er ikke noget som jeg har tænkt specielt meget over før.

Jeg synes det er smart der er to forskellige typer af strings som har forskellige formål.

## Code Snippets

1. Vi bruger `&'static str` ved at definere en konstant string, der ikke skal ændres i løbet af programmet.

`/src/util/appdata.rs` : linje 10

```rust
const REPO: &str = "https://raw.githubusercontent.com/Peter537/Soft1-Rust-Eksamensopgave/main";
```

\*Det er vigtigt at bemærke, selvom vi kun har sat den til at være en `&str`, så er den stadig en `&'static str`, fordi den er defineret som en konstant og derfor ikke kan ændres i løbet af programmet.

2. Vi bruger String i structs for data som bliver loaded i runtime og kan ændres, og længden af Stringen er ikke pre-defineret. Hvis `&str` så skulle man have defineret en lifetime på dataen Driver henter fra, men det er svært fordi vi henter dataen fra database metoder, så hvis man skulle have en `&str` reference til dataen, så skulle den have en lifetime som er længere end Driver structens livscyklus, hvilket er svært at garantere. Det er derfor `&str` ikke kan bruges her, fordi det ville kræve at vi skulle have en reference til dataen som lever længere end Driver structen, og det er bare

Det er smart at bruge `String` i structs her fordi selv hvis Driver bliver ændret hvem der er owner af data, så vil det ikke påvirke variablerne i structen, fordi de stadig bare er tilknyttet af driverens livscyklus.

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

## Other examples

- Eksempel på Owned / borrowed strings

```rust
fn main() {
    let owned: String = String::from("Hello, Rust!");
    let borrowed: &str = &owned;

    println!("Owned: {}", owned);
    println!("Borrowed: {}", borrowed);
}
```

- Eksempel på Arc til shared ownership

```rust
use std::sync::Arc;

fn main() {
    let shared: Arc<str> = Arc::from("Shared string");
    let clone1 = Arc::clone(&shared);
    let clone2 = Arc::clone(&shared);

    println!("{}", clone1);
    println!("{}", clone2);
}
```
