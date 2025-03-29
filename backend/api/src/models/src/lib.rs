use utoipa::ToSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct Document {
    #[schema(example = "1111")]
    pub serial: String,
    #[schema(example = "111111")]
    pub number: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct Car {
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
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub enum Role {
    user,
    operator,
    audit,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct User {
    #[schema(example = 1)]
    pub id: usize,
    #[schema(example = "Ivan")]
    pub name: String,
    #[schema(example = "Ivanov")]
    pub surname: String,
    #[schema(example = "Ivanovich")]
    pub lastname: Option<String>,
    #[schema(example = "email@example.ru")]
    pub email: String,
    #[schema(example = "12345678")]
    pub password: String,
    #[schema(example = "user")]
    pub role: Role,
    #[schema(example = true)]
    pub is_verified: bool,
    #[schema(example = json!({
        "serial": "1111",
        "number": "111111"
    }))]
    pub passport: Option<Document>,
    #[schema(example = json!([]))]
    pub cars: Option<Vec<Car>>,
}

#[derive(Debug, ToSchema, Clone, Copy, Serialize, Deserialize)]
pub struct Location {
    #[schema(example = 53.9222)]
    pub longitude: f64,
    #[schema(example = 53.9333)]
    pub latitude: f64,
}

#[derive(Debug, ToSchema, Serialize, Deserialize)]
pub struct Snap {
    pub camera: Camera,
    pub time: String,
    pub date: String,
    pub gos_num: String,
}

#[derive(Debug, ToSchema, Clone, Copy, Serialize, Deserialize)]
pub struct Camera {
    #[schema(example = 1)]
    pub id: usize,
    #[schema(example = json!([
        { "longitude": 54.98989 , "latitude": 56.89882 }]
    ))]
    pub location: Location,
}

#[derive(Debug, ToSchema, Serialize, Deserialize)]
pub struct TrackInfo {
    pub date: String,
    pub route_date: String,
    pub car: Car,
    pub user: User,
}
