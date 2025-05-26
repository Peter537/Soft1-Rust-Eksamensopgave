# Question 2

Ownership and Borrowing Concepts: Explain the concept of ownership and borrowing in Rust.
How did these features influence the design and functionality of your project?
Provide examples of how managing ownership and borrowing helped improve your code’s safety or performance. What are some of the common problems you encountered using the Rust Ownership and Borrowing model?

## How it's done in Rust

**Ownership**

Every value has a variable that’s its owner. There's only one owner at a time. When the owner goes out of scope, the 
value is dropped.

**Borrowing**

* Immutable Borrowing: You can have multiple immutable references to a value, but you cannot modify it.
* Mutable Borrowing: You can have only one mutable reference to a value at a time, and no immutable references can coexist with it.
- Primitive types that implement the `Copy` trait (e.g., `u16`, `f32`, `bool`, `char`) are copied, not moved, when 
passed to functions. This means the original variable remains usable after being used as an argument.
---
- The borrow checker prevents data races and dangling references:
    - Only one mutable reference to data is allowed at a time, and you cannot have mutable and immutable references simultaneously.
- Sometimes cloning data is necessary to satisfy the borrow checker.
- Ownership and borrowing clarify who owns and who borrows data, making data lifecycles easier to understand.
- The `move` keyword is used in closures when ownership of external data is required inside the closure.
- Lifetimes (`<'a>`) ensure references do not outlive the data they point to, preventing dangling references.
- Rust does not allow null references, avoiding null pointer exceptions. Instead, it uses the `Option` type to represent optional values.

### Compared to other languages

Rust’s ownership and borrowing system is unique compared to most mainstream languages:

- **C/C++:** Memory management is manual, and there is no built-in ownership or borrowing system. This can lead to issues like dangling pointers, double frees, and memory leaks. Rust’s borrow checker enforces safety at compile time, preventing these problems.
- **Java/C#:** These languages use garbage collection to manage memory, so developers don’t need to think about ownership. However, this can lead to unpredictable pauses and higher runtime overhead. Rust avoids garbage collection by enforcing strict ownership and borrowing rules, resulting in more predictable performance.
- **Python/JavaScript:** Memory is managed automatically, and references are handled behind the scenes. There is no concept of borrowing or explicit ownership, which makes it easier to write code but can hide performance issues and bugs related to shared mutable state.

Essentially, Rust only allows data that is explicity deemed necessary to stay alive and frees things that are otherwise not needed. You must be mindful of what data is used and for how long. This to me is so cool.
### My view

I really enjoy Rust's ownership and borrowing model as it enforces constant edge-case handling and makes sure that a system is built to be solid and robust. 
Personally i wouldnt use rust for a todoapp, but i am most definitely interested in using it for bigger projects that need a robust design.

In terms of problems encountered, the most common issues were related to the borrow checker, such as:
- Trying to use a variable after it has been moved.
- Attempting to create multiple mutable references to the same data.

In places of our codebase, we have had to use deferencing to ensure that a datapoint exists for long enough while in other cases we left merely a reference to the data.

## Code Snippets

1. Using `move` in closures to transfer ownership of `career_id`:

File: [`src\ui\main_screen.rs`](../src/ui/main_screen.rs)
```rust
let career_id = career_number; // Instantiated data outside the closure

column = column.with_child(Button::new(label.clone()).on_click(
    move |ctx, data: &mut AppState, _env| { // Move takes ownership of any variables used inside the closure. 
                                            //Owner is therefore taken over and dumped by the end of the closure
        data.game_number = career_id.to_string();
        set_game_number(career_id);
        if has_selected_team() == false {
            data.current_screen = ChooseTeam;
        } else {
            let selected_team = get_selected_team(&career_id.to_string());
            data.selected_team = selected_team;
            data.current_screen = MainGameScreen;
        }

        data.show_modal = false;
        ctx.request_update();
    },
));
```

2. The `&` operator is used to create a reference to a variable, allowing us to immutably borrow it without transferring ownership.

> This is useful when we want to use a variable without taking ownership, allowing us to use it later in the code.
File: [`src\backend\race.rs`](../src/backend
```rust
let driver_lap_times =
    generate_driver_lap_times(&drivers, circuit.lap_amount, circuit.length_km);

let driver_total_times = calculate_driver_total_times(&driver_lap_times);

let driver_results = create_driver_results(&driver_total_times, &driver_lap_times);
```

3. We use `String` in structs loaded from the database because it would be too complex to manage lifetimes of string slices in structs. 
>Slices always require ownership of the data they point to

File: [`src\model\driver.rs`](../src/model/driver.rs)
```rust
pub struct Driver {
    pub id: u16,
    pub first_name: String,
    pub last_name: String,
    pub rating: u8,
    pub country_id: u8,
    pub date_of_birth: String,
    pub racing_number: u8,
    pub image_path: String,
}
```

4. We use a `{}`-scope immediately after `let tx = conn.transaction().unwrap();` because we need to drop our prepared statements before we can commit our transaction. This is because prepared statements borrow `tx`, so we cannot commit until they are dropped.

File: [`src\database\races.rs`](../src/database/races.rs)
```rust
pub fn save_driver_results(
    ...
) {
    let mut conn = get_connection().unwrap();
    let tx = conn.transaction().unwrap();
    {
        let mut stmt_race_driver_results = tx
            .prepare(
                r#"INSERT INTO race_driver_results (
                fk_season_schedule_id, fk_driver_id, fk_team_id, placement, points, status
            ) VALUES (?, ?, ?, ?, ?, ?)"#,
            )
            .unwrap();
        let mut stmt_laps = tx
            .prepare(
                r#"INSERT INTO laps (
                fk_race_driver_result_id, lap_time_ms, lap_number
            ) VALUES (?, ?, ?)"#,
            )
            .unwrap();
        ...
    }
    tx.commit().unwrap();
}
```

- Example of how ownership and moving works in Rust:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // Ownership of the string is moved to s2

    // println!("{}", s1); // Error: s1 is no longer valid
    println!("{}", s2);
}
```

- Example of how multiple immutable borrows work in Rust:

```rust
fn main() {
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2); // Allowed: multiple immutable references
}
```

- Example of how a mutable borrow works in Rust:

```rust
fn main() {
    let mut s = String::from("hello");

    let r = &mut s;
    r.push_str(", world!");
    println!("{}", r); // Allowed: single mutable reference
}
```

- Example of an error when mixing mutable and immutable borrows (causes a compilation error):

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &mut s; // Error: cannot borrow `s` as mutable because it is also borrowed as immutable

    println!("{}, {}", r1, r2);
}
```
