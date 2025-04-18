use crate::database::connection::get_connection;
use crate::model::circuit::CircuitInfo;
use crate::model::lap::Lap;
use crate::model::race_driver_result::RaceDriverResult;
use crate::model::race_driver_result::RaceResult;
use crate::model::season_schedule::SeasonSchedule;

use std::collections::HashMap;

use rusqlite::named_params;

pub fn get_season_schedule_by_id(season_schedule_id: i32) -> Option<SeasonSchedule> {
    let conn = get_connection().unwrap();
    let mut stmt = conn.prepare("SELECT id, fk_season_id, fk_circuit_id, date, status, grand_prix_name FROM season_schedules WHERE id = ?").unwrap();
    let row = stmt.query_row([season_schedule_id], |row| {
        let id = row.get(0)?;
        let season_id = row.get(1)?;
        let circuit_id = row.get(2)?;
        let date = row.get(3)?;
        let status = row.get(4)?;
        let grand_prix_name = row.get(5)?;
        Ok(SeasonSchedule {
            id,
            season_id,
            circuit_id,
            date,
            status,
            grand_prix_name,
        })
    });
    match row {
        Ok(season_schedule) => Some(season_schedule),
        Err(_) => None,
    }
}

pub fn update_race_status(season_schedule_id: i32, status: &str) {
    let conn = get_connection().unwrap();
    let _ = conn.execute(
        "UPDATE season_schedules SET status = :status WHERE id = :id",
        named_params! {
            ":status": status,
            ":id": season_schedule_id,
        },
    );
}

pub fn save_driver_results(
    season_schedule_id: i32,
    driver_results: Vec<(i32, (RaceDriverResult, Vec<Lap>))>,
) {
    // Establish connection and start transaction
    let mut conn = get_connection().unwrap();
    let tx = conn.transaction().unwrap();

    {
        // Prepare statement for race_driver_results
        let mut stmt_race_driver_results = tx
        .prepare(
            "INSERT INTO race_driver_results (
                fk_season_schedule_id,
                fk_driver_id,
                fk_team_id,
                placement,
                points,
                status
            ) VALUES (:fk_season_schedule_id, :fk_driver_id, :fk_team_id, :placement, :points, :status)"
        )
        .unwrap();

        // Prepare statement for laps
        let mut stmt_laps = tx
            .prepare(
                "INSERT INTO laps (
                fk_race_driver_result_id,
                lap_time_ms,
                lap_number
            ) VALUES (:fk_race_driver_result_id, :lap_time_ms, :lap_number)",
            )
            .unwrap();

        // Process each driver
        for (_driver_id, (race_driver_result, laps)) in driver_results {
            // Insert race_driver_results record
            stmt_race_driver_results
                .execute(named_params! {
                    ":fk_season_schedule_id": season_schedule_id,
                    ":fk_driver_id": race_driver_result.driver_id,
                    ":fk_team_id": race_driver_result.team_id,
                    ":placement": race_driver_result.placement,
                    ":points": race_driver_result.points,
                    ":status": race_driver_result.status,
                })
                .unwrap();

            // Get the generated ID
            let race_driver_result_id = tx.last_insert_rowid();

            // Insert all laps for this driver
            for lap in laps {
                stmt_laps
                    .execute(named_params! {
                        ":fk_race_driver_result_id": race_driver_result_id,
                        ":lap_time_ms": lap.lap_time_ms,
                        ":lap_number": lap.lap_number,
                    })
                    .unwrap();
            }
        }
    }

    // Commit the transaction
    tx.commit().unwrap();
}

pub fn get_next_race() -> Option<SeasonSchedule> {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare(
            "SELECT id, fk_season_id, fk_circuit_id, date, status, grand_prix_name FROM season_schedules WHERE status = 'Upcoming' ORDER BY date ASC LIMIT 1",
        )
        .unwrap();
    let row = stmt.query_row([], |row| {
        let id: i32 = row.get(0)?;
        let season_id: i32 = row.get(1)?;
        let circuit_id: i32 = row.get(2)?;
        let date: String = row.get(3)?;
        let status: String = row.get(4)?;
        let grand_prix_name: String = row.get(5)?;
        Ok(SeasonSchedule {
            id,
            season_id,
            circuit_id,
            date,
            status,
            grand_prix_name,
        })
    });
    match row {
        Ok(season_schedule) => Some(season_schedule),
        Err(_) => None,
    }
}

