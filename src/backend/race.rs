use crate::database::circuit::get_circuit_by_id;
use crate::database::driver::get_all_drivers;
use crate::database::driver::get_team_id_by_driver_id;
use crate::database::race::{get_season_schedule_by_id, save_driver_results, update_race_status};
use crate::model::Lap;
use crate::model::RaceDriverResult;
use rand::Rng;

pub fn start_race(season_schedule_id: i32) {
    // 1. Hent object fra databasen
    println!("Starting get_season_schedule_by_id");
    let race = match get_season_schedule_by_id(season_schedule_id) {
        Some(race) => race,
        None => {
            println!("Race not found!");
            return;
        }
    };

    println!("Starting get_circuit_by_id");
    let circuit = match get_circuit_by_id(race.circuit_id) {
        Some(circuit) => circuit,
        None => {
            println!("Circuit not found!");
            return;
        }
    };

    // 2. Hent alle drivers fra databasen
    println!("Starting get_all_drivers");
    let drivers = get_all_drivers();

    // 3. Opret laptimes i form af Vec<DriverID, <Lap>>
    println!("Generating lap times for drivers");
    let mut driver_laps: Vec<(i32, Vec<f64>)> = Vec::new();
    for driver in drivers {
        let mut lap_times = Vec::new();
        for _ in 0..circuit.lap_amount {
            let lap_time = generate_lap_time(driver.rating as u32, circuit.length_km as f64);
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
    for (index, (driver_id, _)) in total_driver_time.iter().enumerate() {
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

        let team_id = match get_team_id_by_driver_id(&driver_id) {
            Some(team_id) => team_id,
            None => {
                println!("Team not found for driver ID: {}", driver_id);
                continue;
            }
        };

        let race_driver_result = RaceDriverResult {
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
                    lap_time_ms: (*lap_time * 1000.0) as i32, // Convert seconds to milliseconds
                    lap_number: (lap_number + 1) as i32,
                });
            }
        }

        driver_results.push((*driver_id, (race_driver_result, laps)));
    }

    // 6. Update season_schedule i databasen
    println!("updating race status to Finished");
    update_race_status(season_schedule_id, "Finished");

    // 7. Gem driver race result & laps i databasen
    println!("Saving driver results and laps to database");
    save_driver_results(season_schedule_id, driver_results);
}

fn generate_lap_time(driver_rating: u32, circuit_length: f64) -> f64 {
    // Constants for the algorithm
    const BASE_SPEED: f64 = 200.0; // Average speed in km/h
    const RATING_MAX: u32 = 100; // Maximum driver rating
    const RATING_MIN: u32 = 70; // Minimum driver rating
    const RANDOMNESS_FACTOR: f64 = 0.05; // 5% variability

    // Step 1: Calculate base lap time (in hours)
    let base_lap_time = circuit_length / BASE_SPEED;

    // Step 2: Adjust based on driver rating
    // Higher rating -> lower lap time (faster)
    let driver_factor =
        1.0 - ((driver_rating - RATING_MIN) as f64 / (RATING_MAX - RATING_MIN) as f64) * 0.02;
    let adjusted_lap_time = base_lap_time * driver_factor;

    // Step 3: Add randomness (±5% variation)
    let mut rng = rand::thread_rng();
    let random_factor = 1.0 + (rng.gen::<f64>() * 2.0 - 1.0) * RANDOMNESS_FACTOR;
    let final_lap_time = adjusted_lap_time * random_factor;

    // Step 4: Convert to seconds for output
    final_lap_time * 3600.0
}
