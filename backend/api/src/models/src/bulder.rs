use super::*;

pub struct DocumentBuilder {
    document: Document,
}

impl DocumentBuilder {
    pub fn new() -> Self {
        Self {
            document: Document::default(),
        }
    }

    pub fn serial(mut self, serial: impl Into<String>) -> Self {
        self.document.serial = serial.into();
        self
    }

    pub fn number(mut self, number: impl Into<String>) -> Self {
        self.document.number = number.into();
        self
    }

    pub fn build(self) -> Document {
        self.document
    }
}

pub struct CarBuilder {
    car: Car,
}

impl CarBuilder {
    pub fn new() -> Self {
        Self {
            car: Car::default(),
        }
    }

    pub fn owner_fio(
        mut self,
        first_name: impl Into<String>,
        last_name: impl Into<String>,
        middle_name: Option<String>,
    ) -> Self {
        self.car.owner_fio = (first_name.into(), last_name.into(), middle_name);
        self
    }

    pub fn gos_num(mut self, gos_num: impl Into<String>) -> Self {
        self.car.gos_num = gos_num.into();
        self
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.car.model = model.into();
        self
    }

    pub fn mark(mut self, mark: impl Into<String>) -> Self {
        self.car.mark = mark.into();
        self
    }

    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.car.color = color.into();
        self
    }

    pub fn year(mut self, year: u16) -> Self {
        self.car.year = year;
        self
    }

    pub fn vin(mut self, vin: impl Into<String>) -> Self {
        self.car.vin = vin.into();
        self
    }

    pub fn sts(mut self, sts: Document) -> Self {
        self.car.sts = sts;
        self
    }

    pub fn pts(mut self, pts: Document) -> Self {
        self.car.pts = pts;
        self
    }

    pub fn build(self) -> Car {
        self.car
    }
}

pub struct UserBuilder {
    user: User,
}

impl UserBuilder {
    pub fn new() -> Self {
        Self {
            user: User::default(),
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.user.name = name.into();
        self
    }

    pub fn surname(mut self, surname: impl Into<String>) -> Self {
        self.user.surname = surname.into();
        self
    }

    pub fn lastname(mut self, lastname: Option<String>) -> Self {
        self.user.lastname = lastname;
        self
    }

    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.user.email = email.into();
        self
    }

    pub fn role(mut self, role: Role) -> Self {
        self.user.role = role;
        self
    }

    pub fn is_verified(mut self, is_verified: bool) -> Self {
        self.user.is_verified = is_verified;
        self
    }

    pub fn passport(mut self, passport: Option<Document>) -> Self {
        self.user.passport = passport;
        self
    }

    pub fn build(self) -> User {
        self.user
    }
}

pub struct LocationBuilder {
    location: Location,
}

impl LocationBuilder {
    pub fn new() -> Self {
        Self {
            location: Location::default(),
        }
    }

    pub fn longitude(mut self, longitude: f64) -> Self {
        self.location.longitude = longitude;
        self
    }

    pub fn latitude(mut self, latitude: f64) -> Self {
        self.location.latitude = latitude;
        self
    }

    pub fn build(self) -> Location {
        self.location
    }
}

pub struct PointDataBuilder {
    point_data: PointData,
}

impl PointDataBuilder {
    pub fn new() -> Self {
        Self {
            point_data: PointData::default(),
        }
    }

    pub fn speed(mut self, speed: Option<u16>) -> Self {
        self.point_data.speed = speed;
        self
    }

    pub fn cords(mut self, cords: Location) -> Self {
        self.point_data.cords = cords;
        self
    }

    pub fn build(self) -> PointData {
        self.point_data
    }
}

pub struct CameraBuilder {
    camera: Camera,
}

impl CameraBuilder {
    pub fn new() -> Self {
        Self {
            camera: Camera::default(),
        }
    }

    pub fn id(mut self, id: usize) -> Self {
        self.camera.id = id;
        self
    }

    pub fn is_radar(mut self, is_radar: bool) -> Self {
        self.camera.is_radar = is_radar;
        self
    }

    pub fn location(mut self, location: Location) -> Self {
        self.camera.location = location;
        self
    }

    pub fn build(self) -> Camera {
        self.camera
    }
}

pub struct SnapBuilder {
    snap: Snap,
}

impl SnapBuilder {
    pub fn new() -> Self {
        Self {
            snap: Snap::default(),
        }
    }

    pub fn camera(mut self, camera: Camera) -> Self {
        self.snap.camera = camera;
        self
    }

    pub fn time(mut self, time: impl Into<String>) -> Self {
        self.snap.time = time.into();
        self
    }

    pub fn speed(mut self, speed: Option<u16>) -> Self {
        self.snap.speed = speed;
        self
    }

    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.snap.date = date.into();
        self
    }

    pub fn gos_num(mut self, gos_num: impl Into<String>) -> Self {
        self.snap.gos_num = gos_num.into();
        self
    }

    pub fn build(self) -> Snap {
        self.snap
    }
}

pub struct TrackInfoBuilder {
    track_info: TrackInfo,
}

impl TrackInfoBuilder {
    pub fn new() -> Self {
        Self {
            track_info: TrackInfo::default(),
        }
    }

    pub fn track_time(mut self, track_time: impl Into<String>) -> Self {
        self.track_info.track_time = track_time.into();
        self
    }

    pub fn route_date(mut self, route_date: impl Into<String>) -> Self {
        self.track_info.route_date = route_date.into();
        self
    }

    pub fn car(mut self, car: Car) -> Self {
        self.track_info.car = car;
        self
    }

    pub fn user(mut self, user: User) -> Self {
        self.track_info.user = user;
        self
    }

    pub fn build(self) -> TrackInfo {
        self.track_info
    }
}