pub fn get_circuit_info(race_id: &i32) -> Option<CircuitInfo> {
    let conn = get_connection().unwrap();

    let mut stmt = conn
        .prepare(
            r#"
        SELECT 
            c.name AS CircuitName,
            (c.city || ', ' || co.name) AS Location,
            c.length_km AS LengthOfCircuit,
            c.lap_amount AS LapAmount,
            c.image_circuit AS ImagePath
        FROM season_schedules ss
        JOIN circuits c ON ss.fk_circuit_id = c.id
        JOIN countries co ON c.fk_country_id = co.id
        WHERE ss.id = ?
        "#,
        )
        .unwrap();

    let circuit = stmt
        .query_row([&race_id], |row| {
            Ok(CircuitInfo {
                circuit_name: row.get(0)?,
                location: row.get(1)?,
                length_km: row.get(2)?,
                lap_amount: row.get(3)?,
                image_path: row.get(4)?,
            })
        })
        .unwrap();

    Some(circuit)
}

fn get_race_results(race_id: i32) -> Option<RaceResult> {
    let conn = get_connection().unwrap();

    let mut stmt = conn
        .prepare(
            r#"
        SELECT 
            rdr.placement AS Position,
            d.racing_number AS DriverNumber,
            (d.first_name || ' ' || d.last_name) AS DriverName,
            t.full_name AS Team,
            MIN(l.lap_time_ms) AS FastestLapTime_ms
        FROM race_driver_results rdr
        JOIN drivers d ON rdr.fk_driver_id = d.id
        JOIN teams t ON rdr.fk_team_id = t.id
        LEFT JOIN laps l ON l.fk_race_driver_result_id = rdr.id
        WHERE rdr.fk_season_schedule_id = ?
        GROUP BY rdr.id, d.id, t.id
        ORDER BY rdr.placement ASC
        "#,
        )
        .unwrap();

    let result = stmt
        .query_row([race_id], |row| {
            Ok(RaceResult {
                position: row.get(0)?,
                driver_number: row.get(1)?,
                driver_name: row.get(2)?,
                team: row.get(3)?,
                fastest_lap_time_ms: row.get(4)?,
            })
        })
        .unwrap();

    Some(result)
}


pub fn get_race_list() -> Option<Vec<(String, String, String)>> {
    let conn = match get_connection() {
        Ok(conn) => conn,
        Err(_) => return None, // Connection failed
    };

    // Get the selected team from game_config
    let selected_team_id: i32 = match conn.query_row(
        "SELECT selected_team FROM game_config WHERE id = 1",
        [],
        |row| row.get(0),
    ) {
        Ok(id) => id,
        Err(_) => return None, // No config or selected team
    };

    // Query to get all race details (including future races) and winner
    let mut race_stmt = match conn.prepare(
        r#"
        SELECT 
            ss.id AS schedule_id,
            ss.grand_prix_name,
            d.first_name || ' ' || d.last_name AS winner_name
        FROM season_schedules ss
        LEFT JOIN race_driver_results rdr ON ss.id = rdr.fk_season_schedule_id AND rdr.placement = 1
        LEFT JOIN drivers d ON rdr.fk_driver_id = d.id
        ORDER BY CASE WHEN d.first_name IS NOT NULL THEN 0 ELSE 1 END, ss.date
        "#,
    ) {
        Ok(stmt) => stmt,
        Err(_) => return None, // Query preparation failed
    };

    let race_rows = match race_stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,         // schedule_id
            row.get::<_, String>(1)?,      // grand_prix_name
            row.get::<_, Option<String>>(2)?, // winner_name (NULL for future races)
        ))
    }) {
        Ok(rows) => rows,
        Err(_) => return None, // Query execution failed
    };

    // Query to get selected team's driver placements for completed races
    let mut team_stmt = match conn.prepare(
        r#"
        SELECT 
            ss.id AS schedule_id,
            rdr.placement
        FROM season_schedules ss
        JOIN race_driver_results rdr ON ss.id = rdr.fk_season_schedule_id
        WHERE rdr.fk_team_id = ?1
        ORDER BY ss.date, rdr.placement
        "#,
    ) {
        Ok(stmt) => stmt,
        Err(_) => return None, // Query preparation failed
    };

    let team_rows = match team_stmt.query_map([selected_team_id], |row| {
        Ok((
            row.get::<_, i32>(0)?,         // schedule_id
            row.get::<_, Option<i32>>(1)?, // placement
        ))
    }) {
        Ok(rows) => rows,
        Err(_) => return None, // Query execution failed
    };

    // Aggregate race details
    let mut race_map: HashMap<i32, (String, String, Vec<i32>)> = HashMap::new();
    for row in race_rows {
        let (schedule_id, grand_prix_name, winner_name) = match row {
            Ok(row) => row,
            Err(_) => continue, // Skip invalid rows
        };
        race_map.insert(
            schedule_id,
            (
                grand_prix_name,
                winner_name.unwrap_or("TBD".to_string()), // "TBD" for future races
                Vec::new(),
            ),
        );
    }

    // Add team placements (up to two) for completed races
    for row in team_rows {
        let (schedule_id, placement) = match row {
            Ok(row) => row,
            Err(_) => continue, // Skip invalid rows
        };
        if let Some(entry) = race_map.get_mut(&schedule_id) {
            if let Some(pos) = placement {
                if entry.2.len() < 2 {
                    // Only add up to two positions
                    entry.2.push(pos);
                }
            }
        }
    }

    // Convert to Vec<(String, String, String)>
    let mut race_list: Vec<(String, String, String)> = race_map
        .iter()
        .map(|(&schedule_id, (grand_prix_name, winner_name, placements))| {
            let placements_str = if placements.is_empty() {
                "TBD".to_string()
            } else {
                placements
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            };
            (
                grand_prix_name.clone(),
                winner_name.clone(),
                placements_str,
            )
        })
        .collect();

    // sort race list, by haveing tbd at the end
    race_list.sort_by(|a, b| {
        if a.1 == "TBD" && b.1 != "TBD" {
            std::cmp::Ordering::Greater
        } else if a.1 != "TBD" && b.1 == "TBD" {
            std::cmp::Ordering::Less
        } else {
            a.0.cmp(&b.0) // Sort by grand_prix_name if both are not "TBD"
        }
    });

    if race_list.is_empty() {
        return None;
    }

    Some(race_list)
}

