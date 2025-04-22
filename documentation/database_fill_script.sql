-- Enable foreign key constraints to maintain referential integrity
PRAGMA foreign_keys = ON;

-- Populate the countries table with countries involved in the 2025 F1 season
INSERT INTO countries (id, name, image_country) VALUES
(1, 'Australia', 'australia'),
(2, 'Austria', 'austria'),
(3, 'Azerbaijan', 'azerbaijan'),
(4, 'Bahrain', 'bahrain'),
(5, 'Belgium', 'belgium'),
(6, 'Brazil', 'brazil'),
(7, 'Canada', 'canada'),
(8, 'China', 'china'),
(9, 'France', 'france'),
(10, 'Germany', 'germany'),
(11, 'Hungary', 'hungary'),
(12, 'Italy', 'italy'),
(13, 'Japan', 'japan'),
(14, 'Mexico', 'mexico'),
(15, 'Monaco', 'monaco'),
(16, 'Netherlands', 'netherlands'),
(17, 'New Zealand', 'new_zealand'),
(18, 'Qatar', 'qatar'),
(19, 'Saudi Arabia', 'saudi_arabia'),
(20, 'Singapore', 'singapore'),
(21, 'Spain', 'spain'),
(22, 'Switzerland', 'switzerland'),
(23, 'Thailand', 'thailand'),
(24, 'United Arab Emirates', 'uae'),
(25, 'United Kingdom', 'uk'),
(26, 'United States', 'usa');

-- Populate the drivers table with driver details
INSERT INTO drivers (id, first_name, last_name, rating, fk_country_id, date_of_birth, racing_number, image_driver) VALUES
(1, 'Max', 'Verstappen', 99, 16, '1997-09-30', 1, 'verstappen'),
(2, 'Lando', 'Norris', 97, 25, '1999-11-13', 4, 'norris'),
(3, 'Gabriel', 'Bortoleto', 70, 6, '2004-10-14', 5, 'bortoleto'),
(4, 'Isack', 'Hadjar', 71, 9, '2004-09-28', 6, 'hadjar'),
(5, 'Jack', 'Doohan', 70, 1, '2003-01-20', 7, 'doohan'),
(6, 'Pierre', 'Gasly', 72, 9, '1996-02-07', 10, 'gasly'),
(7, 'Andrea Kimi', 'Antonelli', 77, 12, '2006-08-25', 12, 'antonelli'),
(8, 'Fernando', 'Alonso', 73, 21, '1981-07-29', 14, 'alonso'),
(9, 'Charles', 'Leclerc', 82, 15, '1997-10-16', 16, 'leclerc'),
(10, 'Lance', 'Stroll', 74, 7, '1998-10-29', 18, 'stroll'),
(11, 'Yuki', 'Tsunoda', 72, 13, '2000-05-11', 22, 'tsunoda'),
(12, 'Alexander', 'Albon', 75, 23, '1996-03-23', 23, 'albon'),
(13, 'Nico', 'Hulkenberg', 72, 10, '1987-08-19', 27, 'hulkenberg'),
(14, 'Liam', 'Lawson', 70, 17, '2002-02-11', 30, 'lawson'),
(15, 'Esteban', 'Ocon', 76, 9, '1996-09-17', 31, 'ocon'),
(16, 'Lewis', 'Hamilton', 79, 25, '1985-01-07', 44, 'hamilton'),
(17, 'Carlos', 'Sainz Jr.', 72, 21, '1994-09-01', 55, 'sainz'),
(18, 'George', 'Russell', 88, 25, '1998-02-15', 63, 'russell'),
(19, 'Oscar', 'Piastri', 93, 1, '2001-04-06', 81, 'piastri'),
(20, 'Oliver', 'Bearman', 73, 25, '2005-05-08', 87, 'bearman');

-- Populate the teams table with team details
INSERT INTO teams (id, short_name, full_name, first_entry, team_chief, chassis, power_unit, image_team, image_car) VALUES
(1, 'Alpine', 'BWT Alpine Formula One Team', 2021, 'Oliver Oakes', 'A525', 'Renault', 'alpine', 'alpine'),
(2, 'Aston Martin', 'Aston Martin Aramco Formula One Team', 2021, 'Andy Cowell', 'AMR25', 'Mercedes', 'aston_martin', 'aston_martin'),
(3, 'Ferrari', 'Scuderia Ferrari HP', 1950, 'Frédéric Vasseur', 'SF-25', 'Ferrari', 'ferrari', 'ferrari'),
(4, 'Haas', 'MoneyGram Haas F1 Team', 2016, 'Ayao Komatsu', 'VF-25', 'Ferrari', 'haas', 'haas'),
(5, 'McLaren', 'McLaren Formula 1 Team', 1966, 'Andrea Stella', 'MCL39', 'Mercedes', 'mclaren', 'mclaren'),
(6, 'Mercedes', 'Mercedes-AMG PETRONAS Formula One Team', 2010, 'Toto Wolff', 'W16', 'Mercedes', 'mercedes', 'mercedes'),
(7, 'Racing Bulls', 'Visa Cash App Racing Bulls Formula One Team', 2006, 'Laurent Mekies', 'VCARB 02', 'Honda RBPT', 'racing_bulls', 'racing_bulls'),
(8, 'Red Bull Racing', 'Oracle Red Bull Racing', 2005, 'Christian Horner', 'RB21', 'Honda RBPT', 'red_bull_racing', 'red_bull_racing'),
(9, 'Kick Sauber', 'Stake F1 Team Kick Sauber', 1993, 'Jonathan Wheatley', 'C45', 'Ferrari', 'kick_sauber', 'kick_sauber'),
(10, 'Williams', 'Atlassian Williams Racing', 1978, 'James Vowles', 'FW47', 'Mercedes', 'williams', 'williams');

