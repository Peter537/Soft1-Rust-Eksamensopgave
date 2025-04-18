use crate::{database::connection::get_connection, model::team::Team};
use std::collections::HashMap;
use crate::model::season::{RaceInfo, SeasonInfo};

pub fn get_all_teams() -> Vec<(String, String, Vec<(String, String)>)> {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare(
            "SELECT t.full_name, t.short_name, d.first_name, d.last_name 
            FROM teams t JOIN driver_contracts dc ON t.id = dc.fk_team_id
            JOIN drivers d ON dc.fk_driver_id = d.id ORDER BY t.short_name, d.last_name",
        )
        .unwrap();

    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0).unwrap(),
                row.get::<_, String>(1).unwrap(),
                row.get::<_, String>(2).unwrap(),
                row.get::<_, String>(3).unwrap(),
            ))
        })
        .unwrap();

    let mut teams_map: HashMap<String, (String, String, Vec<(String, String)>)> = HashMap::new();

    for row in rows {
        let (full_name, short_name, first_name, last_name) = row.unwrap();

        teams_map
            .entry(short_name.clone())
            .and_modify(|(_, _, drivers)| drivers.push((first_name.clone(), last_name.clone())))
            .or_insert((full_name, short_name.clone(), vec![(first_name, last_name)]));
    }

    let mut teams: Vec<(String, String, Vec<(String, String)>)> = teams_map.into_values().collect();

    teams.sort_by(|a, b| a.1.cmp(&b.1)); // sort by short_name

    teams
}

pub fn save_selected_team(team_short_name: &str) {
    // Open a connection to the SQLite database
    let conn = get_connection().unwrap();

    // First find the team ID by short name
    let mut stmt = conn
        .prepare("SELECT id FROM teams WHERE short_name = ?")
        .unwrap();

    println!("Searching for team with short name: {}", team_short_name);

    let team_id: i32 = stmt
        .query_row([team_short_name], |row| row.get(0))
        .unwrap_or(-1);
    if team_id == -1 {
        println!("Team with short name {} not found", team_short_name);
        return;
    }

    // Update the selected team in the database
    let mut stmt = conn
        .prepare("UPDATE game_config SET selected_team = ? WHERE id = 1")
        .unwrap();
    println!("Updating selected team to ID: {}", team_id);
    stmt.execute([team_id]).unwrap();
}

pub fn get_selected_team() -> Option<String> {
    // Open a connection to the SQLite database
    let conn = get_connection().unwrap();

    // Query the selected team from the database
    let mut stmt = conn.prepare("SELECT t.short_name FROM teams t JOIN game_config c ON t.id = c.selected_team WHERE c.id = 1").unwrap();

    let selected_team = stmt
        .query_row([], |row| Ok(row.get::<_, String>(0).unwrap()))
        .ok();

    if let Some(ref team) = selected_team {
        println!("Selected team: {}", team);
    } else {
        println!("No team selected or not found in the database.");
    }

    selected_team
}

pub fn get_own_team_standing() -> Option<(String, Vec<String>, i32)> {
    // Open a connection to the SQLite database
    let conn = get_connection().unwrap();

    let mut stmt = conn
        .prepare(
            r#"
        SELECT 
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
        ORDER BY total_points DESC
        "#,
        )
        .unwrap();

    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0).unwrap(),
                row.get::<_, String>(1).unwrap(),
                row.get::<_, i32>(2).unwrap(),
            ))
        })
        .unwrap();

    let mut team_name = String::new();
    let mut drivers: Vec<String> = Vec::new();
    let mut total_points = 0;

    for row in rows {
        let (name, driver_name, points) = row.unwrap();
        if team_name.is_empty() {
            team_name = name;
        }
        drivers.push(driver_name);
        total_points += points;
    }
    println!(
        "Team: {}, Drivers: {:?}, Total Points: {}",
        team_name, drivers, total_points
    );

    if team_name.is_empty() {
        None
    } else {
        Some((team_name, drivers, total_points))
    }
}

