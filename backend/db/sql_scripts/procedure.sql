CREATE OR REPLACE PROCEDURE verify_user(
    u_id INTEGER,
    new_passport_serial INTEGER,
    new_passport_num INTEGER
)
LANGUAGE plpgsql
AS $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM AppUser WHERE id = u_id) THEN
        RAISE EXCEPTION 'User with id % does not exist', u_id;
    END IF;

    IF EXISTS (SELECT 1 FROM AppUser WHERE passport_serial = new_passport_serial AND passport_num = new_passport_num AND id != u_id) THEN
        RAISE EXCEPTION 'Passport data already exists for another user';
    END IF;

    UPDATE AppUser
    SET is_verified = TRUE,
        passport_serial = new_passport_serial,
        passport_num = new_passport_num
    WHERE id = u_id;

    COMMIT;
END;
$$;