-- Populate the team_bases table with team base details
INSERT INTO team_bases (id, fk_team_id, city, fk_country_id) VALUES
(1, 1, 'Enstone', 25),          -- Alpine base in Enstone, United Kingdom
(2, 2, 'Silverstone', 25),      -- Aston Martin base in Silverstone, United Kingdom
(3, 3, 'Maranello', 12),        -- Ferrari base in Maranello, Italy
(4, 4, 'Kannapolis', 26),       -- Haas base in Kannapolis, United States
(5, 5, 'Woking', 25),           -- McLaren base in Woking, United Kingdom
(6, 6, 'Brackley', 25),         -- Mercedes base in Brackley, United Kingdom
(7, 7, 'Faenza', 12),           -- Racing Bulls base in Faenza, Italy
(8, 8, 'Milton Keynes', 25),    -- Red Bull Racing base in Milton Keynes, United Kingdom
(9, 9, 'Hinwil', 22),           -- Kick Sauber base in Hinwil, Switzerland
(10, 10, 'Grove', 25);          -- Williams base in Grove, United Kingdom

-- Populate the driver_contracts table, including mid-season team switches
INSERT INTO driver_contracts (fk_driver_id, fk_team_id, date_begin, date_end, monthly_wage) VALUES
(1, 8, '2025-01-01', '2025-12-31', 100000.0),   -- Max Verstappen with Red Bull
(2, 5, '2025-01-01', '2025-12-31', 100000.0),   -- Lando Norris with McLaren
(3, 9, '2025-01-01', '2025-12-31', 100000.0),   -- Gabriel Bortoleto with Kick Sauber
(4, 7, '2025-01-01', '2025-12-31', 100000.0),   -- Isack Hadjar with RB
(5, 1, '2025-01-01', '2025-12-31', 100000.0),   -- Jack Doohan with Alpine
(6, 1, '2025-01-01', '2025-12-31', 100000.0),   -- Pierre Gasly with Alpine
(7, 6, '2025-01-01', '2025-12-31', 100000.0),   -- Andrea Kimi Antonelli with Mercedes
(8, 2, '2025-01-01', '2025-12-31', 100000.0),   -- Fernando Alonso with Aston Martin
(9, 3, '2025-01-01', '2025-12-31', 100000.0),   -- Charles Leclerc with Ferrari
(10, 2, '2025-01-01', '2025-12-31', 100000.0),  -- Lance Stroll with Aston Martin
(11, 8, '2025-01-01', '2025-12-31', 100000.0),  -- Yuki Tsunoda with Red Bull
(12, 10, '2025-01-01', '2025-12-31', 100000.0), -- Alexander Albon with Williams
(13, 9, '2025-01-01', '2025-12-31', 100000.0),  -- Nico Hulkenberg with Kick Sauber
(14, 7, '2025-01-01', '2025-12-31', 100000.0),  -- Liam Lawson with RB
(15, 4, '2025-01-01', '2025-12-31', 100000.0),  -- Esteban Ocon with Haas
(16, 3, '2025-01-01', '2025-12-31', 100000.0),  -- Lewis Hamilton with Ferrari
(17, 10, '2025-01-01', '2025-12-31', 100000.0), -- Carlos Sainz Jr. with Williams
(18, 6, '2025-01-01', '2025-12-31', 100000.0),  -- George Russell with Mercedes
(19, 5, '2025-01-01', '2025-12-31', 100000.0),  -- Oscar Piastri with McLaren
(20, 4, '2025-01-01', '2025-12-31', 100000.0);  -- Oliver Bearman with Haas

