import Foundation

struct RequestAvgSpeedData: Encodable {
    let gos_num: String
    let location: Coordinate
}

struct ResponseAvgSpeedData: Decodable {
    let status: ResponseStatus
    let avg_speed: Double?
}

class CameraAPIManager {
    static private let cameraPath: String = "/camera"
    static private let avgSpeedPath: String = "\(CameraAPIManager.cameraPath)/avg-speed"
    
    static func getAvgSpeed(data: RequestAvgSpeedData) -> (ResponseAvgSpeedData?, Int) {
        guard let url = URL(string: "\(APIUrl)\(CameraAPIManager.avgSpeedPath)") else {
            print("Invalid URL")
            fatalError("Invalid URL: \(APIUrl)\(CameraAPIManager.avgSpeedPath)")
        }
        
        guard let jsonData = try? JSONEncoder().encode(data) else {
            print("Failed to encode JSON")
            fatalError("Failed to encode JSON")
        }
        
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        request.httpBody = jsonData

        let data = templateRequestToAPI(request: request, ret_data: ResponseAvgSpeedData.self)
        return data
    }
}
