use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub struct Document {
    #[schema(example = "1111")]
    pub serial: String,
    #[schema(example = "111111")]
    pub number: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct Car {
    pub owner_fio: (String, String, Option<String>),
    pub gos_num: String,
    pub model: String,
    pub mark: String,
    pub color: String,
    pub year: u16,
    pub vin: String,
    pub sts: Document,
    pub pts: Document,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub enum Role {
    user,
    operator,
    audit,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub struct User {
    #[schema(example = "Ivan")]
    pub name: String,
    #[schema(example = "Ivanov")]
    pub surname: String,
    #[schema(example = "Ivanovich")]
    pub lastname: Option<String>,
    #[schema(example = "email@example.ru")]
    pub email: String,
    #[schema(example = "user")]
    pub role: Role,
    #[schema(example = true)]
    pub is_verified: bool,
    #[schema(example = json!({
        "serial": "1111",
        "number": "111111"
    }))]
    pub passport: Option<Document>,
}

#[derive(Debug, ToSchema, Clone, Copy, Serialize, Deserialize)]
pub struct Location {
    #[schema(example = 53.9222)]
    pub longitude: f64,
    #[schema(example = 53.9333)]
    pub latitude: f64,
}

#[derive(ToSchema, Deserialize, Serialize, Debug)]
pub struct PointData {
    #[schema(example = 60)]
    pub speed: Option<u16>,
    #[schema(example = json!([
        { "longitude": 54.98989 , "latitude": 56.89882 }]
    ))]
    pub cords: Location,
}

#[derive(Debug, ToSchema, Serialize, Deserialize, Clone)]
pub struct Snap {
    pub camera: Camera,
    pub time: String,
    pub speed: Option<u16>,
    pub date: String,
    pub gos_num: String,
}

#[derive(Debug, ToSchema, Clone, Copy, Serialize, Deserialize)]
pub struct Camera {
    #[schema(example = 1)]
    pub id: usize,
    #[schema(example = true)]
    pub is_radar: bool,
    #[schema(example = json!([
        { "longitude": 54.98989 , "latitude": 56.89882 }]
    ))]
    pub location: Location,
}

#[derive(Debug, ToSchema, Serialize, Deserialize)]
pub struct TrackInfo {
    pub track_time: String,
    pub route_date: String,
    pub car: Car,
    pub user: User,
}
