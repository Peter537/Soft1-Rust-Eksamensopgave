use crate::database::connection::get_connection;
use crate::model::{Driver, DriverContract, RaceInfo, SeasonInfo};
use std::collections::HashMap;

pub fn get_driver_by_id(id: &u16) -> Option<Driver> {
    let conn = get_connection().unwrap();
    let mut stmt = conn.prepare(
        r#"SELECT id, first_name, last_name, rating, fk_country_id, date_of_birth, racing_number, image_driver
           FROM drivers WHERE id = ?"#
    ).unwrap();
    let row = stmt.query_row([id], |row| {
        Ok(Driver {
            id: row.get(0)?,
            first_name: row.get(1)?,
            last_name: row.get(2)?,
            rating: row.get(3)?,
            country_id: row.get(4)?,
            date_of_birth: row.get(5)?,
            racing_number: row.get(6)?,
            image_path: row.get(7)?,
        })
    });
    match row {
        Ok(driver) => Some(driver),
        Err(_) => None,
    }
}

pub fn get_all_drivers() -> Vec<Driver> {
    let conn = get_connection().unwrap();
    let mut stmt = conn.prepare(
        r#"SELECT id, first_name, last_name, rating, fk_country_id, date_of_birth, racing_number, image_driver
           FROM drivers"#
    ).unwrap();
    let driver_iter = stmt
        .query_map([], |row| {
            Ok(Driver {
                id: row.get(0)?,
                first_name: row.get(1)?,
                last_name: row.get(2)?,
                rating: row.get(3)?,
                country_id: row.get(4)?,
                date_of_birth: row.get(5)?,
                racing_number: row.get(6)?,
                image_path: row.get(7)?,
            })
        })
        .unwrap();
    driver_iter.filter_map(Result::ok).collect()
}

pub fn get_team_id_by_driver_id(driver_id: &u16) -> Option<u16> {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare("SELECT fk_team_id FROM driver_contracts WHERE fk_driver_id = ?")
        .unwrap();
    let row = stmt.query_row([driver_id], |row| row.get(0));
    match row {
        Ok(team_id) => Some(team_id),
        Err(_) => None,
    }
}

pub fn get_top_driver_standings(limit: Option<u8>) -> Vec<Vec<String>> {
    let conn = get_connection().unwrap();
    let base_query = r#"
        SELECT 
            d.first_name || ' ' || d.last_name AS driver_name,
            COALESCE(SUM(rdr.points), 0) AS total_points
        FROM drivers d
        LEFT JOIN race_driver_results rdr ON d.id = rdr.fk_driver_id
        GROUP BY d.id, d.first_name, d.last_name
        ORDER BY total_points DESC
    "#;
    let final_query = match limit {
        Some(n) => format!("{} LIMIT {}", base_query, n),
        None => base_query.to_string(),
    };
    let mut stmt = conn.prepare(&final_query).unwrap();
    let rows = stmt
        .query_map([], |row| {
            Ok(vec![
                row.get::<_, String>(0)?,          // driver_name
                row.get::<_, i32>(1)?.to_string(), // total_points
            ])
        })
        .unwrap();
    let mut standings = Vec::new();
    let mut position = 1;
    for row in rows {
        if let Ok(mut row_vec) = row {
            row_vec.insert(0, position.to_string());
            standings.push(row_vec);
            position += 1;
        }
    }
    standings
}

pub fn get_driver_id_by_fullname(full_name: &str) -> Option<u16> {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare("SELECT id FROM drivers WHERE first_name || ' ' || last_name = ?")
        .unwrap();
    let row = stmt.query_row([full_name], |row| row.get(0));
    match row {
        Ok(id) => Some(id),
        Err(_) => None,
    }
}

