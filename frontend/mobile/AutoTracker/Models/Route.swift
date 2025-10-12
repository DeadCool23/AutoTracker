struct Coordinate: Codable {
    let latitude: Double
    let longitude: Double
}

struct PointData: Codable {
    var cords: Coordinate
    var speed: UInt16?
}

struct Route: Codable {
    var cords: [PointData]
}
