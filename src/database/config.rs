use crate::database::connection::get_connection;

pub fn get_current_date() -> Option<String> {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare("SELECT \"current_date\" FROM game_config")
        .unwrap();
    let row = stmt.query_row([], |row| {
        let value: String = row.get(0)?;
        Ok(value)
    });
    match row {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}
