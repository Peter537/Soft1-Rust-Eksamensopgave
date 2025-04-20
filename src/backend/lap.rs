use rand::Rng;

pub fn generate_lap_time(driver_rating: u32, circuit_length: f64) -> f64 {
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
