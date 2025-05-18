use crate::util::appdata::get_game_saves_path;
use rusqlite::Connection;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::Ordering;
use std::sync::{Mutex, MutexGuard};

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

pub fn get_connection() -> Result<ConnectionGuard, String> {
    let mut conn_guard = CONNECTION
        .lock()
        .map_err(|_| "Failed to lock connection mutex".to_string())?;

    let game_number = super::GAME_NUMBER.load(Ordering::SeqCst);
    if game_number == 0 {
        return Err("Game number is not set".to_string());
    }

    if conn_guard.is_none() {
        let db_file = format!("Career_{}.db", game_number);
        let db_path = get_game_saves_path().join(db_file);
        let conn =
            Connection::open(db_path).map_err(|e| format!("Failed to open database: {}", e))?;

        *conn_guard = Some(conn);
    }

    Ok(ConnectionGuard(conn_guard))
}

pub fn delete_connection() {
    let mut conn_guard = CONNECTION.lock().unwrap();
    *conn_guard = None;
}
