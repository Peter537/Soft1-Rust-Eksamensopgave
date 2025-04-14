-- Disable foreign key constraints temporarily
PRAGMA foreign_keys = OFF;

-- Drop tables in an order that prevents dependency conflicts
DROP TABLE IF EXISTS laps;
DROP TABLE IF EXISTS race_driver_results;
DROP TABLE IF EXISTS race_results;
DROP TABLE IF EXISTS season_schedules;
DROP TABLE IF EXISTS seasons;
DROP TABLE IF EXISTS driver_contracts;
DROP TABLE IF EXISTS drivers;
DROP TABLE IF EXISTS teams;
DROP TABLE IF EXISTS circuits;
DROP TABLE IF EXISTS config;
DROP TABLE IF EXISTS countries;

-- Recreate the tables in a dependency-safe order

-- Table: countries (parent table for others)
CREATE TABLE countries (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  image_country TEXT
);

-- Table: drivers (depends on countries)
CREATE TABLE drivers (
  id INTEGER PRIMARY KEY,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  rating INTEGER NOT NULL,
  fk_country_id INTEGER NOT NULL,
  date_of_birth TIMESTAMP NOT NULL,
  racing_number INTEGER NOT NULL,
  image_driver TEXT,
  FOREIGN KEY (fk_country_id) REFERENCES countries(id)
);

-- Table: teams (depends on countries)
CREATE TABLE teams (
  id INTEGER PRIMARY KEY,
  short_name TEXT NOT NULL,
  full_name TEXT NOT NULL,
  fk_country_id INTEGER NOT NULL,
  base_city TEXT NOT NULL,
  first_entry INTEGER NOT NULL,
  team_chief TEXT NOT NULL,
  chassis TEXT NOT NULL,
  power_unit TEXT NOT NULL,
  image_team TEXT,
  image_car TEXT,
  FOREIGN KEY (fk_country_id) REFERENCES countries(id)
);

-- Table: driver_contracts (depends on drivers and teams)
CREATE TABLE driver_contracts (
  id INTEGER PRIMARY KEY,
  fk_driver_id INTEGER NOT NULL,
  fk_team_id INTEGER NOT NULL,
  date_begin TIMESTAMP NOT NULL,
  date_end TIMESTAMP NOT NULL,
  monthly_wage DOUBLE NOT NULL,
  FOREIGN KEY (fk_driver_id) REFERENCES drivers(id),
  FOREIGN KEY (fk_team_id) REFERENCES teams(id)
);

-- Table: circuits (depends on countries)
CREATE TABLE circuits (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  fk_country_id INTEGER NOT NULL,
  city TEXT NOT NULL,
  length_km DOUBLE NOT NULL,
  lap_amount INTEGER NOT NULL,
  image_circuit TEXT,
  FOREIGN KEY (fk_country_id) REFERENCES countries(id)
);

-- Table: race_driver_results (depends on season_schedules, drivers, teams)
CREATE TABLE race_driver_results (
  id INTEGER PRIMARY KEY,
  fk_season_schedule_id INTEGER NOT NULL,
  fk_driver_id INTEGER NOT NULL,
  fk_team_id INTEGER NOT NULL,
  placement INTEGER,
  points INTEGER NOT NULL DEFAULT 0,
  status TEXT NOT NULL,  -- Ex: Finished, DNF
  FOREIGN KEY (fk_season_schedule_id) REFERENCES season_schedules(id),
  FOREIGN KEY (fk_driver_id) REFERENCES drivers(id),
  FOREIGN KEY (fk_team_id) REFERENCES teams(id)
);

-- Table: laps (depends on race_driver_results)
CREATE TABLE laps (
  id INTEGER PRIMARY KEY,
  fk_race_driver_result_id INTEGER NOT NULL,
  lap_time_ms DOUBLE NOT NULL,
  lap_number INTEGER NOT NULL,
  FOREIGN KEY (fk_race_driver_result_id) REFERENCES race_driver_results(id)
);

-- Table: seasons
CREATE TABLE seasons (
  id INTEGER PRIMARY KEY,
  year INTEGER NOT NULL
);

-- Table: season_schedules (depends on seasons, circuits, race_results)
CREATE TABLE season_schedules (
  id INTEGER PRIMARY KEY,
  fk_season_id INTEGER NOT NULL,
  fk_circuit_id INTEGER NOT NULL,
  date TIMESTAMP NOT NULL,
  status TEXT NOT NULL,
  grand_prix_name TEXT NOT NULL,
  FOREIGN KEY (fk_season_id) REFERENCES seasons(id),
  FOREIGN KEY (fk_circuit_id) REFERENCES circuits(id)
);

-- Table: config (optionally depends on teams)
CREATE TABLE config (
  id INTEGER PRIMARY KEY,
  starting_year INTEGER NOT NULL,
  current_date INTEGER NOT NULL,
  selected_team INTEGER,
  FOREIGN KEY (selected_team) REFERENCES teams(id)
);

-- Re-enable foreign key constraints
PRAGMA foreign_keys = ON;
