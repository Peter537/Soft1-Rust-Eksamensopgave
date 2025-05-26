## Question 7

Structs and Data Organization: Structs are fundamental for organizing data in Rust.
How did you decide when to use structs versus tuples or arrays in your project?
Discuss any instances where you used the derive attribute with structs or used impl to add behaviour.

## Links

## How it's done in Rust

- Using the `derive` attribute reduces boilerplate and makes it easier to update structs in the future.
- Structs are more readable and better for organizing complex data, as they allow you to name related fields and add behavior with methods or traits. They help create self-documenting code.
- Tuples group values of different types and are useful for representing a single value with multiple components, or for returning multiple values from a function.
- Arrays store values of the same type and are ideal for fixed-size lists or when you need indexing.
- `impl` is used to add methods and functionality to structs.

### Compared to other languages
You can compare a struct in Rust to a record in Java, as both are primarily used to represent data. However, in Rust, structs are mutable by default, whereas Java records are immutable. When you add methods to a struct in Rust using `impl`, it becomes more like a class in Java or C#, allowing you to encapsulate both data and behavior.

- In languages like C# or Java, tuples are often used for simple data grouping, but they lack named fields, making them less readable compared to Rust's tuples with named fields.

### My view

I find that using structs in Rust allows for better organization and clarity in my code. They provide a way to encapsulate related data and behavior, making it easier to understand the purpose of each component. Tuples are useful for simple groupings, but I prefer structs when I need to represent more complex entities with multiple attributes.

We use tuples mainly for quickly displaying a group of values without needing to define a full struct, such as when calculating total times for drivers
## Code Snippets

1. **Model structs represent complex entities in our application**

`src/model/driver.rs` : lines 1–10

```rust
pub struct Driver {
    pub id: u16,
    ...
    pub image_path: String,
}
```
We use structs like `Driver` to group related fields, making the code more readable and maintainable.

2. **Arrays and tuples for simple data structures**

`src/database/race.rs` : lines 48–56

Here, we use tuples `(u16, f32)` to pair a driver ID with a lap time.

```rust
fn calculate_driver_total_times(driver_lap_times: &[(u16, Vec<f32>)]) -> Vec<(u16, f32)> {
    let mut driver_total_times = Vec::new();
    for (driver_id, laps) in driver_lap_times {
        let total_time: f32 = laps.iter().sum();
        driver_total_times.push((*driver_id, total_time));
    }
    driver_total_times.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    driver_total_times
}
```
Tuples and arrays are ideal for grouping simple values or fixed-size collections.

3. **Using the `derive` attribute for standard trait implementations**

`src/ui/mod.rs` : lines 29–37

```rust
#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub current_screen: Screen,
    pub game_number: String,
    pub selected_team: Option<String>,
    pub current_date: String,
    pub last_race_update_time: String,
    pub show_modal: bool,
}
```
The `derive` attribute automatically implements common traits, reducing boilerplate and making structs easier to use.

4. **Arrays for fixed lists**

`src/util/appdata.rs` : lines 115–126

```rust
const TEAMS: [&str; 10] = [
    "alpine.png",
    ...
    "williams.png",
];
```
Arrays are used for fixed-size lists, such as a set of team image filenames.

5. **Adding behavior to structs with `impl`**

`src/ui/loading_screen.rs` : lines 9–41

```rust
pub fn build_screen() -> impl Widget<AppState> {
    Flex::column()
        ...
        .controller(LoadingController)
}

struct LoadingController;

impl<W: Widget<AppState>> Controller<AppState, W> for LoadingController {
    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &AppState,
        env: &Env,
    ) {
        ...
    }
}
```
We use `impl` to add methods and behavior to structs, such as implementing controller logic for UI components.

