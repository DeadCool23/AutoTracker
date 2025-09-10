
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
