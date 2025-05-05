pub struct RaceDriverResult {
    pub driver_id: i32,
    pub team_id: i32,
    pub placement: i32,
    pub points: i32,
    pub status: String,
}

pub struct RaceResult {
    pub position: i32,
    pub driver_number: i32,
    pub driver_name: String,
    pub team: String,
    pub points: i32,
    pub total_time_ms: i64,
}