pub fn get_race_schedule_info() -> Option<Vec<(String, String, String, String, String, String, String)>> {
    let conn = get_connection().unwrap();

    let mut stmt = conn.prepare(
        r#"
        SELECT 
            ss.date,
            c.name AS country_name,
            ss.grand_prix_name,
            ss.status,
            COALESCE(d1.first_name || ' ' || d1.last_name, 'TBD') AS pos1_name,
            COALESCE(d2.first_name || ' ' || d2.last_name, 'TBD') AS pos2_name,
            COALESCE(d3.first_name || ' ' || d3.last_name, 'TBD') AS pos3_name
        FROM season_schedules ss
        JOIN circuits ci ON ss.fk_circuit_id = ci.id
        JOIN countries c ON ci.fk_country_id = c.id
        LEFT JOIN race_driver_results rdr1 ON ss.id = rdr1.fk_season_schedule_id AND rdr1.placement = 1
        LEFT JOIN drivers d1 ON rdr1.fk_driver_id = d1.id
        LEFT JOIN race_driver_results rdr2 ON ss.id = rdr2.fk_season_schedule_id AND rdr2.placement = 2
        LEFT JOIN drivers d2 ON rdr2.fk_driver_id = d2.id
        LEFT JOIN race_driver_results rdr3 ON ss.id = rdr3.fk_season_schedule_id AND rdr3.placement = 3
        LEFT JOIN drivers d3 ON rdr3.fk_driver_id = d3.id
        ORDER BY CASE WHEN d1.first_name IS NOT NULL THEN 0 ELSE 1 END, ss.date
        "#,
    ).unwrap();

    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?, 
            row.get::<_, String>(1)?, 
            row.get::<_, String>(2)?, 
            row.get::<_, String>(3)?, 
            row.get::<_, String>(4)?, 
            row.get::<_, String>(5)?, 
            row.get::<_, String>(6)?,
        ))
    }).unwrap();

    let mut race_list: Vec<(String, String, String, String, String, String, String)> = Vec::new();
    for row in rows {
        match row {
            Ok((date, country_name, grand_prix_name, status, pos1_name, pos2_name, pos3_name)) => {
                race_list.push((
                    date,
                    country_name,
                    grand_prix_name,
                    status,
                    pos1_name,
                    pos2_name,
                    pos3_name,
                ));
            }
            Err(_) => continue,
        }
    }

    if race_list.is_empty() {
        return None;
    }

    Some(race_list)
}