use crate::database::connection::get_connection;
use std::collections::HashMap;


pub fn get_all_teams() -> Vec<(String, String, Vec<(String, String)>)>{
    let conn = get_connection().unwrap();
    let mut stmt = conn.prepare(
        "SELECT t.full_name, t.short_name, d.first_name, d.last_name 
            FROM teams t JOIN driver_contracts dc ON t.id = dc.fk_team_id
            JOIN drivers d ON dc.fk_driver_id = d.id ORDER BY t.short_name, d.last_name").unwrap();

    let rows = stmt.query_map([], |row| {
        Ok((
                row.get::<_, String>(0).unwrap(),
                row.get::<_, String>(1).unwrap(),
                row.get::<_, String>(2).unwrap(),
                row.get::<_, String>(3).unwrap(),
        ))
    }).unwrap();

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