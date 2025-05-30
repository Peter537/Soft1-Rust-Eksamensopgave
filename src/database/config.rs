use crate::database::connection::get_connection;
use chrono::NaiveDate;

pub fn get_current_date() -> Option<NaiveDate> {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare("SELECT \"current_date\" FROM game_config")
        .unwrap();
    let row = stmt.query_row([], |row| {
        let value: String = row.get(0)?;
        Ok(value)
    });
    match row {
        Ok(date_str) => NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").ok(),
        Err(_) => None,
    }
}

pub fn update_current_date(date: &NaiveDate) {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare("UPDATE game_config SET \"current_date\" = ?")
        .unwrap();
    stmt.execute([date.to_string()]).unwrap();
}

pub fn has_selected_team() -> bool {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare("SELECT COUNT(*) FROM game_config WHERE selected_team IS NOT NULL")
        .unwrap();
    let row = stmt.query_row([], |row| {
        let count: i32 = row.get(0)?;
        Ok(count)
    });
    match row {
        Ok(count) => count > 0,
        Err(_) => false,
    }
}
