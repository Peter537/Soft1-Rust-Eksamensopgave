use crate::database::set_game_number;
use crate::util::file::download_file;
use once_cell::sync::Lazy;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::collections::HashSet;
use std::path::PathBuf;
use std::{env, fs};

const REPO: &str = "https://raw.githubusercontent.com/Peter537/Soft1-Rust-Eksamensopgave/main";

static APPDATA_PATH: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from(env::var("APPDATA").expect("APPDATA not set")));

static BASE_PATH: Lazy<PathBuf> = Lazy::new(|| APPDATA_PATH.join("FormulaOneManager"));

static MOD_DEFAULT_PATH: Lazy<PathBuf> = Lazy::new(|| BASE_PATH.join("Mod").join("Default"));

static GAME_SAVES_PATH: Lazy<PathBuf> = Lazy::new(|| BASE_PATH.join("GameSaves"));

static CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| BASE_PATH.join("Config"));

const CARS: [&str; 10] = [
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
];

const CIRCUITS: [&str; 24] = [
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
];

const COUNTRIES: [&str; 26] = [
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
];

const DRIVERS: [&str; 20] = [
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
];

const TEAMS: [&str; 10] = [
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
];

const MOD_IMAGE_PATHS: [(&'static str, &'static [&str]); 5] = [
    ("Cars", &CARS),
    ("Circuits", &CIRCUITS),
    ("Countries", &COUNTRIES),
    ("Drivers", &DRIVERS),
    ("Teams", &TEAMS),
];

pub fn create_files_if_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mod_default_path = get_mod_default_path();

    fs::create_dir_all(&mod_default_path)?;
    fs::create_dir_all(&*GAME_SAVES_PATH)?;
    fs::create_dir_all(&*CONFIG_PATH)?;

    let mut downloads = Vec::new();

    if !mod_default_path.join("database.db").exists() {
        let url = format!("{}/mod/database.db", REPO);
        downloads.push((url, mod_default_path.join("database.db")));
    }

    for &(dir, files) in MOD_IMAGE_PATHS.iter() {
        let local_path = mod_default_path.join(dir);
        fs::create_dir_all(&local_path)?;
        for file in files {
            let url = format!("{}/mod/{}/{}", REPO, dir, file);
            let dest = local_path.join(file);
            if !dest.exists() {
                downloads.push((url, dest));
            }
        }
    }

    println!("Starting downloads...");
    println!("Total files to download: {}", downloads.len());
    let results: Vec<Result<(), Box<dyn std::error::Error + Send>>> = downloads
        .into_par_iter()
        .map(|(url, dest)| {
            println!(
                "Attempting to download from URL: {} to destination: {:?}",
                url, dest
            );
            let result = download_file(&url, &dest);
            if let Err(ref e) = result {
                println!("Failed to download {}: {}", url, e);
            }
            result
        })
        .collect();

    let errors: Vec<_> = results.into_iter().filter_map(Result::err).collect();
    if !errors.is_empty() {
        println!("Errors occurred during file downloads: {:?}", errors);
        return Err(format!("Failed to download some files: {:?}", errors).into());
    }

    Ok(())
}

pub fn get_existing_careers() -> Vec<u16> {
    let mut existing = HashSet::new();
    if let Ok(entries) = fs::read_dir(&*GAME_SAVES_PATH) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                        if file_name.starts_with("Career_") && file_name.ends_with(".db") {
                            let num_str = &file_name[7..file_name.len() - 3];
                            if let Ok(num) = num_str.parse::<u16>() {
                                existing.insert(num);
                            }
                        }
                    }
                }
            }
        }
    }

    let mut sorted = existing.into_iter().collect::<Vec<u16>>();
    sorted.sort();
    sorted
}

pub fn create_new_career() {
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

    set_game_number(new_career_number as u16);

    fs::copy(
        &*BASE_PATH.join("Mod").join("Default").join("database.db"),
        &*GAME_SAVES_PATH.join(format!("Career_{}.db", new_career_number)),
    )
    .expect("Failed to copy default database.db to new career file");
}

pub fn get_mod_default_path() -> &'static PathBuf {
    &*MOD_DEFAULT_PATH
}
