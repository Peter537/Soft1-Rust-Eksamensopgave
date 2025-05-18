use connection::delete_connection;
use std::sync::atomic::{AtomicU16, Ordering};

pub mod circuit;
pub mod config;
mod connection;
pub mod country;
pub mod driver;
pub mod race;
pub mod teams;

static GAME_NUMBER: AtomicU16 = AtomicU16::new(0);

pub fn set_game_number(number: u16) {
    GAME_NUMBER.store(number, Ordering::SeqCst);
    delete_connection();
}
