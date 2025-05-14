use crate::database::connection::get_connection;
use crate::model::{Lap, RaceDriverResult, RaceResult, SeasonSchedule};
use rusqlite::named_params;
use std::collections::HashMap;

pub fn get_season_schedule_by_id(season_schedule_id: i32) -> Option<SeasonSchedule> {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare("SELECT id, fk_circuit_id, date, status FROM season_schedules WHERE id = ?")
        .unwrap();
    let row = stmt.query_row([season_schedule_id], |row| {
        Ok(SeasonSchedule {
            id: row.get(0)?,
            circuit_id: row.get(1)?,
            date: row.get(2)?,
            status: row.get(3)?,
        })
    });
    match row {
        Ok(schedule) => Some(schedule),
        Err(_) => None,
    }
}

pub fn get_race_id_by_grandprix_name(grand_prix_name: &str) -> Option<i32> {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare("SELECT id FROM season_schedules WHERE grand_prix_name = ?")
        .unwrap();
    let row = stmt.query_row([grand_prix_name], |row| row.get(0));
    match row {
        Ok(id) => Some(id),
        Err(_) => None,
    }
}

pub fn update_race_status(season_schedule_id: i32, status: &str) {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare("UPDATE season_schedules SET status = ? WHERE id = ?")
        .unwrap();
    stmt.execute([status, &season_schedule_id.to_string()])
        .unwrap();
}

pub fn save_driver_results(
    season_schedule_id: i32,
    driver_results: Vec<(i32, (RaceDriverResult, Vec<Lap>))>,
) {
    let mut conn = get_connection().unwrap();
    let tx = conn.transaction().unwrap();
    {
        let mut stmt_race_driver_results = tx.prepare(
            r#"INSERT INTO race_driver_results (
                fk_season_schedule_id, fk_driver_id, fk_team_id, placement, points, status
            ) VALUES (:fk_season_schedule_id, :fk_driver_id, :fk_team_id, :placement, :points, :status)"#
        ).unwrap();
        let mut stmt_laps = tx
            .prepare(
                r#"INSERT INTO laps (
                fk_race_driver_result_id, lap_time_ms, lap_number
            ) VALUES (:fk_race_driver_result_id, :lap_time_ms, :lap_number)"#,
            )
            .unwrap();
        for (_driver_id, (race_driver_result, laps)) in driver_results {
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
            let race_driver_result_id = tx.last_insert_rowid();
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
    tx.commit().unwrap();
}

pub fn get_next_race() -> Option<SeasonSchedule> {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare(
            r#"SELECT id, fk_circuit_id, date, status
           FROM season_schedules
           WHERE status = 'Upcoming'
           ORDER BY date ASC LIMIT 1"#,
        )
        .unwrap();
    let row = stmt.query_row([], |row| {
        Ok(SeasonSchedule {
            id: row.get(0)?,
            circuit_id: row.get(1)?,
            date: row.get(2)?,
            status: row.get(3)?,
        })
    });
    match row {
        Ok(schedule) => Some(schedule),
        Err(_) => None,
    }
}

pub fn get_race_results(race_id: &i32) -> Vec<RaceResult> {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare(
            r#"SELECT 
            rdr.placement AS Position,
            d.racing_number AS DriverNumber,
            (d.first_name || ' ' || d.last_name) AS DriverName,
            t.short_name AS Team,
            rdr.points AS Points,
            COALESCE(SUM(l.lap_time_ms), 0) AS TotalTime_ms
        FROM race_driver_results rdr
        JOIN drivers d ON rdr.fk_driver_id = d.id
        JOIN teams t ON rdr.fk_team_id = t.id
        LEFT JOIN laps l ON l.fk_race_driver_result_id = rdr.id
        WHERE rdr.fk_season_schedule_id = ?
        GROUP BY rdr.id, d.id, t.id
        ORDER BY rdr.placement ASC"#,
        )
        .unwrap();
    let results = stmt
        .query_map([race_id], |row| {
            Ok(RaceResult {
                position: row.get(0)?,
                driver_number: row.get(1)?,
                driver_name: row.get(2)?,
                team: row.get(3)?,
                points: row.get(4)?,
                total_time_ms: row.get(5)?,
            })
        })
        .unwrap();
    results.filter_map(Result::ok).collect()
}

