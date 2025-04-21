## Game Logic

1. Kalender / Next Action
2. Race logic
3. Screens
4. DB Logic
5. Load/Create Game

## Screens

### Home Screen

1. Create new Save Game => Choose Team Screen
2. Load existing Save Game => Main Game Screen
3. Delete Save Game

### Choose Team Screen

1. Select a Team from a list of available teams
2. Create Game Save => Main Game Screen

### Main Game Screen

1. 'Next Turn' button to advance the game

### Race Screen

1. Display result (if race is done)

### Driver Screen

1. Display driver ratings
2. Display season stats (team, position, points, races, etc.)
3. Display career stats (team, position, points, etc.)

### Team Screen

1. Display season stats (drivers, position, points, races, etc.)
2. Display career stats (drivers, position, points, etc.)

### Leaderboard Screen

1. Display the current leaderboards (individual & team rankings)

### Season Overview Screen

1. Display the Races and Winners
2. Display top 3(?) of leaderboard (individual & team rankings)

---

## Exam Questions & Where to add

### 1 Error Handling and Enums

Ift. Result<>, så kan vi bruge det til når der kan ske fejl, ex. ved `/util/image_loader.rs` hvis billede-lokationen ikke eksisterer.

Ift. Option<>, så kan vi bruge det i database-metoderne hvis nu der ikke er nogen data i databasen.

Alle steder med `.expect()` skal vi fjerne og fejlhåndtere ordentligt.

### 2. Ownership and Borrowing Concepts

### 3. Strings and String Handling in Rust

### 4. Polymorphism

### 5. Memory Management

### 6. Pattern Matching and Control Flow

Vi bruger f.eks. pattern-matching i mod.rs' `build_ui` for at vælge hvilken UI der skal vises.

### 7. Structs and Data Organization

Vi bruger Structs når vi henter noget ud fra databasen.

Vi bruger Tuples/Arrays når det er små-ting inde i koden som skal flyttes rundt på.

### 8. Module System and Code Organization

In each directory, we have a file `mod.rs` which is the entry point for that directory.

Vi har en `mod.rs` fil i hver mappe, som er entry-pointet for den mappe. Inde i den fil vælger vi så hvad som andre mapper skal have adgang til (ved `pub mod`), og hvad skal være privat for mappen (ved `mod`).

### 9. Concurrency in Rust

Vi bruger Mutex i database-connection håndteringen.
