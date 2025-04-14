use rusqlite::Connection;
use std::env;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard};

// Global state to hold the connection, initially None
static CONNECTION: Mutex<Option<Connection>> = Mutex::new(None);

// Custom struct to hold the MutexGuard and provide access to Connection
pub struct ConnectionGuard(MutexGuard<'static, Option<Connection>>);

impl Deref for ConnectionGuard {
    type Target = Connection;
    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
}

// Function to get or create the connection based on the career number
pub fn get_connection() -> Result<ConnectionGuard, String> {
    // Lock the mutex to safely access or modify the connection
    let mut conn_guard = CONNECTION
        .lock()
        .map_err(|_| "Failed to lock connection mutex".to_string())?;

    // Check if the connection is not yet initialized
    if conn_guard.is_none() {
        // Get the user's roaming app data directory
        let appdata = env::var("APPDATA").expect("APPDATA environment variable not found");
        let base_path = PathBuf::from(appdata).join("FormulaOneManager");
        let save_games_path = base_path.join("GameSaves");

        // Construct the database file path (e.g., Career_1.db)
        let db_file = format!("Career_{}.db", 1);
        let db_path = save_games_path.join(db_file);

        // Open the connection to the database
        let conn =
            Connection::open(db_path).map_err(|e| format!("Failed to open database: {}", e))?;
        *conn_guard = Some(conn); // Store the connection in the global state
    }

    Ok(ConnectionGuard(conn_guard))
}