-- Populate the circuits table with circuit details for the first few races
INSERT INTO circuits (id, name, fk_country_id, city, length_km, lap_amount, image_circuit) VALUES
(1, 'Albert Park Circuit', 1, 'Melbourne', 5.278, 58, 'albert_park'),
(2, 'Shanghai International Circuit', 8, 'Shanghai', 5.451, 56, 'shanghai'),
(3, 'Suzuka International Racing Course', 13, 'Suzuka', 5.807, 53, 'suzuka'),
(4, 'Bahrain International Circuit', 4, 'Sakhir', 5.412, 57, 'bahrain'),
(5, 'Jeddah Corniche Circuit', 19, 'Jeddah', 6.174, 50, 'jeddah'),
(6, 'Miami International Autodrome', 26, 'Miami', 5.412, 57, 'miami'),
(7, 'Autodromo Enzo e Dino Ferrari', 12, 'Imola', 4.909, 63, 'imola'),
(8, 'Circuit de Monaco', 15, 'Monaco', 3.337, 78, 'monaco'),
(9, 'Circuit de Barcelona-Catalunya', 21, 'Barcelona', 4.657, 66, 'barcelona'),
(10, 'Circuit Gilles Villeneuve', 7, 'Montreal', 4.361, 70, 'gilles_villeneuve'),
(11, 'Red Bull Ring', 2, 'Spielberg', 4.318, 71, 'red_bull_ring'),
(12, 'Silverstone Circuit', 25, 'Silverstone', 5.891, 52, 'silverstone'),
(13, 'Circuit de Spa-Francorchamps', 5, 'Stavelot', 7.004, 44, 'spa'),
(14, 'Hungaroring', 11, 'Mogyoród', 4.381, 70, 'hungaroring'),
(15, 'Circuit Zandvoort', 16, 'Zandvoort', 4.259, 72, 'zandvoort'),
(16, 'Autodromo Nazionale di Monza', 12, 'Monza', 5.793, 53, 'monza'),
(17, 'Baku City Circuit', 20, 'Baku', 6.003, 51, 'baku'),
(18, 'Marina Bay Street Circuit', 3, 'Singapore', 4.940, 62, 'singapore'),
(19, 'Circuit of The Americas', 26, 'Austin', 5.513, 56, 'cota'),
(20, 'Autódromo Hermanos Rodríguez', 14, 'Mexico City', 4.304, 71, 'mexico_city'),
(21, 'Autódromo José Carlos Pace', 6, 'São Paulo', 4.309, 71, 'interlagos'),
(22, 'Las Vegas Street Circuit', 26, 'Las Vegas', 6.201, 50, 'las_vegas'),
(23, 'Lusail International Circuit', 18, 'Lusail', 5.419, 57, 'lusail'),
(24, 'Yas Marina Circuit', 24, 'Abu Dhabi', 5.281, 58, 'yas_marina');

-- Populate a season
INSERT INTO seasons (id, year) VALUES
(1, 2025);

-- Populate a season schedule
INSERT INTO season_schedules (fk_season_id, fk_circuit_id, date, status, grand_prix_name) VALUES
(1, 1, '2025-03-16', 'Upcoming', 'Australian Grand Prix'),
(1, 2, '2025-03-23', 'Upcoming', 'Chinese Grand Prix'),
(1, 3, '2025-04-06', 'Upcoming', 'Japanese Grand Prix'),
(1, 4, '2025-04-13', 'Upcoming', 'Bahrain Grand Prix'),
(1, 5, '2025-04-20', 'Upcoming', 'Saudi Arabian Grand Prix'),
(1, 6, '2025-05-04', 'Upcoming', 'Miami Grand Prix'),
(1, 7, '2025-05-18', 'Upcoming', 'Emilia Romagna Grand Prix'),
(1, 8, '2025-05-25', 'Upcoming', 'Monaco Grand Prix'),
(1, 9, '2025-06-01', 'Upcoming', 'Spanish Grand Prix'),
(1, 10, '2025-06-15', 'Upcoming', 'Canadian Grand Prix'),
(1, 11, '2025-06-29', 'Upcoming', 'Austrian Grand Prix'),
(1, 12, '2025-07-06', 'Upcoming', 'British Grand Prix'),
(1, 13, '2025-07-27', 'Upcoming', 'Belgian Grand Prix'),
(1, 14, '2025-08-03', 'Upcoming', 'Hungarian Grand Prix'),
(1, 15, '2025-08-31', 'Upcoming', 'Dutch Grand Prix'),
(1, 16, '2025-09-07', 'Upcoming', 'Italian Grand Prix'),
(1, 17, '2025-09-21', 'Upcoming', 'Azerbaijan Grand Prix'),
(1, 18, '2025-10-05', 'Upcoming', 'Singapore Grand Prix'),
(1, 19, '2025-10-19', 'Upcoming', 'United States Grand Prix'),
(1, 20, '2025-10-26', 'Upcoming', 'Mexico City Grand Prix'),
(1, 21, '2025-11-09', 'Upcoming', 'São Paulo Grand Prix'),
(1, 22, '2025-11-22', 'Upcoming', 'Las Vegas Grand Prix'),
(1, 23, '2025-11-30', 'Upcoming', 'Qatar Grand Prix'),
(1, 24, '2025-12-07', 'Upcoming', 'Abu Dhabi Grand Prix');

-- Populate the config table with season settings
INSERT INTO game_config (starting_year, current_date, selected_team) VALUES
(2025, '2025-01-01', NULL);