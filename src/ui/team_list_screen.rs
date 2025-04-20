use druid::widget::{Button, Flex, Label, Scroll, SizedBox};
use druid::Widget;

use crate::database::connection::get_connection;
use crate::ui::component::table::make_table;

use super::AppState;
use super::Screen::Main;

use crate::ui::component::goto::{goto_driver, goto_team};

pub fn build_screen() -> impl Widget<AppState> {
    let all_teams = get_team_data();

    let col = vec![
        "Short Name".to_string(),
        "Team Name".to_string(),
        "Points".to_string(),
        "Driver 1".to_string(),
        "Driver 2".to_string(),
        "Country".to_string(),
    ];

    let driver_table = make_table(col, all_teams, vec![(0, goto_team()), (3, goto_driver()), (4, goto_driver())]);

    Flex::column()
        .with_spacer(20.0)
        .with_child(Label::new("Team List Screen"))
        .with_child(
            Button::new("Back to Main").on_click(|_ctx, data: &mut AppState, _env| {
                data.current_screen = Main;
                _ctx.request_update();
            }),
        )
        .with_spacer(20.0)
        .with_child(SizedBox::new(Scroll::new(driver_table).vertical()).height(400.0))
}

// should this be moved to the database module?
pub fn get_team_data() -> Vec<Vec<String>> {
    let conn = get_connection().unwrap();

    let mut stmt = conn
        .prepare(
            r#"
        SELECT 
            t.short_name,
            t.full_name,
            COALESCE((
                SELECT SUM(rdr.points)
                FROM race_driver_results rdr
                WHERE rdr.fk_team_id = t.id
            ), 0) AS total_points,
            (
                SELECT GROUP_CONCAT(d2.first_name || ' ' || d2.last_name, ',')
                FROM driver_contracts dc2
                JOIN drivers d2 ON dc2.fk_driver_id = d2.id
                WHERE dc2.fk_team_id = t.id
                AND (dc2.date_end IS NULL OR dc2.date_end > strftime('%s', 'now') * 1000)
                ORDER BY d2.last_name
            ) AS drivers,
            c.name AS country
        FROM teams t
        JOIN countries c ON t.fk_country_id = c.id
        GROUP BY t.id, t.full_name, c.name
        ORDER BY total_points DESC
        "#,
        )
        .unwrap();

    let team_iter = stmt
        .query_map([], |row| {
            let short_name: String = row.get(0)?;
            let team_name: String = row.get(1)?;
            let points: i32 = row.get(2)?;
            let drivers: Option<String> = row.get(3)?; // GROUP_CONCAT may return NULL
            let country: String = row.get(4)?;

            let driver1 = drivers
                .as_ref()
                .and_then(|d| d.split(',').next())
                .unwrap_or("");

            let driver2 = drivers
                .as_ref()
                .and_then(|d| d.split(',').nth(1))
                .unwrap_or("");
            
            Ok(vec![
                short_name,
                team_name,
                points.to_string(),
                driver1.to_string(),
                driver2.to_string(),
                country,
            ])
        })
        .unwrap();

    let mut teams: Vec<Vec<String>> = Vec::new();
    for team in team_iter {
        teams.push(team.unwrap());
    }

    println!("Team data: {:?}", teams); // Debug print

    teams
}
