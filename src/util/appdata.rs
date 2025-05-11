use crate::database::set_game_number;
use crate::util::file::download_file;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::PathBuf;

const REPO: &str = "https://raw.githubusercontent.com/Peter537/Soft1-Rust-Eksamensopgave/main";

pub fn create_files_if_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    let appdata = std::env::var("APPDATA").expect("APPDATA environment variable not found");
    let base_path = PathBuf::from(appdata).join("FormulaOneManager");

    let mod_default_path = base_path.join("Mod").join("Default");
    fs::create_dir_all(&mod_default_path)?;

    let default_database_path = mod_default_path.join("database.db");
    if !default_database_path.exists() {
        let url = REPO.to_owned() + "/mod/database.db";
        download_file(url.as_str(), &default_database_path)?;
    }

    let mod_image_paths = [
        (
            "Cars",
            vec![
                "alpine.png",
                "aston_martin.png",
                "ferrari.png",
                "haas.png",
                "kick_sauber.png",
                "mclaren.png",
                "mercedes.png",
                "racing_bulls.png",
                "red_bull_racing.png",
                "williams.png",
            ],
        ),
        (
            "Circuits",
            vec![
                "albert_park.png",
                "bahrain.png",
                "baku.png",
                "barcelona.png",
                "cota.png",
                "gilles_villeneuve.png",
                "hungaroring.png",
                "imola.png",
                "interlagos.png",
                "jeddah.png",
                "las_vegas.png",
                "lusail.png",
                "mexico_city.png",
                "miami.png",
                "monaco.png",
                "monza.png",
                "red_bull_ring.png",
                "shanghai.png",
                "silverstone.png",
                "singapore.png",
                "spa.png",
                "suzuka.png",
                "yas_marina.png",
                "zandvoort.png",
            ],
        ),
        (
            "Countries",
            vec![
                "australia.png",
                "austria.png",
                "azerbaijan.png",
                "bahrain.png",
                "belgium.png",
                "brazil.png",
                "canada.png",
                "china.png",
                "france.png",
                "germany.png",
                "hungary.png",
                "italy.png",
                "japan.png",
                "mexico.png",
                "monaco.png",
                "netherlands.png",
                "new_zealand.png",
                "qatar.png",
                "saudi_arabia.png",
                "singapore.png",
                "spain.png",
                "switzerland.png",
                "thailand.png",
                "uae.png",
                "uk.png",
                "usa.png",
            ],
        ),
        (
            "Drivers",
            vec![
                "albon.png",
                "alonso.png",
                "antonelli.png",
                "bearman.png",
                "bortoleto.png",
                "doohan.png",
                "gasly.png",
                "hadjar.png",
                "hamilton.png",
                "hulkenberg.png",
                "lawson.png",
                "leclerc.png",
                "norris.png",
                "ocon.png",
                "piastri.png",
                "russell.png",
                "sainz.png",
                "stroll.png",
                "tsunoda.png",
                "verstappen.png",
            ],
        ),
        (
            "Teams",
            vec![
                "alpine.png",
                "aston_martin.png",
                "ferrari.png",
                "haas.png",
                "kick_sauber.png",
                "mclaren.png",
                "mercedes.png",
                "racing_bulls.png",
                "red_bull_racing.png",
                "williams.png",
            ],
        ),
    ];

    for (dir, files) in mod_image_paths.iter() {
        let local_path = mod_default_path.join(dir);
        fs::create_dir_all(&local_path)?;
        for file in files {
            let url = format!("{}/mod/{}/{}", REPO, dir, file);
            let dest = local_path.join(file);
            if !dest.exists() {
                download_file(&url, &dest)?;
            }
        }
    }

    let game_saves_path = base_path.join("GameSaves");
    fs::create_dir_all(&game_saves_path)?;

    let config_path = base_path.join("Config");
    fs::create_dir_all(&config_path)?;

    Ok(())
}

pub fn get_existing_careers() -> Vec<i32> {
    // Get the AppData/Roaming path
    let appdata = env::var("APPDATA").expect("APPDATA environment variable not found");
    let base_path = PathBuf::from(appdata).join("FormulaOneManager");
    let game_saves_path = base_path.join("GameSaves");

    let mut existing = HashSet::new();
    if let Ok(entries) = fs::read_dir(game_saves_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                        if file_name.starts_with("Career_") && file_name.ends_with(".db") {
                            let num_str = &file_name[7..file_name.len() - 3];
                            if let Ok(num) = num_str.parse::<i32>() {
                                existing.insert(num);
                            }
                        }
                    }
                }
            }
        }
    }
    let mut sorted = existing.into_iter().collect::<Vec<i32>>();
    sorted.sort();
    sorted
}

pub fn create_new_career() {
    // Get the AppData/Roaming path
    let appdata = env::var("APPDATA").expect("APPDATA environment variable not found");
    let base_path = PathBuf::from(appdata).join("FormulaOneManager");
    let game_saves_path = base_path.join("GameSaves");

    let existing = get_existing_careers();
    let new_career_number = if existing.is_empty() {
        1
    } else {
        // find the lowest number possible that isn't already taken
        let mut new_career_number = 1;
        while existing.contains(&new_career_number) {
            new_career_number += 1;
        }
        new_career_number
    };

    set_game_number(new_career_number as i32); // Set the game number for the new career

    fs::copy(
        base_path.join("Mod").join("Default").join("database.db"),
        game_saves_path.join(format!("Career_{}.db", new_career_number)),
    )
    .expect("Failed to copy default database.db to new career file");
}
