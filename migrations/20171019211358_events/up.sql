PRAGMA foreign_keys = ON;
CREATE TABLE IF NOT EXISTS events (
  id INTEGER PRIMARY KEY,
  sport TEXT NOT NULL,
  title TEXT NOT NULL,
  country TEXT NOT NULL,
  location TEXT NOT NULL,
  track TEXT NOT NULL
);

/* INSERT INTO events (sport, round, location, country) VALUES ('Formula 1', 1, 'Melbourne', 'Australia'); */
/* INSERT INTO events (sport, round, location, country) VALUES ('Formula 1', 2, 'Shanghai', 'China'); */
/* INSERT INTO events (sport, round, location, country) VALUES ('Formula 1', 3, 'Sakhir', 'Bahrain'); */
/* INSERT INTO events (sport, round, location, country) VALUES ('Formula 1', 4, 'Sochi', 'Russia'); */
/* INSERT INTO events (sport, round, location, country) VALUES ('Formula 1', 5, 'Catalunya', 'Spain'); */
/* INSERT INTO events (sport, round, location, country) VALUES ('Formula 1', 6, 'Monaco', 'Monaco'); */
/* INSERT INTO events (sport, round, location, country) VALUES ('Formula 1', 7, 'Montreal', 'Canada'); */
/* INSERT INTO events (sport, round, location, country) VALUES ('Formula 1', 8, 'Baku', 'Azerbaijan'); */
/* INSERT INTO events (sport, round, location, country) VALUES ('Formula 1', 9, 'Speilberg', 'Austria'); */
/* INSERT INTO events (sport, round, location, country) VALUES ('Formula 1', 10, 'Silverstone', 'England'); */
/* INSERT INTO events (sport, round, location, country) VALUES ('Formula 1', 11, 'Magyar, Nagydij', 'Hungary'); */
/* INSERT INTO events (sport, round, location, country) VALUES ('Formula 1', 12, 'Spa-Francorchamps', 'Belgium'); */
/* INSERT INTO events (sport, round, location, country) VALUES ('Formula 1', 13, 'Monza', 'Italy'); */
/* INSERT INTO events (sport, round, location, country) VALUES ('Formula 1', 14, 'Singapore', 'Singapore'); */
/* INSERT INTO events (sport, round, location, country) VALUES ('Formula 1', 15, 'Kuala Lumpur', 'Malaysia'); */
/* INSERT INTO events (sport, round, location, country) VALUES ('Formula 1', 16, 'Suzuka', 'Japan'); */
/* INSERT INTO events (sport, round, location, country) VALUES ('Formula 1', 17, 'Austin', 'United States'); */
/* INSERT INTO events (sport, round, location, country) VALUES ('Formula 1', 18, 'Mexico City', 'Mexico'); */
/* INSERT INTO events (sport, round, location, country) VALUES ('Formula 1', 19, 'Sao Paulo', 'Brazil'); */
/* INSERT INTO events (sport, round, location, country) VALUES ('Formula 1', 20, 'Yas Marina', 'Abu Dhabi'); */
