import Foundation

struct ResponseAuthData: Decodable {
    let status: ResponseStatus
    let user: User?
}

class AuthAPIManager {
    static private let authPath: String = "/user/auth"
    static func auth(login: String, password: String) -> (ResponseAuthData?, Int) {
        guard let url = URL(string: "\(APIUrl)\(AuthAPIManager.authPath)") else {
            print("Invalid URL")
            fatalError("Invalid URL: \(APIUrl)\(AuthAPIManager.authPath)")
        }
        
        let userData: [String: Any] = [
            "email": login,
            "pswd": password
        ]
        
        guard let jsonData = try? JSONSerialization.data(withJSONObject: userData, options: []) else {
            print("Failed to encode JSON")
            fatalError("Failed to encode JSON")
        }
        
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        request.httpBody = jsonData

        let data = templateRequestToAPI(request: request, ret_data: ResponseAuthData.self)
        print(data.1)
        return data
    }
}
