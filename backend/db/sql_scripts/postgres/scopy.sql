\copy Camera FROM '/data/cameras.csv' DELIMITER ',' CSV HEADER;
\copy CarOwner FROM '/data/owners.csv' DELIMITER ',' CSV HEADER;
\copy Car FROM '/data/cars.csv' DELIMITER ',' CSV HEADER;
\copy CarSnapshot FROM '/data/snaps.csv' DELIMITER ',' CSV HEADER;
\copy STS FROM '/data/stss.csv' DELIMITER ',' CSV HEADER;
\copy PTS FROM '/data/ptss.csv' DELIMITER ',' CSV HEADER;
\copy AppUser FROM '/data/users.csv' DELIMITER ',' CSV HEADER;

SELECT setval('carsnapshot_id_seq', (SELECT COALESCE(MAX(id), 0) + 1 FROM carsnapshot));
SELECT setval('appuser_id_seq', (SELECT COALESCE(MAX(id), 0) + 1 FROM appuser));