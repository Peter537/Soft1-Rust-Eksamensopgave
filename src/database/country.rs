use crate::database::connection::get_connection;

pub fn get_country_image_path(country_id: i32) -> Option<String> {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare("SELECT image_country FROM countries WHERE id = ?")
        .unwrap();
    let row = stmt.query_row([country_id], |row| {
        let image_path: String = row.get(0)?;
        Ok(image_path)
    });
    match row {
        Ok(image_path) => Some(image_path),
        Err(_) => None,
    }
}
