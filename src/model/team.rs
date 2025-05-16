pub struct Team {
    pub id: u16,
    pub short_name: String,
    pub full_name: String,
    pub first_entry: u16,
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