pub fn get_race_list() -> Vec<Vec<String>> {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare("SELECT selected_team FROM game_config WHERE id = 1")
        .unwrap();
    let selected_team_id: i32 = match stmt.query_row([], |row| row.get(0)) {
        Ok(id) => id,
        Err(_) => return Vec::new(),
    };
    let mut race_stmt = conn
        .prepare(
            r#"SELECT 
            ss.date AS date,
            ss.id AS schedule_id,
            ss.grand_prix_name,
            d.first_name || ' ' || d.last_name AS winner_name
        FROM season_schedules ss
        LEFT JOIN race_driver_results rdr ON ss.id = rdr.fk_season_schedule_id AND rdr.placement = 1
        LEFT JOIN drivers d ON rdr.fk_driver_id = d.id
        ORDER BY ss.date"#,
        )
        .unwrap();
    let race_rows = match race_stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, i32>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, Option<String>>(3)?,
        ))
    }) {
        Ok(rows) => rows,
        Err(_) => return Vec::new(),
    };
    let mut team_stmt = conn
        .prepare(
            r#"SELECT 
            ss.id AS schedule_id,
            rdr.placement
        FROM season_schedules ss
        JOIN race_driver_results rdr ON ss.id = rdr.fk_season_schedule_id
        WHERE rdr.fk_team_id = ?
        ORDER BY ss.date, rdr.placement"#,
        )
        .unwrap();
    let team_rows = match team_stmt.query_map([selected_team_id], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, Option<i32>>(1)?))
    }) {
        Ok(rows) => rows,
        Err(_) => return Vec::new(),
    };
    let mut race_map: HashMap<i32, (String, String, String, Vec<i32>)> = HashMap::new();
    for row in race_rows {
        if let Ok((date, schedule_id, grand_prix_name, winner_name)) = row {
            race_map.insert(
                schedule_id,
                (
                    date,
                    grand_prix_name,
                    winner_name.unwrap_or("TBD".to_string()),
                    Vec::new(),
                ),
            );
        }
    }
    for row in team_rows {
        if let Ok((schedule_id, placement)) = row {
            if let Some(entry) = race_map.get_mut(&schedule_id) {
                if let Some(pos) = placement {
                    if entry.3.len() < 2 {
                        entry.3.push(pos);
                    }
                }
            }
        }
    }
    let mut race_list: Vec<Vec<String>> = race_map
        .values()
        .map(|(date, grand_prix_name, winner_name, placements)| {
            let placements_str = if placements.is_empty() {
                "TBD".to_string()
            } else {
                placements
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            };
            vec![
                date.clone(),
                grand_prix_name.clone(),
                winner_name.clone(),
                placements_str,
            ]
        })
        .collect();
    race_list.sort_by(|a, b| a[0].cmp(&b[0]));
    race_list
}

pub fn get_race_schedule_info() -> Vec<Vec<String>> {
    let conn = get_connection().unwrap();
    let mut stmt = conn.prepare(
        r#"SELECT 
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
        ORDER BY CASE WHEN d1.first_name IS NOT NULL THEN 0 ELSE 1 END, ss.date"#
    ).unwrap();
    let rows = stmt
        .query_map([], |row| {
            Ok(vec![
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, String>(6)?,
            ])
        })
        .unwrap();
    rows.filter_map(Result::ok).collect()
}

pub fn is_next_race(race_id: i32) -> bool {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare("SELECT MIN(id) FROM season_schedules WHERE status = 'Upcoming'")
        .unwrap();
    let min_upcoming_id: Option<i32> = stmt.query_row([], |row| row.get(0)).ok();
    min_upcoming_id == Some(race_id)
}
