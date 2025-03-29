use super::error::DataAccessError;
use super::repositories_traits;
use models::{Camera, Car, Document, Location, Role, Snap, TrackInfo, User};

pub struct MockUserRepo;

#[allow(unused_variables)]
impl repositories_traits::UserRepository for MockUserRepo {
    fn get_user_by_auth_info(
        &self,
        email: &str,
        pswd: &str,
    ) -> Result<Option<User>, DataAccessError> {
        Ok(Some(User {
            id: 1,
            name: "mock_name".to_string(),
            surname: "mock_surname".to_string(),
            lastname: None,
            email: email.to_string(),
            passport: None,
            cars: None,
            password: pswd.to_string(),
            role: Role::user,
            is_verified: false,
        }))
    }
    fn get_user_by_email(&self, email: &str) -> Result<Option<User>, DataAccessError> {
        if email == "exist@exist.com" {
            Ok(Some(User {
                id: 1,
                name: "mock_name".to_string(),
                surname: "mock_surname".to_string(),
                lastname: None,
                email: email.to_string(),
                passport: None,
                cars: None,
                password: "1234567890".to_string(),
                role: Role::user,
                is_verified: false,
            }))
        } else {
            Ok(None)
        }
    }
    fn get_user_by_passport(
        &self,
        passport: &Document,
    ) -> Result<Option<Vec<User>>, DataAccessError> {
        unimplemented!()
    }
    fn get_user_by_fio(
        &self,
        name: Option<&str>,
        surname: Option<&str>,
        last_name: Option<&str>,
    ) -> Result<Option<Vec<User>>, DataAccessError> {
        unimplemented!()
    }
    fn add_user(&self, user: &User) -> Result<(), DataAccessError> {
        Ok(())
    }
    fn update_user_passport(
        &self,
        email: &String,
        passport: &Document,
    ) -> Result<(), DataAccessError> {
        Ok(())
    }
}

pub struct MockCarRepo;

#[allow(unused_variables)]
impl repositories_traits::CarRepository for MockCarRepo {
    fn get_car_by_gos_number_mask(&self, gos_number: &str) -> Result<Vec<Car>, DataAccessError> {
        Ok(vec![])
    }
    fn get_car_by_owner_fio(
        &self,
        name: Option<&str>,
        surname: Option<&str>,
        last_name: Option<&str>,
    ) -> Result<Vec<Car>, DataAccessError> {
        Ok(vec![])
    }
    fn get_car_by_owner_passport(&self, passport: &Document) -> Result<Vec<Car>, DataAccessError> {
        Ok(vec![])
    }
}

pub struct MockTrackInfoRepo;

#[allow(unused_variables)]
impl repositories_traits::TrackInfoRepository for MockTrackInfoRepo {
    fn insert_track_info(
        &self,
        gos_num: &str,
        user_id: usize,
        route_date: &str,
    ) -> Result<(), DataAccessError> {
        Ok(())
    }
    fn get_track_info_by_date(&self, date: &str) -> Result<Vec<TrackInfo>, DataAccessError> {
        Ok(vec![])
    }
    fn get_track_info_by_car_gos_number_mask(
        &self,
        gos_number: &str,
    ) -> Result<Vec<TrackInfo>, DataAccessError> {
        Ok(vec![])
    }
    fn get_track_info_by_user_fio(
        &self,
        name: Option<&str>,
        surname: Option<&str>,
        last_name: Option<&str>,
    ) -> Result<Vec<TrackInfo>, DataAccessError> {
        Ok(vec![])
    }
    fn get_track_info_by_user_passport(
        &self,
        passport: &Document,
    ) -> Result<Vec<TrackInfo>, DataAccessError> {
        Ok(vec![])
    }
}

pub struct MockSnapRepo;

#[allow(unused_variables)]
impl repositories_traits::SnapRepository for MockSnapRepo {
    fn insert_snap(&self, snap: &Snap) -> Result<(), DataAccessError> {
        Ok(())
    }
    fn get_car_snaps_by_date(
        &self,
        gos_number: &str,
        date: &str,
    ) -> Result<Vec<Snap>, DataAccessError> {
        Ok(vec![
            Snap {
                gos_num: gos_number.to_string(),
                camera: Camera {
                    id: 1,
                    location: Location {
                        latitude: 55.75222,
                        longitude: 37.61556,
                    },
                },
                date: date.to_string(),
                time: "8:10".to_string(),
            },
            Snap {
                gos_num: gos_number.to_string(),
                camera: Camera {
                    id: 2,
                    location: Location {
                        latitude: 77.75222,
                        longitude: 47.61556,
                    },
                },
                date: date.to_string(),
                time: "9:15".to_string(),
            },
            Snap {
                gos_num: gos_number.to_string(),
                camera: Camera {
                    id: 1,
                    location: Location {
                        latitude: 55.75222,
                        longitude: 37.61556,
                    },
                },
                date: date.to_string(),
                time: "10:30".to_string(),
            },
        ])
    }
}

pub struct MockCameraRepo;

impl repositories_traits::CameraRepository for MockCameraRepo {
    fn get_camera_count(&self) -> Result<usize, DataAccessError> {
        Ok(10)
    }
    fn get_camera_by_id(&self, id: usize) -> Result<Camera, DataAccessError> {
        Ok(Camera {
            id: id,
            location: Location {
                latitude: 55.75222,
                longitude: 37.61556,
            }
        })
    }
}