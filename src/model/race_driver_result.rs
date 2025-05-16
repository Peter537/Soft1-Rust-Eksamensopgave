pub struct RaceDriverResult {
    pub driver_id: u16,
    pub team_id: u16,
    pub placement: u8,
    pub points: u16,
    pub status: String,
}

pub struct RaceResult {
    pub position: u8,
    pub driver_number: u8,
    pub driver_name: String,
    pub team: String,
    pub points: u16,
    pub total_time_ms: u32,
}
