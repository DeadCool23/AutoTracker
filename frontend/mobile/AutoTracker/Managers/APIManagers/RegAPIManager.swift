import Foundation

struct RequestRegData: Encodable {
    let firstname: String
    let surname: String
    let lastname: String
    let email: String
    let pswd: String
    let rep_pswd: String
}

struct ResponseRegData: Decodable {
    let status: ResponseStatus
}

class RegAPIManager {
    static private let regPath: String = "/user/registr"
    static func reg(data: RequestRegData) -> (ResponseAuthData?, Int) {
        guard let url = URL(string: "\(APIUrl)\(RegAPIManager.regPath)") else {
            print("Invalid URL")
            fatalError("Invalid URL: \(APIUrl)\(RegAPIManager.regPath)")
        }
        
        guard let jsonData = try? JSONEncoder().encode(data) else {
            print("Failed to encode JSON")
            fatalError("Failed to encode JSON")
        }
        
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        request.httpBody = jsonData

        let data = templateRequestToAPI(request: request, ret_data: ResponseAuthData.self)
        return data
    }
}
