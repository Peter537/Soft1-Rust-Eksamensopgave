pub struct SeasonInfo {
    pub total_points: u16,
    pub overall_position: u8,
    pub races: Vec<RaceInfo>,
}

pub struct RaceInfo {
    pub grand_prix_name: String,
    pub date: String,
    pub team_positions: Vec<u8>,
    pub race_points: u16,
}
