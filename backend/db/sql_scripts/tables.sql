CREATE TABLE AppUser (
    id SERIAL PRIMARY KEY,
    login TEXT NOT NULL,
    password TEXT NOT NULL,
    role TEXT NOT NULL,
    name TEXT NOT NULL,
    surname TEXT NOT NULL,
    lastname TEXT NOT NULL,
    is_verified BOOLEAN DEFAULT FALSE,
    passport_serial INTEGER,
    passport_num INTEGER
);

CREATE TABLE Car (
    id SERIAL PRIMARY KEY,
    mark TEXT NOT NULL,
    model TEXT NOT NULL,
    vin TEXT NOT NULL,
    mileage INTEGER NOT NULL,
    color TEXT NOT NULL
);

CREATE TABLE TrackInfo (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    track_date DATE NOT NULL,
    route_date DATE NOT NULL,
    car_id INTEGER NOT NULL
);

CREATE TABLE CarOwner (
    id SERIAL PRIMARY KEY,
    car_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    surname TEXT NOT NULL,
    age INTEGER NOT NULL,
    drive_exp INTEGER NOT NULL,
    passport_serial INTEGER NOT NULL,
    passport_num INTEGER NOT NULL,
    drive_license_serial INTEGER NOT NULL,
    drive_license_num INTEGER NOT NULL
);

CREATE TABLE PTS (
    id SERIAL PRIMARY KEY,
    serial INTEGER NOT NULL,
    number INTEGER NOT NULL,
    import_country TEXT NOT NULL
);

CREATE TABLE OwnerHistory (
    id SERIAL PRIMARY KEY,
    pts_id INTEGER NOT NULL,
    mileage INTEGER NOT NULL,
    reg_date DATE NOT NULL,
    dereg_date DATE,
);

CREATE TABLE OwnerHistoryOwner (
    id SERIAL PRIMARY KEY,
    owner_id INTEGER NOT NULL,
    owner_history_id INTEGER NOT NULL,
);

CREATE TABLE STS (
    id SERIAL PRIMARY KEY,
    car_id INTEGER NOT NULL,
    owner_id INTEGER NOT NULL,
    vin TEXT NOT NULL,
    gos_num TEXT NOT NULL,
    mark TEXT NOT NULL,
    model TEXT NOT NULL,
    horse_power INTEGER,
    weight INTEGER,
    serial INTEGER NOT NULL,
    num INTEGER NOT NULL,
    engine_type TEXT NOT NULL,
    car_class TEXT NOT NULL,
    release_date DATE NOT NULL,
    reg_date DATE NOT NULL,
);

CREATE TABLE Camera (
    id SERIAL PRIMARY KEY,
    longitude DOUBLE PRECISION NOT NULL,
    latitude DOUBLE PRECISION NOT NULL,
    install_date DATE NOT NULL,
    is_radar BOOLEAN DEFAULT FALSE
);

CREATE TABLE CarSnapshot (
    id SERIAL PRIMARY KEY,
    camera_id INTEGER NOT NULL,
    snap_datetime DATETIME NOT NULL,
    speed INTEGER,
    gos_num TEXT NOT NULL,
    road_line INTEGER
);