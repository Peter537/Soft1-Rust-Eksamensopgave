use crate::database::connection::get_connection;
use crate::model::driver::Driver;

pub fn get_driver_by_id(id: i32) -> Option<Driver> {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare("SELECT d.id, d.first_name, d.last_name, d.rating, d.fk_country_id, d.date_of_birth, d.racing_number, d.image_driver
                        FROM drivers d
                        WHERE d.id = ?")
        .unwrap();
    let row = stmt
        .query_row([id], |row| {
            let id = row.get(0)?;
            let first_name = row.get(1)?;
            let last_name = row.get(2)?;
            let rating = row.get(3)?;
            let country_id = row.get(4)?;
            let date_of_birth = row.get(5)?;
            let racing_number = row.get(6)?;
            let image_path = row.get(7)?;
            Ok(Driver {
                id,
                first_name,
                last_name,
                rating,
                country_id,
                date_of_birth,
                racing_number,
                image_path,
            })
        })
        .unwrap();
    Some(row)
}
