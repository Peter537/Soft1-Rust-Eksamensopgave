use connection::delete_connection;
use std::sync::atomic::{AtomicI32, Ordering};

pub mod circuit;
pub mod config;
mod connection;
pub mod country;
pub mod driver;
pub mod race;
pub mod teams;

static GAME_NUMBER: AtomicI32 = AtomicI32::new(0); // Single source of truth for GAME_NUMBER

pub fn set_game_number(number: i32) {
    // Set the game number in the static variable
    GAME_NUMBER.store(number, Ordering::SeqCst);
    delete_connection();
}
