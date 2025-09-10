use super::*;
use super::bulder::*;

pub struct DocumentMother;

impl DocumentMother {
    pub fn valid_document() -> Document {
        DocumentBuilder::new()
            .serial("1111")
            .number("111111")
            .build()
    }
    
    pub fn invalid_document() -> Document {
        DocumentBuilder::new()
            .serial("abc")
            .number("hgygj")
            .build()
    }
}

pub struct CarMother;

impl CarMother {
    pub fn valid_car() -> Car {
        CarBuilder::new()
            .owner_fio("Иван", "Иванов", Some("Иванович".to_string()))
            .gos_num("А123БВ777")
            .model("Solaris")
            .mark("Hyundai")
            .color("Белый")
            .year(2020)
            .vin("Z94CB41AAGR323012")
            .sts(DocumentMother::valid_document())
            .pts(DocumentMother::valid_document())
            .build()
    }
    
    pub fn invalid_gos_num_car() -> Car {
        CarBuilder::new()
            .owner_fio("Петр", "Петров", None)
            .gos_num("В456Д123")
            .model("Camry")
            .mark("Toyota")
            .color("Черный")
            .year(2018)
            .vin("JTDBU4EE7AJ123456")
            .sts(DocumentMother::valid_document())
            .pts(DocumentMother::valid_document())
            .build()
    }
    
    pub fn invalid_vin_car() -> Car {
        CarBuilder::new()
            .owner_fio("Сергей", "Сидоров", Some("Петрович".to_string()))
            .gos_num("Е789ЖК456")
            .model("Logan")
            .mark("Renault")
            .color("Серый")
            .year(2015)
            .vin("VF1L0H0Y35456789")
            .sts(DocumentMother::valid_document())
            .pts(DocumentMother::valid_document())
            .build()
    }
}

pub struct UserMother;

impl UserMother {
    pub fn valid_user() -> User {
        UserBuilder::new()
            .name("Иван")
            .surname("Иванов")
            .lastname(Some("Иванович".to_string()))
            .email("ivanov@example.ru")
            .role(Role::user)
            .is_verified(true)
            .passport(Some(DocumentMother::valid_document()))
            .build()
    }
    
    pub fn operator_user() -> User {
        UserBuilder::new()
            .name("Оператор")
            .surname("Системный")
            .lastname(None)
            .email("operator@system.ru")
            .role(Role::operator)
            .is_verified(true)
            .passport(Some(DocumentMother::valid_document()))
            .build()
    }
    
    pub fn audit_user() -> User {
        UserBuilder::new()
            .name("Аудитор")
            .surname("Проверяющий")
            .lastname(Some("Сергеевич".to_string()))
            .email("audit@company.ru")
            .role(Role::audit)
            .is_verified(true)
            .passport(None)
            .build()
    }
    
    pub fn unverified_user() -> User {
        UserBuilder::new()
            .name("Неверифицированный")
            .surname("Пользователь")
            .lastname(None)
            .email("unverified@mail.ru")
            .role(Role::user)
            .is_verified(false)
            .passport(None)
            .build()
    }

    pub fn exist_user() -> User {
        UserBuilder::new()
            .name("Неверифицированный")
            .surname("Пользователь")
            .lastname(None)
            .email("exist@exist.ru")
            .role(Role::user)
            .is_verified(false)
            .passport(None)
            .build()
    }
}

pub struct LocationMother;

impl LocationMother {
    pub fn moscow_location() -> Location {
        LocationBuilder::new()
            .longitude(37.6173)
            .latitude(55.7558)
            .build()
    }
    
    pub fn zero_location() -> Location {
        LocationBuilder::new()
            .longitude(0.0)
            .latitude(0.0)
            .build()
    }
}

pub struct PointDataMother;

impl PointDataMother {
    pub fn moving_point() -> PointData {
        PointDataBuilder::new()
            .speed(Some(60))
            .cords(LocationMother::moscow_location())
            .build()
    }
    
    pub fn unknown_speed_point() -> PointData {
        PointDataBuilder::new()
            .speed(None)
            .cords(LocationMother::moscow_location())
            .build()
    }
}

pub struct CameraMother;

impl CameraMother {
    pub fn radar_camera() -> Camera {
        CameraBuilder::new()
            .id(1)
            .is_radar(true)
            .location(LocationMother::moscow_location())
            .build()
    }
    
    pub fn regular_camera() -> Camera {
        CameraBuilder::new()
            .id(2)
            .is_radar(false)
            .location(LocationMother::moscow_location())
            .build()
    }
}

pub struct SnapMother;

impl SnapMother {
    pub fn speed_violation_snap() -> Snap {
        SnapBuilder::new()
            .camera(CameraMother::radar_camera())
            .time("12:30")
            .speed(Some(90))
            .date("15.01.2025")
            .gos_num("А123БВ777")
            .build()
    }
    
    pub fn invalid_gos_num_snap() -> Snap {
        SnapBuilder::new()
            .camera(CameraMother::regular_camera())
            .time("15:20")
            .speed(Some(50))
            .date("15.01.2025")
            .gos_num("В456Г123")
            .build()
    }
    
    pub fn invalid_date_snap() -> Snap {
        SnapBuilder::new()
            .camera(CameraMother::regular_camera())
            .time("18:45")
            .speed(None)
            .date("2024-01-15")
            .gos_num("Е789ЖК456")
            .build()
    }

    pub fn invalid_time_snap() -> Snap {
        SnapBuilder::new()
            .camera(CameraMother::regular_camera())
            .time("18-45")
            .speed(None)
            .date("15.01.2025")
            .gos_num("Е789ЖК456")
            .build()
    }
}

pub struct TrackInfoMother;

impl TrackInfoMother {
    pub fn complete_track_info() -> TrackInfo {
        TrackInfoBuilder::new()
            .track_time("14:30")
            .route_date("15.01.2025")
            .car(CarMother::valid_car())
            .user(UserMother::valid_user())
            .build()
    }
    
    pub fn operator_track_info() -> TrackInfo {
        TrackInfoBuilder::new()
            .track_time("10:15")
            .route_date("15.01.2025")
            .car(CarMother::valid_car())
            .user(UserMother::operator_user())
            .build()
    }
}