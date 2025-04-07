https://dbdiagram.io/d

```dbml
// Use DBML to define your database structure
// Docs: https://dbml.dbdiagram.io/docs

Table drivers {
  id integer [pk]
  first_name varchar [not null]
  last_name varchar [not null, note: "Middle-names will be here"]
  rating integer [not null]
}

Table teams {
  id integer [pk]
  name varchar [not null]
}

Table driver_contracts {
  id integer [pk]
  fk_driver_id integer [not null]
  fk_team_id integer [not null]
  date_begin timestamp [not null]
  date_end timestamp [not null]
}

Ref: drivers.id < driver_contracts.fk_driver_id

Ref: teams.id < driver_contracts.fk_team_id

Table circuits {
  id integer [pk]
  name varchar [not null]
  country varchar [not null]
  length_km double [not null]
  lap_amount integer [not null]
}

Table race_results {
  id integer [pk]
  fk_circuit_id integer [not null]
  year integer [not null]
  date timestamp [not null]
}

Ref: circuits.id < race_results.fk_circuit_id

Table race_driver_results {
  id integer [pk]
  fk_race_result_id integer [not null]
  fk_driver_id integer [not null]
  placement integer [not null]
  points integer [not null]
}

Ref: race_results.id < race_driver_results.fk_race_result_id
Ref: drivers.id < race_driver_results.fk_driver_id

Table laps {
  id integer [pk]
  fk_race_driver_result_id integer [not null]
  lap_time_ms double [not null]
  lap_number integer [not null]
}

Ref: race_driver_results.id < laps.fk_race_driver_result_id

Table config {
  id integer [pk]
  starting_year integer [not null]
  current_date integer [not null]
  selected_team integer
}

Ref: teams.id < config.selected_team
```
