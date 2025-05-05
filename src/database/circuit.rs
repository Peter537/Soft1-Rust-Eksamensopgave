use crate::database::connection::get_connection;
use crate::model::Circuit;

pub fn get_circuit_by_id(circuit_id: i32) -> Option<Circuit> {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare("SELECT name, fk_country_id, city, length_km, lap_amount, image_circuit FROM circuits WHERE id = ?")
        .unwrap();
    let row = stmt.query_row([circuit_id], |row| {
        let name = row.get(0)?;
        let country_id = row.get(1)?;
        let city = row.get(2)?;
        let length_km = row.get(3)?;
        let lap_amount = row.get(4)?;
        let image_path = row.get(5)?;
        Ok(Circuit {
            name,
            country_id,
            city,
            length_km,
            lap_amount,
            image_path,
        })
    });
    match row {
        Ok(circuit) => Some(circuit),
        Err(_) => None,
    }
}
