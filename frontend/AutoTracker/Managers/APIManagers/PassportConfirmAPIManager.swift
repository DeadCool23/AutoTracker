import Foundation

struct PassportConfirmRequest: Codable {
    let email: String
    let passport: Document
}

struct ResponsePassportConfirmData: Decodable {
    let status: ResponseStatus
}

class PassportConfirmAPIManager {
    static private let passportConfirmPath: String = "/user/passport-confirm"
    static func confirmPassport(data: PassportConfirmRequest) -> (ResponsePassportConfirmData?, Int) {
        guard let url = URL(string: "\(APIUrl)\(passportConfirmPath)") else {
            print("Invalid URL")
            fatalError("Invalid URL: \(APIUrl)\(passportConfirmPath)")
        }
        
        guard let jsonData = try? JSONEncoder().encode(data) else {
            print("Failed to encode JSON")
            fatalError("Failed to encode JSON")
        }
        
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        request.httpBody = jsonData

        let data = templateRequestToAPI(request: request, ret_data: ResponsePassportConfirmData.self)
        return data
    }
}
