use crate::database::connection::get_connection;
use crate::model::driver::Driver;
use crate::model::season::{RaceInfo, SeasonInfo};
use std::collections::HashMap;

pub fn get_driver_by_id(id: &i32) -> Option<Driver> {
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

pub fn get_all_drivers() -> Vec<Driver> {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare("SELECT d.id, d.first_name, d.last_name, d.rating, d.fk_country_id, d.date_of_birth, d.racing_number, d.image_driver
                        FROM drivers d")
        .unwrap();
    let driver_iter = stmt
        .query_map([], |row| {
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
    let mut drivers = Vec::new();
    for driver in driver_iter {
        match driver {
            Ok(driver) => drivers.push(driver),
            Err(_) => continue,
        }
    }
    drivers
}

pub fn get_team_id_by_driver_id(driver_id: &i32) -> Option<i32> {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare("SELECT fk_team_id FROM driver_contracts WHERE fk_driver_id = ?")
        .unwrap();
    let row = stmt.query_row([driver_id], |row| {
        let team_id = row.get(0)?;
        Ok(team_id)
    });
    match row {
        Ok(team_id) => Some(team_id),
        Err(_) => None,
    }
}

pub fn get_top_driver_standings(limit: Option<i32>) -> Option<Vec<(i32, String, i32)>> {
    let conn = get_connection().unwrap();

    let base_query = r#"
        SELECT 
            d.first_name || ' ' || d.last_name AS driver_name,
            COALESCE(SUM(rdr.points), 0) AS total_points
        FROM drivers d
        JOIN race_driver_results rdr ON d.id = rdr.fk_driver_id
        GROUP BY d.id, d.first_name, d.last_name
        ORDER BY total_points DESC
    "#;

    // Prepare query with optional LIMIT
    let final_query = match limit {
        Some(n) => format!("{} LIMIT {}", base_query, n),
        None => base_query.to_string(),
    };

    let mut stmt = conn.prepare(&final_query).unwrap();

    // Unified query_map logic
    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?, // driver_name
                row.get::<_, i32>(1)?,    // total_points
            ))
        })
        .unwrap();

    let mut standings: Vec<(i32, String, i32)> = Vec::new();
    let mut position = 1;

    for row in rows {
        let (driver_name, points) = row.unwrap();
        standings.push((position, driver_name, points));
        position += 1;
    }

    println!("Top drivers standings: {:?}", standings);

    if standings.is_empty() {
        None
    } else {
        Some(standings)
    }
}

pub fn get_driver_id_by_fullname(full_name: &str) -> Option<i32> {
    let conn = get_connection().unwrap();

    // select id from drivers where the full name is equal to full_name: &str
    let mut stmt = conn
        .prepare("SELECT id FROM drivers WHERE first_name || ' ' || last_name = ?")
        .unwrap();
    let row = stmt.query_row([full_name], |row| {
        let id = row.get(0)?;
        Ok(id)
    });

    match row {
        Ok(id) => Some(id),
        Err(_) => None,
    }
}

