import Foundation

struct RequestRouteData: Encodable {
    let date: String
    let gos_num: String
    let user_login: String
}

struct ResponseRouteData: Decodable {
    let status: ResponseStatus
    let route: [PointData]?
}

class RouteGetAPIManager {
    static private let routePath: String = "/car/route"
    static func get(data: RequestRouteData) -> (ResponseRouteData?, Int) {
        let url_str = "\(APIUrl)\(routePath)"
        guard let url = URL(string: "\(url_str)") else {
            print("Invalid URL")
            fatalError("Invalid URL: \(url_str)")
        }
        print(url_str)
        
        print(data)
        guard let jsonData = try? JSONEncoder().encode(data) else {
            print("Failed to encode JSON")
            fatalError("Failed to encode JSON")
        }
        
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        request.httpBody = jsonData

        let data = templateRequestToAPI(request: request, ret_data: ResponseRouteData.self)
        return data
    }
}
