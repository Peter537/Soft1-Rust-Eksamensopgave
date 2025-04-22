use crate::database::connection::get_connection;
use crate::model::country::Country;

pub fn get_country_by_id(country_id: i32) -> Option<Country> {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare("SELECT name, image_country FROM countries WHERE id = ?")
        .unwrap();
    let row = stmt.query_row([country_id], |row| {
        let name: String = row.get(0)?;
        let image_path: String = row.get(1)?;
        Ok(Country {
            id: country_id,
            name,
            image_path, // Assuming you don't need the image path here
        })
    });
    match row {
        Ok(country) => Some(country),
        Err(_) => None,
    }
}
