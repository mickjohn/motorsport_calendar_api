PRAGMA foreign_keys = ON;
CREATE TABLE IF NOT EXISTS sessions (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  date DATETIME,
  time DATETIME,
  event_id INTEGER NOT NULL,
  FOREIGN KEY(event_id) REFERENCES events(id)
);


INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 1', 1, '2017-03-24 00:00:00', '2017-03-24 01:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 2', 1, '2017-03-24 00:00:00', '2017-03-24 05:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 3', 1, '2017-03-25 00:00:00', '2017-03-25 03:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Qualifying', 1, '2017-03-25 00:00:00', '2017-03-25 06:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Race', 1, '2017-03-26 00:00:00', '2017-03-26 05:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 1', 2, '2017-04-07 00:00:00', '2017-04-07 02:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 2', 2, '2017-04-07 00:00:00', '2017-04-07 06:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 3', 2, '2017-04-08 00:00:00', '2017-04-08 04:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Qualifying', 2, '2017-04-08 00:00:00', '2017-04-08 07:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Race', 2, '2017-04-09 00:00:00', '2017-04-09 06:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 1', 3, '2017-04-14 00:00:00', '2017-04-14 11:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 2', 3, '2017-04-14 00:00:00', '2017-04-14 15:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 3', 3, '2017-04-15 00:00:00', '2017-04-15 12:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Qualifying', 3, '2017-04-15 00:00:00', '2017-04-15 15:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Race', 3, '2017-04-16 00:00:00', '2017-04-16 15:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 1', 4, '2017-04-28 00:00:00', '2017-04-28 08:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 2', 4, '2017-04-28 00:00:00', '2017-04-28 12:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 3', 4, '2017-04-29 00:00:00', '2017-04-29 09:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Qualifying', 4, '2017-04-29 00:00:00', '2017-04-29 12:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Race', 4, '2017-04-30 00:00:00', '2017-04-30 12:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 1', 5, '2017-05-12 00:00:00', '2017-05-12 08:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 2', 5, '2017-05-12 00:00:00', '2017-05-12 12:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 3', 5, '2017-05-13 00:00:00', '2017-05-13 09:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Qualifying', 5, '2017-05-13 00:00:00', '2017-05-13 12:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Race', 5, '2017-05-14 00:00:00', '2017-05-14 12:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 1', 6, '2017-05-25 00:00:00', '2017-05-25 08:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 2', 6, '2017-05-25 00:00:00', '2017-05-25 12:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 3', 6, '2017-05-27 00:00:00', '2017-05-27 09:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Qualifying', 6, '2017-05-27 00:00:00', '2017-05-27 12:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Race', 6, '2017-05-28 00:00:00', '2017-05-28 12:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 1', 7, '2017-06-09 00:00:00', '2017-06-09 14:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 2', 7, '2017-06-09 00:00:00', '2017-06-09 18:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 3', 7, '2017-06-10 00:00:00', '2017-06-10 14:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Qualifying', 7, '2017-06-10 00:00:00', '2017-06-10 17:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Race', 7, '2017-06-11 00:00:00', '2017-06-11 16:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 1', 8, '2017-06-23 00:00:00', '2017-06-23 09:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 2', 8, '2017-06-23 00:00:00', '2017-06-23 13:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 3', 8, '2017-06-24 00:00:00', '2017-06-24 10:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Qualifying', 8, '2017-06-24 00:00:00', '2017-06-24 13:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Race', 8, '2017-06-25 00:00:00', '2017-06-25 13:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 1', 9, '2017-07-07 00:00:00', '2017-07-07 08:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 2', 9, '2017-07-07 00:00:00', '2017-07-07 12:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 3', 9, '2017-07-08 00:00:00', '2017-07-08 09:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Qualifying', 9, '2017-07-08 00:00:00', '2017-07-08 12:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Race', 9, '2017-07-09 00:00:00', '2017-07-09 12:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 1', 10, '2017-07-14 00:00:00', '2017-07-14 08:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 2', 10, '2017-07-14 00:00:00', '2017-07-14 12:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 3', 10, '2017-07-15 00:00:00', '2017-07-15 09:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Qualifying', 10, '2017-07-15 00:00:00', '2017-07-15 12:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Race', 10, '2017-07-16 00:00:00', '2017-07-16 12:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 1', 11, '2017-07-28 00:00:00', '2017-07-28 08:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 2', 11, '2017-07-28 00:00:00', '2017-07-28 12:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 3', 11, '2017-07-29 00:00:00', '2017-07-29 09:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Qualifying', 11, '2017-07-29 00:00:00', '2017-07-29 12:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Race', 11, '2017-07-30 00:00:00', '2017-07-30 12:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 1', 12, '2017-08-25 00:00:00', '2017-08-25 08:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 2', 12, '2017-08-25 00:00:00', '2017-08-25 12:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 3', 12, '2017-08-26 00:00:00', '2017-08-26 09:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Qualifying', 12, '2017-08-26 00:00:00', '2017-08-26 12:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Race', 12, '2017-08-27 00:00:00', '2017-08-27 12:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 1', 13, '2017-09-01 00:00:00', '2017-09-01 08:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 2', 13, '2017-09-01 00:00:00', '2017-09-01 12:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 3', 13, '2017-09-02 00:00:00', '2017-09-02 09:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Qualifying', 13, '2017-09-02 00:00:00', '2017-09-02 12:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Race', 13, '2017-09-03 00:00:00', '2017-09-03 12:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 1', 14, '2017-09-15 00:00:00', '2017-09-15 10:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 2', 14, '2017-09-15 00:00:00', '2017-09-15 13:30:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 3', 14, '2017-09-16 00:00:00', '2017-09-16 10:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Qualifying', 14, '2017-09-16 00:00:00', '2017-09-16 13:00:00');
INSERT INTO sessions (name, event_id, date, time) VALUES ('Race', 14, '2017-09-17 00:00:00', '2017-09-17 12:00:00');
/* INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 1', 15, '2017-09-29 00:00:00', '2017-09-29 03:00:00'); */
/* INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 2', 15, '2017-09-29 00:00:00', '2017-09-29 07:00:00'); */
/* INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 3', 15, '2017-09-30 00:00:00', '2017-09-30 06:00:00'); */
/* INSERT INTO sessions (name, event_id, date, time) VALUES ('Qualifying', 15, '2017-09-30 00:00:00', '2017-09-30 09:00:00'); */
/* INSERT INTO sessions (name, event_id, date, time) VALUES ('Race', 15, '2017-10-01 00:00:00', '2017-10-01 07:00:00'); */
/* INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 1', 16, '2017-10-26 00:00:00', '2017-10-26 01:00:00'); */
/* INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 2', 16, '2017-10-26 00:00:00', '2017-10-26 05:00:00'); */
/* INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 3', 16, '2017-10-07 00:00:00', '2017-10-07 03:00:00'); */
/* INSERT INTO sessions (name, event_id, date, time) VALUES ('Qualifying', 16, '2017-10-07 00:00:00', '2017-10-07 06:00:00'); */
/* INSERT INTO sessions (name, event_id, date, time) VALUES ('Race', 16, '2017-10-08 00:00:00', '2017-10-08 05:00:00'); */
/* INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 1', 17, '2017-10-20 00:00:00', '2017-10-20 15:00:00'); */
/* INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 2', 17, '2017-10-20 00:00:00', '2017-10-20 19:00:00'); */
/* INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 3', 17, '2017-10-21 00:00:00', '2017-10-21 16:00:00'); */
/* INSERT INTO sessions (name, event_id, date, time) VALUES ('Qualifying', 17, '2017-10-21 00:00:00', '2017-10-21 19:00:00'); */
/* INSERT INTO sessions (name, event_id, date, time) VALUES ('Race', 17, '2017-10-22 00:00:00', '2017-10-22 19:00:00'); */
/* INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 1', 18, '2017-10-27 00:00:00', '2017-10-27 15:00:00'); */
/* INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 2', 18, '2017-10-27 00:00:00', '2017-10-27 19:00:00'); */
/* INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 3', 18, '2017-10-28 00:00:00', '2017-10-28 15:00:00'); */
/* INSERT INTO sessions (name, event_id, date, time) VALUES ('Qualifying', 18, '2017-10-28 00:00:00', '2017-10-28 18:00:00'); */
/* INSERT INTO sessions (name, event_id, date, time) VALUES ('Race', 18, '2017-10-29 00:00:00', '2017-10-29 19:00:00'); */
/* INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 1', 19, '2017-11-10 00:00:00', '2017-11-10 13:00:00'); */
/* INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 2', 19, '2017-11-10 00:00:00', '2017-11-10 17:00:00'); */
/* INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 3', 19, '2017-11-11 00:00:00', '2017-11-11 14:00:00'); */
/* INSERT INTO sessions (name, event_id, date, time) VALUES ('Qualifying', 19, '2017-11-11 00:00:00', '2017-11-11 17:00:00'); */
/* INSERT INTO sessions (name, event_id, date, time) VALUES ('Race', 19, '2017-11-12 00:00:00', '2017-11-12 17:00:00'); */
/* INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 1', 20, '2017-11-24 00:00:00', null); */
/* INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 2', 20, '2017-11-24 00:00:00', null); */
/* INSERT INTO sessions (name, event_id, date, time) VALUES ('Free Practice 3', 20, '2017-11-25 00:00:00', null); */
/* INSERT INTO sessions (name, event_id, date, time) VALUES ('Qualifying', 20, '2017-11-25 00:00:00', null); */
/* INSERT INTO sessions (name, event_id, date, time) VALUES ('Race', 20, '2017-11-26 00:00:00', null); */
