use rusqlite::Connection;
use std::env;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::sync::atomic::Ordering;
use std::sync::{Mutex, MutexGuard};

// Define the static mutex to hold the database connection
static CONNECTION: Mutex<Option<Connection>> = Mutex::new(None);

pub struct ConnectionGuard(MutexGuard<'static, Option<Connection>>);

impl Deref for ConnectionGuard {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        (*self.0).as_ref().unwrap()
    }
}

impl DerefMut for ConnectionGuard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        (*self.0).as_mut().unwrap()
    }
}

// Function to get or create the connection based on the career number
pub fn get_connection() -> Result<ConnectionGuard, String> {
    // Lock the mutex to safely access or modify the connection
    let mut conn_guard = CONNECTION
        .lock()
        .map_err(|_| "Failed to lock connection mutex".to_string())?;

    let game_number = super::GAME_NUMBER.load(Ordering::SeqCst);
    if game_number == 0 {
        // If the game number is not set, return an error
        return Err("Game number is not set".to_string());
    }

    // Check if the connection is not yet initialized
    if conn_guard.is_none() {
        // Get the user's roaming app data directory
        let appdata = env::var("APPDATA").expect("APPDATA environment variable not found");
        let base_path = PathBuf::from(appdata).join("FormulaOneManager");
        let save_games_path = base_path.join("GameSaves");

        // Construct the database file path (e.g., Career_1.db)
        let db_file = format!("Career_{}.db", game_number);
        let db_path = save_games_path.join(db_file);

        // Open the connection to the database
        let conn =
            Connection::open(db_path).map_err(|e| format!("Failed to open database: {}", e))?;
        *conn_guard = Some(conn); // Store the connection in the global state
    }

    Ok(ConnectionGuard(conn_guard))
}

pub fn delete_connection() {
    // Lock the mutex to safely access or modify the connection
    let mut conn_guard = CONNECTION.lock().unwrap();
    *conn_guard = None; // Reset the connection to force a new one on next call
}
