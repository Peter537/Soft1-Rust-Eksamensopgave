use crate::database::connection::get_connection;
use crate::model::lap::Lap;
use crate::model::race_driver_result::RaceDriverResult;
use crate::model::season_schedule::SeasonSchedule;

use rusqlite::named_params;

pub fn get_season_schedule_by_id(season_schedule_id: i32) -> Option<SeasonSchedule> {
    let conn = get_connection().unwrap();
    let mut stmt = conn.prepare("SELECT id, fk_season_id, fk_circuit_id, date, status, grand_prix_name FROM season_schedules WHERE id = ?").unwrap();
    let row = stmt.query_row([season_schedule_id], |row| {
        let id = row.get(0)?;
        let season_id = row.get(1)?;
        let circuit_id = row.get(2)?;
        let date = row.get(3)?;
        let status = row.get(4)?;
        let grand_prix_name = row.get(5)?;
        Ok(SeasonSchedule {
            id,
            season_id,
            circuit_id,
            date,
            status,
            grand_prix_name,
        })
    });
    match row {
        Ok(season_schedule) => Some(season_schedule),
        Err(_) => None,
    }
}

pub fn update_race_status(season_schedule_id: i32, status: &str) {
    let conn = get_connection().unwrap();
    let _ = conn.execute(
        "UPDATE season_schedules SET status = :status WHERE id = :id",
        named_params! {
            ":status": status,
            ":id": season_schedule_id,
        },
    );
}

pub fn save_driver_results(
    season_schedule_id: i32,
    driver_results: Vec<(i32, (RaceDriverResult, Vec<Lap>))>,
) {
    // Establish connection and start transaction
    let mut conn = get_connection().unwrap();
    let tx = conn.transaction().unwrap();

    {
        // Prepare statement for race_driver_results
        let mut stmt_race_driver_results = tx
        .prepare(
            "INSERT INTO race_driver_results (
                fk_season_schedule_id,
                fk_driver_id,
                fk_team_id,
                placement,
                points,
                status
            ) VALUES (:fk_season_schedule_id, :fk_driver_id, :fk_team_id, :placement, :points, :status)"
        )
        .unwrap();

        // Prepare statement for laps
        let mut stmt_laps = tx
            .prepare(
                "INSERT INTO laps (
                fk_race_driver_result_id,
                lap_time_ms,
                lap_number
            ) VALUES (:fk_race_driver_result_id, :lap_time_ms, :lap_number)",
            )
            .unwrap();

        // Process each driver
        for (_driver_id, (race_driver_result, laps)) in driver_results {
            // Insert race_driver_results record
            stmt_race_driver_results
                .execute(named_params! {
                    ":fk_season_schedule_id": season_schedule_id,
                    ":fk_driver_id": race_driver_result.driver_id,
                    ":fk_team_id": race_driver_result.team_id,
                    ":placement": race_driver_result.placement,
                    ":points": race_driver_result.points,
                    ":status": race_driver_result.status,
                })
                .unwrap();

            // Get the generated ID
            let race_driver_result_id = tx.last_insert_rowid();

            // Insert all laps for this driver
            for lap in laps {
                stmt_laps
                    .execute(named_params! {
                        ":fk_race_driver_result_id": race_driver_result_id,
                        ":lap_time_ms": lap.lap_time_ms,
                        ":lap_number": lap.lap_number,
                    })
                    .unwrap();
            }
        }
    }

    // Commit the transaction
    tx.commit().unwrap();
}
