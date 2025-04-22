pub struct Team {
    pub id: i32,
    pub short_name: String,
    pub full_name: String,
    pub first_entry: i32,
    pub team_chief: String,
    pub chassis: String,
    pub power_unit: String,
    pub image_path_logo: String,
    pub image_path_car: String,
}

pub struct TeamBase {
    pub city: String,
    pub country_name: String,
}
