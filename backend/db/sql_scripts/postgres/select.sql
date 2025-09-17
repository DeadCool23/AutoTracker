SELECT * FROM appuser WHERE login = 'qwerty@gmail.com';

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