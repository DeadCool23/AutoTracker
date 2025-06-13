ALTER TABLE AppUser
    ADD CONSTRAINT unique_passport UNIQUE (passport_serial, passport_num),
    ADD CONSTRAINT check_role CHECK (role IN ('user', 'operator', 'audit')),
    ADD CONSTRAINT unique_login UNIQUE (login),
    ADD CONSTRAINT check_login_email CHECK (login ~ '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$'),
    ADD CONSTRAINT check_passport_format CHECK (passport_serial BETWEEN 1 AND 9999 AND passport_num BETWEEN 1 AND 999999);

ALTER TABLE Car
    ADD FOREIGN KEY (owner_id) REFERENCES CarOwner(id) ON DELETE CASCADE,
    ADD CONSTRAINT unique_vin UNIQUE (vin),
    ADD CONSTRAINT check_mileage CHECK (mileage >= 0),
    ADD CONSTRAINT check_vin_length CHECK (LENGTH(vin) = 17);

ALTER TABLE CarOwner
    ADD CONSTRAINT unique_owner_passport UNIQUE (passport_serial, passport_num),
    ADD CONSTRAINT unique_drive_license UNIQUE (drive_license_serial, drive_license_num),
    ADD CONSTRAINT check_age CHECK (age >= 18),
    ADD CONSTRAINT check_drive_exp CHECK (drive_exp >= 0),
    ADD CONSTRAINT check_owner_passport_format CHECK (passport_serial BETWEEN 1 AND 9999 AND passport_num BETWEEN 1 AND 999999),
    ADD CONSTRAINT check_drive_license_format CHECK (drive_license_serial BETWEEN 1 AND 9999 AND drive_license_num BETWEEN 1 AND 999999);

ALTER TABLE PTS
    ADD FOREIGN KEY (sts_id) REFERENCES STS(id) ON DELETE CASCADE,
    ADD CONSTRAINT unique_pts UNIQUE (pts_serial, pts_number),
    ADD CONSTRAINT check_pts_format CHECK (pts_serial BETWEEN 1 AND 9999 AND pts_number BETWEEN 1 AND 999999);

ALTER TABLE STS
    ADD FOREIGN KEY (car_id) REFERENCES Car(id) ON DELETE CASCADE,
    ADD FOREIGN KEY (owner_id) REFERENCES CarOwner(id) ON DELETE CASCADE,
    ADD CONSTRAINT unique_sts UNIQUE (sts_serial, sts_num),
    ADD CONSTRAINT unique_gos_num UNIQUE (gos_num),
    ADD CONSTRAINT sts_unique_vin UNIQUE (vin),
    ADD CONSTRAINT check_horse_power CHECK (horse_power > 0),
    ADD CONSTRAINT check_weight CHECK (car_weight > 0),
    ADD CONSTRAINT check_engine_type CHECK (engine_type IN ('petrol', 'diesel', 'electric', 'hybrid')),
    ADD CONSTRAINT check_car_class CHECK (car_class IN ('A', 'B', 'C', 'D', 'E', 'F', 'S', 'M', 'J')),
    ADD CONSTRAINT check_release_date CHECK (release_date <= CURRENT_DATE),
    ADD CONSTRAINT check_reg_date CHECK (reg_date >= release_date),
    ADD CONSTRAINT check_sts_format CHECK (sts_serial BETWEEN 1 AND 9999 AND sts_num BETWEEN 1 AND 999999),
    ADD CONSTRAINT check_gos_num_format CHECK (gos_num ~ '^[АВЕКМНОРСТУХ]\d{3}[АВЕКМНОРСТУХ]{2}\d{2,3}$'),
    ADD CONSTRAINT check_vin_length CHECK (LENGTH(vin) = 17);

ALTER TABLE OwnerHistory
    ADD FOREIGN KEY (pts_id) REFERENCES PTS(id) ON DELETE CASCADE,
    ADD CONSTRAINT check_mileage CHECK (mileage >= 0),
    ADD CONSTRAINT check_dates CHECK (dereg_date IS NULL OR dereg_date >= reg_date);

ALTER TABLE Camera
    ADD CONSTRAINT check_install_date CHECK (install_date <= CURRENT_DATE);

ALTER TABLE CarSnapshot
    ADD FOREIGN KEY (camera_id) REFERENCES Camera(id) ON DELETE CASCADE,
    ADD CONSTRAINT check_speed CHECK (speed >= 0),
    ADD CONSTRAINT check_road_line CHECK (road_line >= 0),
    ADD CONSTRAINT check_gos_num_format CHECK (gos_num ~ '^[АВЕКМНОРСТУХ]\d{3}[АВЕКМНОРСТУХ]{2}\d{2,3}$'),
    ADD CONSTRAINT check_snapshot_date CHECK (snap_datetime <= (NOW() AT TIME ZONE 'Europe/Moscow' + INTERVAL '5 seconds'));

ALTER TABLE TrackInfo
    ADD FOREIGN KEY (user_id) REFERENCES AppUser(id) ON DELETE CASCADE,
    ADD FOREIGN KEY (car_id) REFERENCES Car(id) ON DELETE CASCADE,
    ADD CONSTRAINT check_route_date CHECK (route_date <= track_time::DATE);

ALTER TABLE OwnerHistoryOwner
    ADD FOREIGN KEY (owner_id) REFERENCES CarOwner(id) ON DELETE CASCADE,
    ADD FOREIGN KEY (owner_history_id) REFERENCES OwnerHistory(id) ON DELETE CASCADE;