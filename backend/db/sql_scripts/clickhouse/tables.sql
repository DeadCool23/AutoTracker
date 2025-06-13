-- Active: 1747382442971@@127.0.0.1@8123@auto_tracker
CREATE TABLE AppUser (
    id UInt32,
    login String,
    password String,
    role String,
    name String,
    surname String,
    lastname Nullable(String),
    is_verified UInt8,
    passport_serial Nullable(Int32),
    passport_num Nullable(Int32)
) ENGINE = ReplacingMergeTree()
ORDER BY id;

CREATE TABLE Car (
    id UInt32,
    owner_id UInt32,
    mark String,
    model String,
    vin String,
    mileage Int32,
    color String
) ENGINE = MergeTree()
ORDER BY id;

CREATE TABLE TrackInfo (
    id UInt32,
    user_id UInt32,
    track_time DateTime,
    route_date Date,
    car_id UInt32
) ENGINE = MergeTree()
ORDER BY id;

CREATE TABLE CarOwner (
    id UInt32,
    name String,
    surname String,
    lastname Nullable(String),
    age Int32,
    drive_exp Int32,
    passport_serial Int32,
    passport_num Int32,
    drive_license_serial Int32,
    drive_license_num Int32
) ENGINE = MergeTree()
ORDER BY id;

CREATE TABLE PTS (
    id UInt32,
    sts_id UInt32,
    pts_serial Int32,
    pts_number Int32,
    import_country String
) ENGINE = MergeTree()
ORDER BY id;

CREATE TABLE OwnerHistory (
    id UInt32,
    pts_id UInt32,
    mileage Int32,
    reg_date Date,
    dereg_date Date
) ENGINE = MergeTree()
ORDER BY id;

CREATE TABLE OwnerHistoryOwner (
    id UInt32,
    owner_id UInt32,
    owner_history_id UInt32
) ENGINE = MergeTree()
ORDER BY id;

CREATE TABLE STS (
    id UInt32,
    car_id UInt32,
    owner_id UInt32,
    vin String,
    gos_num String,
    mark String,
    model String,
    horse_power Int32,
    car_weight Int32,
    sts_serial Int32,
    sts_num Int32,
    engine_type String,
    car_class String,
    release_date Date,
    reg_date Date
) ENGINE = MergeTree()
ORDER BY id;

CREATE TABLE Camera (
    id UInt32,
    longitude Float64,
    latitude Float64,
    install_date Date,
    is_radar UInt8
) ENGINE = MergeTree()
ORDER BY id;

CREATE TABLE CarSnapshot (
    id UInt32,
    camera_id UInt32,
    snap_datetime DateTime,
    speed Nullable(Int32),
    gos_num String,
    road_line Int32 DEFAULT 0
) ENGINE = MergeTree()
ORDER BY id;