pub fn get_driver_season_info(driver_id: &u16, season_year: &u16) -> Option<SeasonInfo> {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare("SELECT id FROM seasons WHERE year = ?")
        .unwrap();
    let season_id = match stmt.query_row([season_year], |row| row.get(0)) {
        Ok(id) => id,
        Err(_) => return None,
    };
    let mut race_stmt = conn
        .prepare(
            r#"SELECT 
            ss.grand_prix_name,
            ss.date,
            rdr.placement,
            rdr.points
        FROM season_schedules ss
        JOIN race_driver_results rdr ON ss.id = rdr.fk_season_schedule_id
        WHERE ss.fk_season_id = ? AND rdr.fk_driver_id = ?
        ORDER BY ss.date"#,
        )
        .unwrap();
    let race_rows = match race_stmt.query_map([season_id, *driver_id], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, Option<u8>>(2)?,
            row.get::<_, u16>(3)?,
        ))
    }) {
        Ok(rows) => rows,
        Err(_) => return None,
    };
    let mut races = Vec::new();
    let mut race_map: HashMap<String, (String, Vec<u8>, u16)> = HashMap::new();
    for row in race_rows {
        if let Ok((grand_prix_name, date, placement, points)) = row {
            let entry = race_map
                .entry(grand_prix_name.clone())
                .or_insert((date, Vec::new(), 0));
            if let Some(pos) = placement {
                entry.1.push(pos);
            }
            entry.2 += points;
        }
    }
    for (grand_prix_name, (date, team_positions, race_points)) in race_map {
        races.push(RaceInfo {
            grand_prix_name,
            date,
            team_positions,
            race_points,
        });
    }
    races.sort_by(|a, b| a.date.cmp(&b.date));
    let total_points: u16 = races.iter().map(|r| r.race_points).sum();
    let mut driver_points_stmt = conn
        .prepare(
            r#"SELECT rdr.fk_driver_id, SUM(rdr.points) as total_points
           FROM race_driver_results rdr
           JOIN season_schedules ss ON rdr.fk_season_schedule_id = ss.id
           WHERE ss.fk_season_id = ?
           GROUP BY rdr.fk_driver_id
           ORDER BY total_points DESC"#,
        )
        .unwrap();
    let driver_points_rows = match driver_points_stmt.query_map([season_id], |row| {
        Ok((row.get::<_, u16>(0)?, row.get::<_, u16>(1)?))
    }) {
        Ok(rows) => rows,
        Err(_) => return None,
    };
    let driver_points_list: Vec<(u16, u16)> = driver_points_rows.filter_map(Result::ok).collect();
    let mut overall_position = 1;
    for (did, points) in driver_points_list {
        if did == *driver_id {
            break;
        }
        if points > total_points {
            overall_position += 1;
        }
    }
    Some(SeasonInfo {
        total_points,
        overall_position,
        races,
    })
}

pub fn get_driver_data() -> Vec<Vec<String>> {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare(
            r#"SELECT 
            d.first_name || ' ' || d.last_name AS driver_name,
            d.racing_number,
            d.rating,
            c.name AS country,
            t.short_name AS team
        FROM drivers d
        JOIN countries c ON d.fk_country_id = c.id
        LEFT JOIN driver_contracts dc ON dc.fk_driver_id = d.id
        LEFT JOIN teams t ON dc.fk_team_id = t.id
        WHERE dc.date_end IS NULL OR dc.date_end > strftime('%s', 'now') * 1000"#,
        )
        .unwrap();
    let driver_iter = stmt
        .query_map([], |row| {
            Ok(vec![
                row.get::<_, String>(0)?,
                row.get::<_, i32>(1)?.to_string(),
                row.get::<_, i32>(2)?.to_string(),
                row.get::<_, String>(3)?,
                row.get::<_, Option<String>>(4)?.unwrap_or_default(),
            ])
        })
        .unwrap();
    driver_iter.filter_map(Result::ok).collect()
}

pub fn get_driver_contract(driver_id: &u16) -> Option<DriverContract> {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare(
            r#"SELECT date_begin, date_end, monthly_wage
           FROM driver_contracts
           WHERE fk_driver_id = ?"#,
        )
        .unwrap();
    let row = stmt.query_row([driver_id], |row| {
        Ok(DriverContract {
            date_begin: row.get(0)?,
            date_end: row.get(1)?,
            monthly_wage: row.get(2)?,
        })
    });
    match row {
        Ok(contract) => Some(contract),
        Err(_) => None,
    }
}
