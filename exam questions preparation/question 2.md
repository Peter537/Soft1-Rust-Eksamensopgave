## Question 2

Ownership and Borrowing Concepts: Explain the concept of ownership and borrowing in Rust.
How did these features influence the design and functionality of your project?
Provide examples of how managing ownership and borrowing helped improve your code’s safety or performance. What are some of the common problems you encountered using the Rust Ownership and Borrowing model?

### Links

### Code Snippets

1. Vi bruger `move` for at flytte ejerskab ind i en closure for senere brug. Den her bruger også Mutable borrow ved at _data skal opdateres

`/src/ui/component/goto.rs` : linje 15 - 19

```rust
Box::new(move |_ctx: &mut EventCtx, _data: &mut AppState| {
    let driver_id = get_driver_id_by_fullname(&driver).unwrap_or(0);
    _data.current_screen = DriverScreen { driver_id };
    _ctx.request_update();
})
```

2. Vi bruger `&` for at en funktion kan borrow en variabel uden at flytte ejerskabet

`/src/backend/race.rs` : linje 20 - 25

```rust
let driver_lap_times =
    generate_driver_lap_times(&drivers, circuit.lap_amount, circuit.length_km);

let driver_total_times = calculate_driver_total_times(&driver_lap_times);

let driver_results = create_driver_results(&driver_total_times, &driver_lap_times);
```

Hvis vi ikke havde brugt `&driver_lap_times` (men bare `driver_lap_times`), så ville ejerskabet være flyttet ind i funktionen `calculate_driver_total_times`, og så ville den ikke kunne bruges i `create_driver_results` funktionen.

### Additional Information

- Primitive typer, der implementerer `Copy`-traiten (f.eks. `u16`, `f32`, `bool`, `char`), bliver ikke flyttet, når de bruges i en funktion; i stedet bliver deres værdi kopieret. Derfor kan variablen stadig anvendes senere i den oprindelige funktion, selv efter at være blevet brugt som argument i et funktionskald.
- Borrow checker gør så data races og dangling references ikke kan ske.
- Nogle gange bliver man nødt til at clone data for at gøre borrow checker glad (?)
