use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::PathBuf;

pub fn create_files_if_not_exist() {
    // Get the AppData/Roaming path
    let appdata = env::var("APPDATA").expect("APPDATA environment variable not found");
    let base_path = PathBuf::from(appdata).join("FormulaOneManager");

    // Create directories and file structure
    // Create Mod/Default directory and create_script.sql
    let mod_default_path = base_path.join("Mod").join("Default");
    fs::create_dir_all(&mod_default_path).expect("Failed to create Mod/Default directory");
    let default_database_path = mod_default_path.join("default_database.db");
    if !default_database_path.exists() {
        fs::copy("sql/default_database.db", default_database_path)
            .expect("Failed to copy default_database.db");
    }

    // Create GameSaves directory
    let game_saves_path = base_path.join("GameSaves");
    fs::create_dir_all(&game_saves_path).expect("Failed to create GameSaves directory");

    // Create Config directory
    let config_path = base_path.join("Config");
    fs::create_dir_all(&config_path).expect("Failed to create Config directory");
}

pub fn get_existing_careers() -> HashSet<u32> {
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
                            if let Ok(num) = num_str.parse::<u32>() {
                                existing.insert(num);
                            }
                        }
                    }
                }
            }
        }
    }
    existing
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

    fs::copy(
        base_path
            .join("Mod")
            .join("Default")
            .join("default_database.db"),
        game_saves_path.join(format!("Career_{}.db", new_career_number)),
    )
    .expect("Failed to copy default_database.db to new career file");
}
