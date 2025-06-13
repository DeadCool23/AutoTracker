COPY Camera FROM '/data/cameras.csv' DELIMITER ',' CSV HEADER;
COPY CarOwner FROM '/data/owners.csv' DELIMITER ',' CSV HEADER;
COPY Car FROM '/data/cars.csv' DELIMITER ',' CSV HEADER;
COPY CarSnapshot FROM '/data/snaps.csv' DELIMITER ',' CSV HEADER;
COPY STS FROM '/data/stss.csv' DELIMITER ',' CSV HEADER;
COPY PTS FROM '/data/ptss.csv' DELIMITER ',' CSV HEADER;
COPY AppUser FROM '/data/users.csv' DELIMITER ',' CSV HEADER;
