pub struct RaceDriverResult {
    pub id: i32,
    pub season_schedule_id: i32,
    pub driver_id: i32,
    pub team_id: i32,
    pub placement: i32,
    pub points: i32,
    pub status: String,
}

pub struct RaceResult {
    pub position: i32,
    pub driver_number: String,
    pub driver_name: String,
    pub team: String,
    pub fastest_lap_time_ms: i32,
}
