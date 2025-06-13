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



-- ТЕСТИРОВАНИЕ ПРОЦЕДУРЫ ========================================

CREATE EXTENSION IF NOT EXISTS pgtap;;

BEGIN;
SELECT no_plan();

INSERT INTO AppUser (login, role, name, surname, is_verified, passport_serial, passport_num, password) 
VALUES 
    ('test_user1@mail.ru', 'user', 'A', 'AA', FALSE, NULL, NULL, '12345678'),
    ('test_user2@mail.ru', 'user', 'B', 'BB', FALSE, NULL, NULL, '12345678'),
    ('test_user3@mail.ru', 'user', 'C', 'CC', TRUE, 1234, 567890, '12345678'),
    ('test_user4@mail.ru', 'user', 'D', 'DD', TRUE, 4321, 987654, '12345678')
ON CONFLICT (login) DO NOTHING;

-- 1. Успешная верификация пользователя
-- ==
SELECT lives_ok(
    $$CALL verify_user('test_user1@mail.ru', 1234, 567891)$$,
    'Верификация пользователя с корректными данными должна пройти успешно'
);

-- Проверка обновления флага is_verified
SELECT ok(
    (SELECT is_verified FROM AppUser WHERE login = 'test_user1@mail.ru'),
    'После верификации флаг is_verified должен быть TRUE'
);

-- Проверка обновления паспортных данных
SELECT is(
    (SELECT passport_serial FROM AppUser WHERE login = 'test_user1@mail.ru'),
    1234,
    'Серия паспопрта должна обновиться'
);

SELECT is(
    (SELECT passport_num FROM AppUser WHERE login = 'test_user1@mail.ru'),
    567891,
    'Номер паспопрта должен обновиться'
);
-- ==

-- 2. Попытка верификации несуществующего пользователя
-- ==
SELECT throws_ok(
    $$CALL verify_user('non_existent_user@mail.ru', 1111, 222222)$$,
    'P0001',
    'User with login non_existent_user@mail.ru does not exist',
    'При верификации несуществующего пользователя должно быть исключение'
);
-- ==

-- 3. Попытка использовать существующие паспортные данные
-- ==
SELECT throws_ok(
    $$CALL verify_user('test_user2@mail.ru', 1234, 567890)$$,
    'P0001',
    'Passport data already exists for another user',
    'При использовании существующих паспортных данных должно быть исключение'
);
-- ==

-- 4. Обновление своих же паспортных данных
-- ==
SELECT lives_ok(
    $$CALL verify_user('test_user4@mail.ru', 4321, 987654)$$,
    'Должна разрешаться верификация с теми же паспортными данными'
);
-- ==

-- 5. Проверка на NULL значения
-- ==
SELECT throws_ok(
    $$CALL verify_user(NULL, 1234, 567891)$$,
    'P0001',
    'Login cannot be NULL',
    'При u_login = NULL должно быть исключение'
);

SELECT throws_ok(
    $$CALL verify_user('test_user4@mail.ru', NULL, 567891)$$,
    'P0001',
    'Passport serial cannot be NULL',
    'При new_passport_serial = NULL должно быть исключение'
);

SELECT throws_ok(
    $$CALL verify_user('test_user4@mail.ru', 1234, NULL)$$,
    'P0001',
    'Passport number cannot be NULL',
    'При new_passport_num = NULL должно быть исключение'
);
-- ==

SELECT * FROM finish();
ROLLBACK;

DROP EXTENSION pgtap;