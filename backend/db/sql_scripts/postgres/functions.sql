CREATE OR REPLACE FUNCTION get_avg_speed_for_car_at_camera(
    p_gos_num TEXT,
    p_camera_id INTEGER
) 
RETURNS FLOAT AS $$
DECLARE
    avg_speed FLOAT;
BEGIN
    SELECT AVG(speed) INTO avg_speed
    FROM CarSnapshot
    WHERE gos_num = p_gos_num
      AND camera_id = p_camera_id
      AND speed IS NOT NULL;
    
    RETURN COALESCE(avg_speed, 0);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION get_tracks_info(
    p_firstname TEXT DEFAULT NULL,
    p_surname TEXT DEFAULT NULL,
    p_lastname TEXT DEFAULT NULL,
    p_track_date DATE DEFAULT NULL,
    p_gos_num_mask TEXT DEFAULT NULL,
    p_passport_serial INT DEFAULT NULL,
    p_passport_number INT DEFAULT NULL
)
RETURNS TABLE (
    route_date DATE,
    track_time TIMESTAMP,
    name TEXT,
    surname TEXT,
    lastname TEXT,
    login TEXT,
    is_verified BOOLEAN,
    passport_serial INT,
    passport_num INT,
    role TEXT,
    oname TEXT,
    osurname TEXT,
    olastname TEXT,
    gos_num TEXT,
    model TEXT,
    mark TEXT,
    color TEXT,
    release_date DATE,
    vin TEXT,
    sts_serial INT,
    sts_num INT,
    pts_serial INT,
    pts_number INT
) 
LANGUAGE plpgsql AS $$
DECLARE
    query_text TEXT;
    where_clause TEXT := '';
BEGIN
    query_text := '
        SELECT 
            t.route_date, 
            t.track_time, 
            a.name, 
            a.surname, 
            a.lastname, 
            a.login, 
            a.is_verified, 
            a.passport_serial, 
            a.passport_num, 
            a.role,
            o.name as oname, 
            o.surname as osurname, 
            o.lastname as olastname, 
            s.gos_num, 
            s.model, 
            s.mark,
            c.color, 
            s.release_date, 
            s.vin,
            s.sts_serial, 
            s.sts_num, 
            p.pts_serial, 
            p.pts_number
        FROM 
            CarOwner o
            JOIN Car c ON o.id = c.owner_id
            JOIN TrackInfo t ON t.car_id = c.id
            JOIN AppUser a ON a.id = t.user_id
            JOIN STS s ON c.id = s.car_id
            JOIN PTS p ON c.id = p.id';

    IF p_firstname IS NOT NULL THEN
        where_clause := where_clause || ' AND a.name = ' || quote_literal(p_firstname);
    END IF;

    IF p_surname IS NOT NULL THEN
        where_clause := where_clause || ' AND a.surname = ' || quote_literal(p_surname);
    END IF;

    IF p_lastname IS NOT NULL THEN
        where_clause := where_clause || ' AND a.lastname = ' || quote_literal(p_lastname);
    END IF;

    IF p_track_date IS NOT NULL THEN
        where_clause := where_clause || ' AND t.route_date = ' || quote_literal(p_track_date);
    END IF;

    IF p_gos_num_mask IS NOT NULL THEN
        where_clause := where_clause || ' AND s.gos_num LIKE ' || quote_literal(p_gos_num_mask);
    END IF;

    IF p_passport_serial IS NOT NULL THEN
        where_clause := where_clause || ' AND a.passport_serial = ' || quote_literal(p_passport_serial);
    END IF;

    IF p_passport_number IS NOT NULL THEN
        where_clause := where_clause || ' AND a.passport_num = ' || quote_literal(p_passport_number);
    END IF;

    IF where_clause != '' THEN
        query_text := query_text || ' WHERE 1=1' || where_clause;
    END IF;

    RETURN QUERY EXECUTE query_text;
END;
$$;

CREATE OR REPLACE FUNCTION get_cars(
    p_firstname TEXT DEFAULT NULL,
    p_surname TEXT DEFAULT NULL,
    p_lastname TEXT DEFAULT NULL,
    p_gos_num_mask TEXT DEFAULT NULL,
    p_passport_serial INT DEFAULT NULL,
    p_passport_number INT DEFAULT NULL
)
RETURNS TABLE (
    name TEXT,
    surname TEXT,
    lastname TEXT,
    gos_num TEXT,
    model TEXT,
    mark TEXT,
    color TEXT,
    release_date DATE,
    vin TEXT,
    sts_serial INT,
    sts_num INT,
    pts_serial INT,
    pts_number INT
) 
LANGUAGE plpgsql AS $$
DECLARE
    query_text TEXT;
    where_clause TEXT := '';
BEGIN
    query_text := '
        SELECT 
            o.name,
            o.surname,
            o.lastname,
            s.gos_num,
            s.model,
            s.mark, 
            c.color,
            s.release_date,
            s.vin, 
            s.sts_serial,
            s.sts_num,
            p.pts_serial,
            p.pts_number 
        FROM CarOwner o
            JOIN Car c ON o.id = c.owner_id
            JOIN STS s ON c.id = s.car_id
            JOIN PTS p ON c.id = p.id';

    IF p_firstname IS NOT NULL THEN
        where_clause := where_clause || ' AND o.name = ' || quote_literal(p_firstname);
    END IF;

    IF p_surname IS NOT NULL THEN
        where_clause := where_clause || ' AND o.surname = ' || quote_literal(p_surname);
    END IF;

    IF p_lastname IS NOT NULL THEN
        where_clause := where_clause || ' AND o.lastname = ' || quote_literal(p_lastname);
    END IF;

    IF p_gos_num_mask IS NOT NULL THEN
        where_clause := where_clause || ' AND s.gos_num LIKE ' || quote_literal(p_gos_num_mask);
    END IF;

    IF p_passport_serial IS NOT NULL THEN
        where_clause := where_clause || ' AND o.passport_serial = ' || quote_literal(p_passport_serial);
    END IF;

    IF p_passport_number IS NOT NULL THEN
        where_clause := where_clause || ' AND o.passport_num = ' || quote_literal(p_passport_number);
    END IF;

    IF where_clause != '' THEN
        query_text := query_text || ' WHERE 1=1' || where_clause;
    END IF;

    RETURN QUERY EXECUTE query_text;
END;
$$;
