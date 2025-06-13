-- Active: 1742486282168@@127.0.0.1@5432@auto_tracker
SELECT * FROM appuser WHERE login = 'qwerty@gmail.com';

-- qwerty@gmail.com, 12345678 | user
-- aaaaa@icloud.com, 12345678 | audit
-- egorik@gmail.com, 12345678 | operator

-- Кол-во машин пользователя
SELECT a.login, a.password, a.role, count(*) as car_cnt from 
appuser a 
JOIN carowner o on o.passport_num = a.passport_num AND o.passport_serial = a.passport_serial 
JOIN car c on o.id = c.owner_id
GROUP BY a.id
ORDER BY car_cnt DESC;

-- Кол-во проеханых точек в дату
SELECT s.gos_num, sn.snap_datetime::date, count(*) as cnt FROM car c 
JOIN sts s ON s.car_id = c.id 
JOIN carsnapshot sn ON sn.gos_num = s.gos_num
GROUP BY (s.gos_num, sn.snap_datetime::date)
ORDER BY cnt desc;

-- operator check
-- С452ХМ37, 2025-04-23
-- Е119АХ37, 2025-04-23

-- user check for qwerty@gmail.com and id = 93 passport 4721 821463
-- В937ОН67, 2025-04-28
