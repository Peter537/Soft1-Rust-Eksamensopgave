use crate::database;

use crate::backend::lap;

use crate::model::lap::Lap;
use crate::model::race_driver_result::RaceDriverResult;
use crate::model::season;

pub fn start_race(season_schedule_id: i32) {
    // 1. Hent object fra databasen
    println!("Starting get_season_schedule_by_id");
    let race = match database::race::get_season_schedule_by_id(season_schedule_id) {
        Some(race) => race,
        None => {
            println!("Race not found!");
            return;
        }
    };

    println!("Starting get_circuit_by_id");
    let circuit = match database::circuit::get_circuit_by_id(race.circuit_id) {
        Some(circuit) => circuit,
        None => {
            println!("Circuit not found!");
            return;
        }
    };

    // 2. Hent alle drivers fra databasen
    println!("Starting get_all_drivers");
    let drivers = database::driver::get_all_drivers();

    // 3. Opret laptimes i form af Vec<DriverID, <Lap>>
    println!("Generating lap times for drivers");
    let mut driver_laps: Vec<(i32, Vec<f64>)> = Vec::new();
    for driver in drivers {
        let mut lap_times = Vec::new();
        for _ in 0..circuit.lap_amount {
            let lap_time = lap::generate_lap_time(driver.rating as u32, circuit.length_km as f64);
            lap_times.push(lap_time);
        }
        driver_laps.push((driver.id, lap_times));
    }

    // 4. Find top-10 drivers ud fra lap times
    println!("Calculating total lap times for drivers");
    let mut total_driver_time: Vec<(i32, f64)> = Vec::new();
    for (driver_id, laps) in driver_laps.iter() {
        let total_time: f64 = laps.iter().sum();
        total_driver_time.push((*driver_id, total_time));
    }
    total_driver_time.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    // 5. Create driver result objects
    println!("Creating driver results and laps");
    let mut driver_results: Vec<(i32, (RaceDriverResult, Vec<Lap>))> = Vec::new();
    for (index, (driver_id, total_time)) in total_driver_time.iter().enumerate() {
        let placement = (index + 1) as i32;
        let points = match placement {
            1 => 25,
            2 => 18,
            3 => 15,
            4 => 12,
            5 => 10,
            6 => 8,
            7 => 6,
            8 => 4,
            9 => 2,
            10 => 1,
            _ => 0,
        };

        let team_id = match database::driver::get_team_id_by_driver_id(&driver_id) {
            Some(team_id) => team_id,
            None => {
                println!("Team not found for driver ID: {}", driver_id);
                continue;
            }
        };

        let race_driver_result = RaceDriverResult {
            id: 0,                       // Generate or fetch unique ID
            season_schedule_id: race.id, // Assuming race.id exists
            driver_id: *driver_id,
            team_id: team_id,
            placement,
            points,
            status: "Finished".to_string(),
        };

        let mut laps = Vec::new();
        if let Some((_, lap_times)) = driver_laps.iter().find(|(id, _)| id == driver_id) {
            for (lap_number, lap_time) in lap_times.iter().enumerate() {
                laps.push(Lap {
                    id: 0,                                        // Generate or fetch unique ID
                    race_driver_result_id: race_driver_result.id, // Will need to update after result is saved
                    lap_time_ms: (*lap_time * 1000.0) as i32,     // Convert seconds to milliseconds
                    lap_number: (lap_number + 1) as i32,
                });
            }
        }

        driver_results.push((*driver_id, (race_driver_result, laps)));
    }

    // 6. Update season_schedule i databasen
    println!("updating race status to Finished");
    database::race::update_race_status(season_schedule_id, "Finished");

    // 7. Gem driver race result & laps i databasen
    println!("Saving driver results and laps to database");
    database::race::save_driver_results(season_schedule_id, driver_results);
}
