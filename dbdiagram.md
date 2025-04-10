https://dbdiagram.io/d

```dbml
// Use DBML to define your database structure
// Docs: https://dbml.dbdiagram.io/docs

Table drivers {
  id integer [pk]
  first_name varchar [not null]
  last_name varchar [not null, note: "Middle-names will be here"]
  rating integer [not null]
  fk_country_id integer [not null]
  date_of_birth timestamp [not null]
  racing_number integer [not null]
  image_driver varchar
}

Ref: countries.id < drivers.fk_country_id

Table teams {
  id integer [pk]
  name varchar [not null]
  fk_country_id integer [not null]
  base_city varchar [not null]
  first_entry integer [not null]
  team_chief varchar [not null]
  chassis varchar [not null]
  power_unit varchar [not null]
  image_team varchar
  image_var varchar
}

Ref: countries.id < teams.fk_country_id

Table driver_contracts {
  id integer [pk]
  fk_driver_id integer [not null]
  fk_team_id integer [not null]
  date_begin timestamp [not null]
  date_end timestamp [not null]
  monthly_wage double [not null]
}

Ref: drivers.id < driver_contracts.fk_driver_id
Ref: teams.id < driver_contracts.fk_team_id

Table circuits {
  id integer [pk]
  name varchar [not null]
  fk_country_id varchar [not null]
  city varchar [not null]
  length_km double [not null]
  lap_amount integer [not null]
  image_circuit varchar
}

Ref: countries.id < circuits.fk_country_id

Table race_results {
  id integer [pk]
  fk_circuit_id integer [not null]
  race_name varchar [not null]
  date timestamp [not null]
  weather_condition varchar [not null]
}

Ref: circuits.id < race_results.fk_circuit_id

Table race_driver_results {
  id integer [pk]
  fk_race_result_id integer [not null]
  fk_driver_id integer [not null]
  fk_team_id integer [not null]
  placement integer
  points integer [not null, default: 0]
  status varchar [not null, note: "Ex: Finished, DNF"]
}

Ref: race_results.id < race_driver_results.fk_race_result_id
Ref: drivers.id < race_driver_results.fk_driver_id
Ref: teams.id < race_driver_results.fk_team_id

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

Table countries {
  id integer [pk]
  name varchar [not null]
  image_country varchar
}
```
