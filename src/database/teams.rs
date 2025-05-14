use crate::database::connection::get_connection;
use crate::model::{RaceInfo, SeasonInfo, Team, TeamBase};
use std::collections::HashMap;

pub fn get_all_teams() -> Vec<(String, String, Vec<(String, String)>)> {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare(
            r#"SELECT t.full_name, t.short_name, d.first_name, d.last_name 
           FROM teams t 
           JOIN driver_contracts dc ON t.id = dc.fk_team_id
           JOIN drivers d ON dc.fk_driver_id = d.id 
           ORDER BY t.short_name, d.last_name"#,
        )
        .unwrap();
    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        })
        .unwrap();
    let mut teams_map: HashMap<String, (String, String, Vec<(String, String)>)> = HashMap::new();
    for row in rows {
        if let Ok((full_name, short_name, first_name, last_name)) = row {
            teams_map
                .entry(short_name.clone())
                .and_modify(|(_, _, drivers)| drivers.push((first_name.clone(), last_name.clone())))
                .or_insert((full_name, short_name.clone(), vec![(first_name, last_name)]));
        }
    }
    let mut teams: Vec<_> = teams_map.into_values().collect();
    teams.sort_by(|a, b| a.1.cmp(&b.1));
    teams
}

pub fn save_selected_team(team_short_name: &str) {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare("SELECT id FROM teams WHERE short_name = ?")
        .unwrap();
    let team_id: i32 = match stmt.query_row([team_short_name], |row| row.get(0)) {
        Ok(id) => id,
        Err(_) => return,
    };
    let mut stmt = conn
        .prepare("UPDATE game_config SET selected_team = ? WHERE id = 1")
        .unwrap();
    stmt.execute([team_id]).unwrap();
}

pub fn get_selected_team(game_number: &str) -> Option<String> {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare(
            r#"SELECT t.short_name 
           FROM teams t 
           JOIN game_config c ON t.id = c.selected_team 
           WHERE c.id = ?"#,
        )
        .unwrap();
    let row = stmt.query_row([game_number], |row| row.get(0));
    match row {
        Ok(team) => Some(team),
        Err(_) => None,
    }
}

pub fn get_own_team_standing() -> Option<(String, Vec<String>, i32)> {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare(
            r#"SELECT 
            t.short_name,
            d.first_name || ' ' || d.last_name AS driver_name,
            COALESCE(SUM(rdr.points), 0) AS total_points
        FROM game_config c
        JOIN teams t ON c.selected_team = t.id
        LEFT JOIN driver_contracts dc ON t.id = dc.fk_team_id
        LEFT JOIN drivers d ON dc.fk_driver_id = d.id
        LEFT JOIN race_driver_results rdr ON d.id = rdr.fk_driver_id
        WHERE dc.date_end > strftime('%s', 'now') * 1000
        GROUP BY t.short_name, d.id
        ORDER BY total_points DESC"#,
        )
        .unwrap();
    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i32>(2)?,
            ))
        })
        .unwrap();
    let mut team_name = String::new();
    let mut drivers = Vec::new();
    let mut total_points = 0;
    for row in rows {
        if let Ok((name, driver_name, points)) = row {
            if team_name.is_empty() {
                team_name = name;
            }
            drivers.push(driver_name);
            total_points += points;
        }
    }
    if team_name.is_empty() {
        None
    } else {
        Some((team_name, drivers, total_points))
    }
}

