use super::error::DataAccessError;
use super::repositories_traits;
use models::{Camera, Car, Document, Location, Role, Snap, TrackInfo, User};

pub struct MockUserRepo;

#[allow(unused_variables)]
#[async_trait::async_trait]
impl repositories_traits::UserRepository for MockUserRepo {
    async fn get_user_by_auth_info(
        &self,
        email: &str,
        pswd: &str,
    ) -> Result<Option<User>, DataAccessError> {
        Ok(Some(User {
            name: "mock_name".to_string(),
            surname: "mock_surname".to_string(),
            lastname: None,
            email: email.to_string(),
            passport: None,
            role: Role::user,
            is_verified: false,
        }))
    }
    async fn get_user_by_passport(
        &self,
        passport: &Document,
    ) -> Result<Option<User>, DataAccessError> {
        Ok(None)
    }
    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, DataAccessError> {
        if email == "exist@exist.com" {
            Ok(Some(User {
                name: "mock_name".to_string(),
                surname: "mock_surname".to_string(),
                lastname: None,
                email: email.to_string(),
                passport: None,
                role: Role::user,
                is_verified: false,
            }))
        } else {
            Ok(None)
        }
    }
    async fn insert_user(&self, user: &User, pswd: &str) -> Result<(), DataAccessError> {
        Ok(())
    }
    async fn update_user_passport(
        &self,
        email: &String,
        passport: &Document,
    ) -> Result<(), DataAccessError> {
        Ok(())
    }
}

pub struct MockCarRepo;

#[async_trait::async_trait]
#[allow(unused_variables)]
impl repositories_traits::CarRepository for MockCarRepo {
    async fn get_cars_by_filters(
        &self,
        firstname: Option<&str>,
        surname: Option<&str>,
        lastname: Option<&str>,
        passport: Option<Document>,
        gos_num_mask: Option<&str>,
    ) -> Result<Vec<Car>, DataAccessError> {
        Ok(vec![])
    }
    async fn get_car_by_gos_number_mask(
        &self,
        gos_number: &str,
    ) -> Result<Vec<Car>, DataAccessError> {
        Ok(vec![])
    }
    async fn get_car_by_owner_fio(
        &self,
        name: Option<&str>,
        surname: Option<&str>,
        last_name: Option<&str>,
    ) -> Result<Vec<Car>, DataAccessError> {
        Ok(vec![])
    }
    async fn get_car_by_owner_passport(
        &self,
        passport: &Document,
    ) -> Result<Vec<Car>, DataAccessError> {
        Ok(vec![])
    }
}

pub struct MockTrackInfoRepo;

#[async_trait::async_trait]
#[allow(unused_variables)]
impl repositories_traits::TrackInfoRepository for MockTrackInfoRepo {
    async fn insert_track_info(
        &self,
        gos_num: &str,
        user_login: &str,
        route_date: &str,
    ) -> Result<(), DataAccessError> {
        Ok(())
    }
    async fn get_tracks_info_by_filters(
        &self,
        firstname: Option<&str>,
        surname: Option<&str>,
        lastname: Option<&str>,
        passport: Option<Document>,
        gos_num_mask: Option<&str>,
        date: Option<&str>,
    ) -> Result<Vec<TrackInfo>, DataAccessError> {
        Ok(vec![])
    }
    async fn get_track_info_by_date(&self, date: &str) -> Result<Vec<TrackInfo>, DataAccessError> {
        Ok(vec![])
    }
    async fn get_track_info_by_car_gos_number_mask(
        &self,
        gos_number: &str,
    ) -> Result<Vec<TrackInfo>, DataAccessError> {
        Ok(vec![])
    }
    async fn get_track_info_by_user_fio(
        &self,
        name: Option<&str>,
        surname: Option<&str>,
        last_name: Option<&str>,
    ) -> Result<Vec<TrackInfo>, DataAccessError> {
        Ok(vec![])
    }
    async fn get_track_info_by_user_passport(
        &self,
        passport: &Document,
    ) -> Result<Vec<TrackInfo>, DataAccessError> {
        Ok(vec![])
    }
}

pub struct MockSnapRepo;

#[async_trait::async_trait]
#[allow(unused_variables)]
impl repositories_traits::SnapRepository for MockSnapRepo {
    async fn insert_snap(&self, snap: &Snap) -> Result<(), DataAccessError> {
        Ok(())
    }
    async fn get_car_snaps_by_date(
        &self,
        gos_number: &str,
        date: &str,
    ) -> Result<Vec<Snap>, DataAccessError> {
        Ok(vec![
            Snap {
                speed: Some(70),
                gos_num: gos_number.to_string(),
                camera: Camera {
                    id: 1,
                    is_radar: true,
                    location: Location {
                        latitude: 55.75222,
                        longitude: 37.61556,
                    },
                },
                date: date.to_string(),
                time: "8:10".to_string(),
            },
            Snap {
                speed: Some(70),
                gos_num: gos_number.to_string(),
                camera: Camera {
                    id: 2,
                    is_radar: true,
                    location: Location {
                        latitude: 77.75222,
                        longitude: 47.61556,
                    },
                },
                date: date.to_string(),
                time: "9:15".to_string(),
            },
            Snap {
                speed: Some(70),
                gos_num: gos_number.to_string(),
                camera: Camera {
                    id: 1,
                    is_radar: true,
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

#[allow(unused_variables)]
#[async_trait::async_trait]
impl repositories_traits::CameraRepository for MockCameraRepo {
    async fn get_camera_count(&self) -> Result<usize, DataAccessError> {
        Ok(10)
    }
    async fn get_camera_by_id(&self, id: usize) -> Result<Camera, DataAccessError> {
        Ok(Camera {
            id,
            is_radar: true,
            location: Location {
                latitude: 55.75222,
                longitude: 37.61556,
            },
        })
    }
    async fn get_camera_by_location(&self, location: &Location) -> Result<Camera, DataAccessError> {
        Ok(Camera {
            id: 1,
            is_radar: true,
            location: location.clone(),
        })
    }
    async fn get_avg_speed_for_car_at_camera(
        &self,
        gos_num: &str,
        cam_id: usize,
    ) -> Result<f64, DataAccessError> {
        Ok(70.)
    }
}
