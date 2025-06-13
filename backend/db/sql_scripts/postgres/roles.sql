-- ========= СОЗДАНИЕ РОЛЕЙ =========

ALTER TABLE Car ENABLE ROW LEVEL SECURITY;
ALTER TABLE CarOwner ENABLE ROW LEVEL SECURITY;
ALTER TABLE STS ENABLE ROW LEVEL SECURITY;
ALTER TABLE PTS ENABLE ROW LEVEL SECURITY;
ALTER TABLE OwnerHistory ENABLE ROW LEVEL SECURITY;
ALTER TABLE OwnerHistoryOwner ENABLE ROW LEVEL SECURITY;

-- Роль ___Аудит___

CREATE ROLE audit_role WITH
    NOSUPERUSER
    NOCREATEDB
    NOCREATEROLE
    NOINHERIT
    NOREPLICATION
    NOBYPASSRLS
    CONNECTION LIMIT -1
    LOGIN
    PASSWORD 'Aud1t$ecure';

GRANT SELECT ON 
    AppUser,
    Car,
    TrackInfo,
    CarOwner,
    PTS,
    OwnerHistory,
    OwnerHistoryOwner,
    STS
TO audit_role;

CREATE POLICY audit_car_access ON Car
FOR SELECT TO audit_role
USING (true);

CREATE POLICY audit_carowner_access ON CarOwner
FOR SELECT TO audit_role
USING (true);

CREATE POLICY audit_sts_access ON STS
FOR SELECT TO audit_role
USING (true);

CREATE POLICY audit_pts_access ON PTS
FOR SELECT TO audit_role
USING (true);

CREATE POLICY audit_ownerhistory_access ON OwnerHistory
FOR SELECT TO audit_role
USING (true);

CREATE POLICY audit_ownerhistoryowner_access ON OwnerHistoryOwner
FOR SELECT TO audit_role
USING (true);


-- Роль ___Оператор___

CREATE ROLE operator_role WITH
    NOSUPERUSER
    NOCREATEDB
    NOCREATEROLE
    NOINHERIT
    NOREPLICATION
    NOBYPASSRLS
    CONNECTION LIMIT -1
    LOGIN
    PASSWORD 'Oper@tor123';

GRANT SELECT ON 
    AppUser,
    Car,
    CarSnapshot,
    Camera,
    CarOwner,
    PTS,
    OwnerHistory,
    OwnerHistoryOwner,
    STS
TO operator_role;

CREATE POLICY operator_car_access ON Car
FOR SELECT TO operator_role
USING (true);

CREATE POLICY operator_carowner_access ON CarOwner
FOR SELECT TO operator_role
USING (true);

CREATE POLICY operator_sts_access ON STS
FOR SELECT TO operator_role
USING (true);

CREATE POLICY operator_pts_access ON PTS
FOR SELECT TO operator_role
USING (true);

CREATE POLICY operator_ownerhistory_access ON OwnerHistory
FOR SELECT TO operator_role
USING (true);

CREATE POLICY operator_ownerhistoryowner_access ON OwnerHistoryOwner
FOR SELECT TO operator_role
USING (true);


-- Роль ___Пользователь___

CREATE ROLE user_role WITH
    NOSUPERUSER
    NOCREATEDB
    NOCREATEROLE
    NOINHERIT
    NOREPLICATION
    NOBYPASSRLS
    CONNECTION LIMIT -1
    LOGIN
    PASSWORD 'Us3rP@ssword';

GRANT SELECT ON 
    Car,
    PTS,
    OwnerHistory,
    OwnerHistoryOwner,
    STS,
    CarOwner
TO user_role;

CREATE POLICY user_carowner_access ON CarOwner
FOR SELECT TO user_role
USING (
    passport_serial = current_setting('app.passport_serial')::INTEGER AND
    passport_num = current_setting('app.passport_num')::INTEGER
);