pub fn get_driver_season_info(driver_id: i32, season_year: i32) -> Option<SeasonInfo> {
    // Establish a single connection
    let conn = match get_connection() {
        Ok(conn) => conn,
        Err(_) => return None, // Connection failed
    };

    // Get season ID
    let season_id: i32 = match conn.query_row(
        "SELECT id FROM seasons WHERE year = ?1",
        [season_year],
        |row| row.get(0),
    ) {
        Ok(id) => id,
        Err(_) => return None, // Season not found
    };

    // Query to get race details and driver results
    let mut race_stmt = match conn.prepare(
        r#"
        SELECT 
            ss.grand_prix_name,
            ss.date,
            rdr.placement,
            rdr.points
        FROM season_schedules ss
        JOIN race_driver_results rdr ON ss.id = rdr.fk_season_schedule_id
        WHERE ss.fk_season_id = ?1 AND rdr.fk_driver_id = ?2
        ORDER BY ss.date
        "#,
    ) {
        Ok(stmt) => stmt,
        Err(_) => return None, // Query preparation failed
    };

    let race_rows = match race_stmt.query_map([season_id, driver_id], |row| {
        Ok((
            row.get::<_, String>(0)?,      // grand_prix_name
            row.get::<_, String>(1)?,      // date
            row.get::<_, Option<i32>>(2)?, // placement (can be NULL if DNF)
            row.get::<_, i32>(3)?,         // points
        ))
    }) {
        Ok(rows) => rows,
        Err(_) => return None, // Query execution failed
    };

    // Aggregate race results
    let mut races: Vec<RaceInfo> = Vec::new();
    let mut race_map: HashMap<String, (String, Vec<i32>, i32)> = HashMap::new();

    for row in race_rows {
        let (grand_prix_name, date, placement, points) = match row {
            Ok(row) => row,
            Err(_) => continue, // Skip invalid rows
        };
        let entry = race_map
            .entry(grand_prix_name.clone())
            .or_insert((date, Vec::new(), 0));

        // Add placement if not NULL (exclude DNF or invalid placements)
        if let Some(pos) = placement {
            entry.1.push(pos);
        }
        // Add points
        entry.2 += points;
    }

    // Convert HashMap to Vec<RaceInfo>
    for (grand_prix_name, (date, team_positions, race_points)) in race_map {
        races.push(RaceInfo {
            grand_prix_name,
            date,
            team_positions,
            race_points,
        });
    }

    // Sort races by date
    races.sort_by(|a, b| a.date.cmp(&b.date));

    let total_points: i32 = races.iter().map(|r| r.race_points).sum();

    // Query to get total points for all drivers and calculate position
    let mut driver_points_stmt = match conn.prepare(
        r#"
        SELECT rdr.fk_driver_id, SUM(rdr.points) as total_points
        FROM race_driver_results rdr
        JOIN season_schedules ss ON rdr.fk_season_schedule_id = ss.id
        WHERE ss.fk_season_id = ?1
        GROUP BY rdr.fk_driver_id
        ORDER BY total_points DESC
        "#,
    ) {
        Ok(stmt) => stmt,
        Err(_) => return None, // Query preparation failed
    };

    let driver_points_rows = match driver_points_stmt.query_map([season_id], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, i32>(1)?)) // (driver_id, total_points)
    }) {
        Ok(rows) => rows,
        Err(_) => return None, // Query execution failed
    };

    let mut driver_points_list: Vec<(i32, i32)> = Vec::new();
    for row in driver_points_rows {
        match row {
            Ok((did, points)) => driver_points_list.push((did, points)),
            Err(_) => continue, // Skip invalid rows
        }
    }

    // Calculate overall position
    let mut overall_position = 1;
    let mut found_driver = false;
    for (did, points) in driver_points_list {
        if did == driver_id {
            found_driver = true;
            break;
        }
        if points > total_points {
            overall_position += 1;
        }
    }

    // If driver not found in results, return None
    if !found_driver {
        overall_position = 1; // TODO: Handle this case appropriately, maybe an Optional
    }

    Some(SeasonInfo {
        season_year,
        total_points,
        overall_position,
        races,
    })
}

// Should be moved to the database module?
pub fn get_driver_data() -> Vec<Vec<String>> {
    let conn = get_connection().unwrap();

    let mut stmt = conn
        .prepare(
            r#"
        SELECT 
            d.first_name || ' ' || d.last_name AS driver_name,
            d.racing_number,
            d.rating,
            c.name AS country,
            t.short_name AS team
        FROM drivers d
        JOIN countries c ON d.fk_country_id = c.id
        LEFT JOIN driver_contracts dc ON dc.fk_driver_id = d.id
        LEFT JOIN teams t ON dc.fk_team_id = t.id
        WHERE dc.date_end IS NULL OR dc.date_end > strftime('%s', 'now') * 1000
        "#,
        )
        .unwrap();

    let driver_iter = stmt
        .query_map([], |row| {
            let driver_name: String = row.get(0)?;
            let racing_number: i32 = row.get(1)?; // INTEGER in schema
            let rating: i32 = row.get(2)?; // INTEGER in schema
            let country: String = row.get(3)?;
            let team: Option<String> = row.get(4)?; // Handle NULL teams

            Ok(vec![
                driver_name,
                racing_number.to_string(), // Convert i32 to String
                rating.to_string(),        // Convert i32 to String
                country,
                team.unwrap_or_default(), // Use empty string for NULL
            ])
        })
        .unwrap();

    let mut data: Vec<Vec<String>> = Vec::new();
    for driver in driver_iter {
        match driver {
            Ok(driver_data) => data.push(driver_data),
            Err(_) => continue, // Skip rows with errors
        }
    }

    println!("Driver data: {:?}", data); // Debug print

    data
}
