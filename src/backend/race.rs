use crate::database::circuit::get_circuit_by_id;
use crate::database::driver::get_all_drivers;
use crate::database::driver::get_team_id_by_driver_id;
use crate::database::race::{get_season_schedule_by_id, save_driver_results, update_race_status};
use crate::model::{Driver, Lap, RaceDriverResult};
use rand::Rng;

const BASE_SPEED: f32 = 200.0; // Average speed in km/h
const RATING_MAX: u8 = 100; // Maximum driver rating
const RATING_MIN: u8 = 70; // Minimum driver rating
const RANDOMNESS_FACTOR: f32 = 0.05; // 5% variability

const POINTS: [u16; 10] = [25, 18, 15, 12, 10, 8, 6, 4, 2, 1];

pub fn start_race(season_schedule_id: u16) {
    let race = get_season_schedule_by_id(&season_schedule_id).unwrap();
    let circuit = get_circuit_by_id(&race.circuit_id).unwrap();
    let drivers = get_all_drivers();

    let driver_lap_times =
        generate_driver_lap_times(&drivers, circuit.lap_amount, circuit.length_km);

    let driver_total_times = calculate_driver_total_times(&driver_lap_times);

    let driver_results = create_driver_results(&driver_total_times, &driver_lap_times);

    save_driver_results(season_schedule_id, driver_results);
    update_race_status(season_schedule_id, "Finished");
}

fn generate_driver_lap_times(
    drivers: &[Driver],
    lap_amount: u8,
    circuit_length: f32,
) -> Vec<(u16, Vec<f32>)> {
    let mut driver_lap_times = Vec::new();
    for driver in drivers {
        let mut lap_times = Vec::new();
        for _ in 0..lap_amount {
            let lap_time = generate_lap_time(driver.rating, circuit_length);
            lap_times.push(lap_time);
        }
        driver_lap_times.push((driver.id, lap_times));
    }
    driver_lap_times
}

fn calculate_driver_total_times(driver_lap_times: &[(u16, Vec<f32>)]) -> Vec<(u16, f32)> {
    let mut driver_total_times = Vec::new();
    for (driver_id, laps) in driver_lap_times {
        let total_time: f32 = laps.iter().sum();
        driver_total_times.push((*driver_id, total_time));
    }
    driver_total_times.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    driver_total_times
}

fn create_driver_results(
    driver_total_times: &[(u16, f32)],
    driver_lap_times: &[(u16, Vec<f32>)],
) -> Vec<(u16, (RaceDriverResult, Vec<Lap>))> {
    let mut driver_results = Vec::new();
    for (index, (driver_id, _)) in driver_total_times.iter().enumerate() {
        let placement = (index + 1) as u8;
        let points = get_points(placement);
        let team_id = get_team_id_by_driver_id(driver_id).unwrap();

        let race_driver_result = RaceDriverResult {
            driver_id: *driver_id,
            team_id,
            placement,
            points,
            status: "Finished".to_string(),
        };

        let mut laps = Vec::new();
        if let Some((_, lap_times)) = driver_lap_times.iter().find(|(id, _)| id == driver_id) {
            for (lap_number, lap_time) in lap_times.iter().enumerate() {
                laps.push(Lap {
                    lap_time_ms: (*lap_time * 1000.0) as u32,
                    lap_number: (lap_number + 1) as u8,
                });
            }
        }

        driver_results.push((*driver_id, (race_driver_result, laps)));
    }
    driver_results
}

fn get_points(placement: u8) -> u16 {
    if placement <= 10 {
        POINTS[(placement - 1) as usize]
    } else {
        0
    }
}

fn generate_lap_time(driver_rating: u8, circuit_length: f32) -> f32 {
    // calculate base lap time (in hours)
    let base_lap_time = circuit_length / BASE_SPEED;

    // adjust based on driver rating, higher rating -> lower lap time (faster)
    let driver_factor =
        1.0 - ((driver_rating - RATING_MIN) as f32 / (RATING_MAX - RATING_MIN) as f32) * 0.02;
    let adjusted_lap_time = base_lap_time * driver_factor;

    // add randomness (±5% variation)
    let mut rng = rand::thread_rng();
    let random_factor = 1.0 + (rng.gen::<f32>() * 2.0 - 1.0) * RANDOMNESS_FACTOR;
    let final_lap_time = adjusted_lap_time * random_factor;

    // convert to seconds for output
    final_lap_time * 3600.0
}
