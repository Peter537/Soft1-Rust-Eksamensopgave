pub struct Circuit {
    pub id: i32,
    pub name: String,
    pub country_id: i32,
    pub city: String,
    pub length_km: f32,
    pub lap_amount: i32,
    pub image_path: String,
}

pub struct CircuitInfo {
    pub circuit_name: String,
    pub location: String,
    pub length_km: f64,
    pub lap_amount: i32,
    pub image_path: Option<String>,
}