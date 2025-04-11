https://dbdiagram.io/d

```dbml
// Use DBML to define your database structure
// Docs: https://dbml.dbdiagram.io/docs

Table countries {
  id              int      [pk]
  name            text     [not null]
  image_country   text
}

Table drivers {
  id              int      [pk]
  first_name      text     [not null]
  last_name       text     [not null]
  rating          int      [not null]
  fk_country_id   int      [not null]
  date_of_birth   timestamp [not null]
  racing_number   int      [not null]
  image_driver    text
}

Ref: countries.id < drivers.fk_country_id

Table teams {
  id              int      [pk]
  short_name      text     [not null]
  full_name       text     [not null]
  fk_country_id   int      [not null]
  base_city       text     [not null]
  first_entry     int      [not null]
  team_chief      text     [not null]
  chassis         text     [not null]
  power_unit      text     [not null]
  image_team      text
  image_car       text
}

Ref: countries.id < teams.fk_country_id

Table driver_contracts {
  id              int      [pk]
  fk_driver_id    int      [not null]
  fk_team_id      int      [not null]
  date_begin      timestamp [not null]
  date_end        timestamp [not null]
  monthly_wage    double   [not null]
}

Ref: drivers.id < driver_contracts.fk_driver_id
Ref: teams.id < driver_contracts.fk_team_id

Table circuits {
  id              int      [pk]
  name            text     [not null]
  fk_country_id   int      [not null]
  city            text     [not null]
  length_km       double   [not null]
  lap_amount      int      [not null]
  image_circuit   text
}

Ref: countries.id < circuits.fk_country_id

Table race_results {
  id                int    [pk]
  race_name         text   [not null]
  weather_condition text   [not null]
}

Table race_driver_results {
  id                int    [pk]
  fk_race_result_id int    [not null]
  fk_driver_id      int    [not null]
  fk_team_id        int    [not null]
  placement         int
  points            int    [default: 0, not null]
  status            text   [not null]
}

Ref: race_results.id < race_driver_results.fk_race_result_id
Ref: drivers.id < race_driver_results.fk_driver_id
Ref: teams.id < race_driver_results.fk_team_id

Table laps {
  id                        int      [pk]
  fk_race_driver_result_id  int      [not null]
  lap_time_ms               double   [not null]
  lap_number                int      [not null]
}

Ref: race_driver_results.id < laps.fk_race_driver_result_id

Table seasons {
  id      int    [pk]
  year    int    [not null]
}

Table season_schedules {
  id                int      [pk]
  fk_season_id      int      [not null]
  fk_circuit_id     int      [not null]
  fk_race_result_id int      // This can be null if there's no associated race result
  date              timestamp [not null]
  status            text     [not null]
  grand_prix_name   text     [not null]
}

Ref: seasons.id < season_schedules.fk_season_id
Ref: circuits.id < season_schedules.fk_circuit_id
Ref: race_results.id < season_schedules.fk_race_result_id

Table config {
  id             int   [pk]
  starting_year  int   [not null]
  current_date   int   [not null]
  selected_team  int   // This can be null if no team is selected
}

Ref: teams.id < config.selected_team
```
