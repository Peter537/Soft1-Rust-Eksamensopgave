pub struct SeasonInfo {
    pub season_year: i32,
    pub total_points: i32,
    pub overall_position: i32,
    pub races: Vec<RaceInfo>,
}

pub struct RaceInfo {
    pub grand_prix_name: String,
    pub date: String,
    pub team_positions: Vec<i32>,
    pub race_points: i32,
}
