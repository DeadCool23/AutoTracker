CREATE OR REPLACE PROCEDURE verify_user(
    u_login TEXT,
    new_passport_serial INTEGER,
    new_passport_num INTEGER
)
LANGUAGE plpgsql
AS $$
BEGIN
    IF u_login IS NULL THEN
        RAISE EXCEPTION 'Login cannot be NULL';
    END IF;
    
    IF new_passport_serial IS NULL THEN
        RAISE EXCEPTION 'Passport serial cannot be NULL';
    END IF;
    
    IF new_passport_num IS NULL THEN
        RAISE EXCEPTION 'Passport number cannot be NULL';
    END IF;

    IF NOT EXISTS (SELECT 1 FROM AppUser WHERE login = u_login) THEN
        RAISE EXCEPTION 'User with login % does not exist', u_login;
    END IF;

    IF EXISTS (SELECT 1 FROM AppUser WHERE passport_serial = new_passport_serial AND passport_num = new_passport_num AND login != u_login) THEN
        RAISE EXCEPTION 'Passport data already exists for another user';
    END IF;

    UPDATE AppUser
    SET is_verified = TRUE,
        passport_serial = new_passport_serial,
        passport_num = new_passport_num
    WHERE login = u_login;
END;
$$;