pub fn get_top_teams_standings(limit: Option<i32>) -> Option<Vec<(i32, String, i32)>> {
    // position, team, points
    let conn = get_connection().unwrap();

    let base_query = r#"
        SELECT 
            t.short_name,
            COALESCE(SUM(rdr.points), 0) AS total_points
        FROM teams t
        LEFT JOIN race_driver_results rdr ON t.id = rdr.fk_team_id
        GROUP BY t.id, t.short_name
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
                row.get::<_, String>(0)?, // short_name
                row.get::<_, i32>(1)?,    // total_points
            ))
        })
        .unwrap();

    let mut standings: Vec<(i32, String, i32)> = Vec::new();
    let mut position = 1;

    for row in rows {
        let (team_name, points) = row.unwrap();
        standings.push((position, team_name, points));
        position += 1;
    }

    println!("Top teams standings: {:?}", standings);

    if standings.is_empty() {
        None
    } else {
        Some(standings)
    }
}

pub fn get_team_info(team_id: &i32) -> Option<Team> {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare(
            r#"
            SELECT 
                id, short_name, full_name, fk_country_id, base_city, first_entry, 
                team_chief, chassis, power_unit, image_team, image_car
            FROM teams
            WHERE id = ?
            "#,
        )
        .unwrap();

    let mut rows = stmt.query([team_id]).unwrap();

    if let Some(row) = rows.next().unwrap() {
        let team = Team {
            id: row.get(0).unwrap(),
            short_name: row.get(1).unwrap(),
            full_name: row.get(2).unwrap(),
            country_id: row.get(3).unwrap(),
            base_city: row.get(4).unwrap(),
            first_entry: row.get(5).unwrap(),
            team_chief: row.get(6).unwrap(),
            chassis: row.get(7).unwrap(),
            power_unit: row.get(8).unwrap(),
            image_path_logo: row.get(9).unwrap(),
            image_path_car: row.get(10).unwrap(),
        };
        Some(team)
    } else {
        None
    }
}

pub fn get_team_id_by_short_name(short_name: &str) -> Option<i32> {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare("SELECT id FROM teams WHERE short_name = ?")
        .unwrap();

    let team_id: i32 = stmt
        .query_row([short_name], |row| row.get(0))
        .unwrap_or(-1);

    if team_id == -1 {
        println!("Team with short name: '{}' not found", short_name);
        None
    } else {
        Some(team_id)
    }
}

pub fn get_team_season_info(team_id: i32, season_year: i32) -> Option<SeasonInfo> {
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

    // Query to get race details and team results
    let mut race_stmt = match conn.prepare(
        r#"
        SELECT 
            ss.grand_prix_name,
            ss.date,
            rdr.placement,
            rdr.points
        FROM season_schedules ss
        JOIN race_driver_results rdr ON ss.id = rdr.fk_season_schedule_id
        WHERE ss.fk_season_id = ?1 AND rdr.fk_team_id = ?2
        ORDER BY ss.date
        "#,
    ) {
        Ok(stmt) => stmt,
        Err(_) => return None, // Query preparation failed
    };

    let race_rows = match race_stmt.query_map([season_id, team_id], |row| {
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

    // Calculate total points for the team
    let total_points: i32 = races.iter().map(|r| r.race_points).sum();

    // Query to get total points for all teams and calculate position
    let mut team_points_stmt = match conn.prepare(
        r#"
        SELECT rdr.fk_team_id, SUM(rdr.points) as total_points
        FROM race_driver_results rdr
        JOIN season_schedules ss ON rdr.fk_season_schedule_id = ss.id
        WHERE ss.fk_season_id = ?1
        GROUP BY rdr.fk_team_id
        ORDER BY total_points DESC
        "#,
    ) {
        Ok(stmt) => stmt,
        Err(_) => return None, // Query preparation failed
    };

    let team_points_rows = match team_points_stmt.query_map([season_id], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, i32>(1)?)) // (team_id, total_points)
    }) {
        Ok(rows) => rows,
        Err(_) => return None, // Query execution failed
    };

    let mut team_points_list: Vec<(i32, i32)> = Vec::new();
    for row in team_points_rows {
        match row {
            Ok((tid, points)) => team_points_list.push((tid, points)),
            Err(_) => continue, // Skip invalid rows
        }
    }

    // Calculate overall position
    let mut overall_position = 1;
    let mut found_team = false;
    for (tid, points) in team_points_list {
        if tid == team_id {
            found_team = true;
            break;
        }
        if points > total_points {
            overall_position += 1;
        }
    }

    // If team not found in results, return None
    if !found_team {
        return None;
    }

    Some(SeasonInfo {
        season_year,
        total_points,
        overall_position,
        races,
    })
}