CREATE POLICY user_car_access ON Car
FOR SELECT TO user_role
USING (
    id IN (
        SELECT car_id FROM CarOwner
        WHERE passport_serial = current_setting('app.passport_serial')::INTEGER
          AND passport_num = current_setting('app.passport_num')::INTEGER
    )
);

CREATE POLICY user_sts_access ON STS
FOR SELECT TO user_role
USING (
    car_id IN (
        SELECT id FROM Car
        WHERE id IN (
            SELECT car_id FROM CarOwner
            WHERE passport_serial = current_setting('app.passport_serial')::INTEGER
              AND passport_num = current_setting('app.passport_num')::INTEGER
        )
    )
);

CREATE POLICY user_pts_access ON PTS
FOR SELECT TO user_role
USING (
    sts_id IN (
        SELECT id FROM STS
        WHERE car_id IN (
            SELECT car_id FROM CarOwner
            WHERE passport_serial = current_setting('app.passport_serial')::INTEGER
              AND passport_num = current_setting('app.passport_num')::INTEGER
        )
    )
);

CREATE POLICY user_ownerhistory_access ON OwnerHistory
FOR SELECT TO user_role
USING (
    pts_id IN (
        SELECT id FROM PTS
        WHERE sts_id IN (
            SELECT id FROM STS
            WHERE car_id IN (
                SELECT car_id FROM CarOwner
                WHERE passport_serial = current_setting('app.passport_serial')::INTEGER
                  AND passport_num = current_setting('app.passport_num')::INTEGER
            )
        )
    )
);

CREATE POLICY user_ownerhistoryowner_access ON OwnerHistoryOwner
FOR SELECT TO user_role
USING (
    owner_id IN (
        SELECT id FROM CarOwner
        WHERE passport_serial = current_setting('app.passport_serial')::INTEGER
          AND passport_num = current_setting('app.passport_num')::INTEGER
    )
);

-- ========= УДАЛЕНИЕ РОЛЕЙ =========

DROP POLICY IF EXISTS audit_car_access ON Car;
DROP POLICY IF EXISTS audit_carowner_access ON CarOwner;
DROP POLICY IF EXISTS audit_sts_access ON STS;
DROP POLICY IF EXISTS audit_pts_access ON PTS;
DROP POLICY IF EXISTS audit_ownerhistory_access ON OwnerHistory;
DROP POLICY IF EXISTS audit_ownerhistoryowner_access ON OwnerHistoryOwner;


DROP POLICY IF EXISTS operator_car_access ON Car;
DROP POLICY IF EXISTS operator_carowner_access ON CarOwner;
DROP POLICY IF EXISTS operator_sts_access ON STS;
DROP POLICY IF EXISTS operator_pts_access ON PTS;
DROP POLICY IF EXISTS operator_ownerhistory_access ON OwnerHistory;
DROP POLICY IF EXISTS operator_ownerhistoryowner_access ON OwnerHistoryOwner;


DROP POLICY IF EXISTS user_carowner_access ON CarOwner;
DROP POLICY IF EXISTS user_car_access ON Car;
DROP POLICY IF EXISTS user_sts_access ON STS;
DROP POLICY IF EXISTS user_pts_access ON PTS;
DROP POLICY IF EXISTS user_ownerhistory_access ON OwnerHistory;
DROP POLICY IF EXISTS user_ownerhistoryowner_access ON OwnerHistoryOwner;


REVOKE ALL PRIVILEGES ON TABLE Car, CarOwner, STS, PTS, OwnerHistory, OwnerHistoryOwner FROM audit_role;
REVOKE ALL PRIVILEGES ON TABLE Car, CarOwner, STS, PTS, OwnerHistory, OwnerHistoryOwner FROM operator_role;
REVOKE ALL PRIVILEGES ON TABLE Car, CarOwner, STS, PTS, OwnerHistory, OwnerHistoryOwner FROM user_role;


DROP ROLE IF EXISTS audit_role;
DROP ROLE IF EXISTS operator_role;
DROP ROLE IF EXISTS user_role;
