use crate::database::connection::get_connection;
use std::collections::HashMap;

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