pub fn get_top_teams_standings(limit: Option<i32>) -> Vec<Vec<String>> {
    let conn = get_connection().unwrap();
    let base_query = r#"SELECT 
        t.short_name,
        COALESCE(SUM(rdr.points), 0) AS total_points
    FROM teams t
    LEFT JOIN race_driver_results rdr ON t.id = rdr.fk_team_id
    GROUP BY t.id, t.short_name
    ORDER BY total_points DESC"#;
    let final_query = match limit {
        Some(n) => format!("{} LIMIT {}", base_query, n),
        None => base_query.to_string(),
    };
    let mut stmt = conn.prepare(&final_query).unwrap();
    let rows = stmt
        .query_map([], |row| {
            Ok(vec![
                row.get::<_, String>(0)?,
                row.get::<_, i32>(1)?.to_string(),
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

pub fn get_team_info(team_id: &i32) -> Option<Team> {
    let conn = get_connection().unwrap();
    let mut stmt = conn.prepare(
        r#"SELECT 
            id, short_name, full_name, first_entry, team_chief, chassis, power_unit, image_team, image_car
        FROM teams
        WHERE id = ?"#
    ).unwrap();
    let row = stmt.query_row([team_id], |row| {
        Ok(Team {
            id: row.get(0)?,
            short_name: row.get(1)?,
            full_name: row.get(2)?,
            first_entry: row.get(3)?,
            team_chief: row.get(4)?,
            chassis: row.get(5)?,
            power_unit: row.get(6)?,
            image_path_logo: row.get(7)?,
            image_path_car: row.get(8)?,
        })
    });
    match row {
        Ok(team) => Some(team),
        Err(_) => None,
    }
}

pub fn get_team_id_by_short_name(short_name: &str) -> Option<i32> {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare("SELECT id FROM teams WHERE short_name = ?")
        .unwrap();
    let row = stmt.query_row([short_name], |row| row.get(0));
    match row {
        Ok(id) => Some(id),
        Err(_) => None,
    }
}

pub fn get_team_id_by_full_name(full_name: &str) -> Option<i32> {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare("SELECT id FROM teams WHERE full_name = ?")
        .unwrap();
    let row = stmt.query_row([full_name], |row| row.get(0));
    match row {
        Ok(id) => Some(id),
        Err(_) => None,
    }
}

pub fn get_team_season_info(team_id: i32, season_year: i32) -> Option<SeasonInfo> {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare("SELECT id FROM seasons WHERE year = ?")
        .unwrap();
    let season_id: i32 = match stmt.query_row([season_year], |row| row.get(0)) {
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
        WHERE ss.fk_season_id = ? AND rdr.fk_team_id = ?
        ORDER BY ss.date"#,
        )
        .unwrap();
    let race_rows = match race_stmt.query_map([season_id, team_id], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, Option<i32>>(2)?,
            row.get::<_, i32>(3)?,
        ))
    }) {
        Ok(rows) => rows,
        Err(_) => return None,
    };
    let mut races = Vec::new();
    let mut race_map: HashMap<String, (String, Vec<i32>, i32)> = HashMap::new();
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
    let total_points: i32 = races.iter().map(|r| r.race_points).sum();
    let mut team_points_stmt = conn
        .prepare(
            r#"SELECT rdr.fk_team_id, SUM(rdr.points) as total_points
           FROM race_driver_results rdr
           JOIN season_schedules ss ON rdr.fk_season_schedule_id = ss.id
           WHERE ss.fk_season_id = ?
           GROUP BY rdr.fk_team_id
           ORDER BY total_points DESC"#,
        )
        .unwrap();
    let team_points_rows = match team_points_stmt.query_map([season_id], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, i32>(1)?))
    }) {
        Ok(rows) => rows,
        Err(_) => return None,
    };
    let team_points_list: Vec<(i32, i32)> = team_points_rows.filter_map(Result::ok).collect();
    let mut overall_position = 1;
    for (tid, points) in team_points_list {
        if tid == team_id {
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

pub fn get_team_data() -> Vec<Vec<String>> {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare(
            r#"SELECT 
            t.short_name,
            t.full_name,
            COALESCE((
                SELECT SUM(rdr.points)
                FROM race_driver_results rdr
                WHERE rdr.fk_team_id = t.id
            ), 0) AS total_points,
            (
                SELECT GROUP_CONCAT(d2.first_name || ' ' || d2.last_name, ',')
                FROM driver_contracts dc2
                JOIN drivers d2 ON dc2.fk_driver_id = d2.id
                WHERE dc2.fk_team_id = t.id
                AND (dc2.date_end IS NULL OR dc2.date_end > strftime('%s', 'now') * 1000)
                ORDER BY d2.last_name
            ) AS drivers
        FROM teams t
        GROUP BY t.id, t.full_name
        ORDER BY total_points DESC"#,
        )
        .unwrap();
    let team_iter = stmt
        .query_map([], |row| {
            let drivers: Option<String> = row.get(3)?;
            Ok(vec![
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i32>(2)?.to_string(),
                drivers
                    .as_ref()
                    .and_then(|d| d.split(',').next())
                    .unwrap_or("")
                    .to_string(),
                drivers
                    .as_ref()
                    .and_then(|d| d.split(',').nth(1))
                    .unwrap_or("")
                    .to_string(),
            ])
        })
        .unwrap();
    team_iter.filter_map(Result::ok).collect()
}

pub fn get_team_base_by_team_id(team_id: i32) -> Option<TeamBase> {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare(
            r#"SELECT tb.city, c.name 
           FROM team_bases tb 
           JOIN countries c ON c.id = tb.fk_country_id 
           WHERE tb.fk_team_id = ?"#,
        )
        .unwrap();
    let row = stmt.query_row([team_id], |row| {
        Ok(TeamBase {
            city: row.get(0)?,
            country_name: row.get(1)?,
        })
    });
    match row {
        Ok(base) => Some(base),
        Err(_) => None,
    }
}
