use druid::widget::{Button, Flex, Label, Scroll, SizedBox};
use druid::Widget;

use crate::database::connection::get_connection;
use crate::ui::component::table::make_table;

use super::AppState;
use super::Screen::Main;

pub fn build_screen() -> impl Widget<AppState> {
    
    let all_drivers = get_driver_data();

    let col: Vec<String> = vec![
        "Name".to_string(),
        "Racing Number".to_string(),
        "Rating".to_string(),
        "Country".to_string(),
        "Team".to_string(),
    ];

    let data: Vec<Vec<String>> = all_drivers;
    
    let driver_table = make_table(col, data, vec![]);
    
    Flex::column()
        .with_spacer(20.0)
        .with_child(Label::new("Driver List Screen"))
        .with_child(
            Button::new("Back to Main").on_click(|_ctx, data: &mut AppState, _env| {
                data.current_screen = Main;
                _ctx.request_update();
            }),
        )
        .with_spacer(20.0)
        .with_child(SizedBox::new(Scroll::new(driver_table).vertical())
            .height(500.0), // set to desired scrollable height
)
}

// Should be moved to the database module?
fn get_driver_data() -> Vec<Vec<String>> {
    let conn = get_connection().unwrap();

    let mut stmt = conn
        .prepare(r#"
        SELECT 
            d.first_name || ' ' || d.last_name AS driver_name,
            d.racing_number,
            d.rating,
            c.name AS country,
            t.full_name AS team
        FROM drivers d
        JOIN countries c ON d.fk_country_id = c.id
        LEFT JOIN driver_contracts dc ON dc.fk_driver_id = d.id
        LEFT JOIN teams t ON dc.fk_team_id = t.id
        WHERE dc.date_end IS NULL OR dc.date_end > strftime('%s', 'now') * 1000
        "#)
        .unwrap();

    let driver_iter = stmt
        .query_map([], |row| {
            let driver_name: String = row.get(0)?;
            let racing_number: i32 = row.get(1)?; // INTEGER in schema
            let rating: i32 = row.get(2)?;        // INTEGER in schema
            let country: String = row.get(3)?;
            let team: Option<String> = row.get(4)?; // Handle NULL teams

            Ok(vec![
                driver_name,
                racing_number.to_string(), // Convert i32 to String
                rating.to_string(),        // Convert i32 to String
                country,
                team.unwrap_or_default(),  // Use empty string for NULL
            ])
        })
        .unwrap();

    let mut data: Vec<Vec<String>> = Vec::new();
    for driver in driver_iter {
        match driver {
            Ok(driver_data) => data.push(driver_data),
            Err(_) => continue, // Skip rows with errors
        }
    }

    println!("Driver data: {:?}", data); // Debug print
    
    data
}
