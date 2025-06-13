//#[derive(Serialize, Deserialize, Debug, ToSchema)]
//pub struct Car {
//    pub owner_fio: (String, String, Option<String>),
//    pub gos_num: String,
//    pub model: String,
//    pub mark: String,
//    pub color: String,
//    pub year: u16,
//    pub vin: String,
//    pub sts: Document,
//    pub pts: Document,
//}

struct OwnerFIO: Codable {
    let surname: String
    let name: String
    let lastname: String?
}

extension OwnerFIO {
    init(from decoder: Decoder) throws {
        var container = try decoder.unkeyedContainer()
        let surname = try container.decode(String.self)
        let name = try container.decode(String.self)
        let lastname = try container.decodeIfPresent(String.self)
        
        self.init(surname: surname, name: name, lastname: lastname)
    }
}

struct Car: Codable {
    let owner_fio: OwnerFIO
    let gos_num: String
    let model: String
    let mark: String
    let color: String
    let year: UInt
    let vin: String
    let sts: Document
    let pts: Document
}
