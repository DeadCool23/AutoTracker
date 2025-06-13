import Foundation

struct SearchCarRequest: Encodable {
    let lastname: String?
    let surname: String?
    let name: String?
    let passport: Document?
    let gos_num: String?
}

struct SearchTrackInfoRequest: Encodable {
    let lastname: String?
    let surname: String?
    let name: String?
    let passport: Document?
    let date: String?
    let gos_num: String?
}

struct SearchByFIORequest: Encodable {
    let lastname: String?
    let surname: String?
    let name: String?
}

struct SearchByPassportRequest: Encodable {
    let passport: Document
}

struct SearchByDateRequest: Encodable {
    let date: String
}

struct SearchByGosNumMaskRequest: Encodable {
    let gos_num: String
}

struct FindedCarsResponse: Decodable {
    let status: ResponseStatus
    let cars: [Car]
}

struct FindedTrackInfoResponse: Decodable {
    let status: ResponseStatus
    let track_info: [TrackInfo]
}

class SearchAPIManager {
    static private let searchPath = "/search"
    
    static private let carPath = "/car"
    static private let trackInfoPath = "/track-info"
    
    static private let carSearchPath = "\(carPath)\(searchPath)"
    static private let trackInfoSearchPath = "\(trackInfoPath)\(searchPath)"
    
    static private let searchByFIOPath = "/by-fio"
    static private let searchByDatePath = "/by-date"
    static private let searchByPassportPath = "/by-passport"
    static private let searchByGosNumMaskPath = "/by-gos-num-mask"
    
    static func searchCar(filters: SearchCarRequest) -> (FindedCarsResponse?, Int) {
        let url_str = "\(APIUrl)\(carSearchPath)"
        guard let url = URL(string: "\(url_str)") else {
            print("Invalid URL")
            fatalError("Invalid URL: \(url_str)")
        }
        
        print(filters)
        
        guard let jsonData = try? JSONEncoder().encode(filters) else {
            print("Failed to encode JSON")
            fatalError("Failed to encode JSON")
        }
        
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        request.httpBody = jsonData

        let data = templateRequestToAPI(request: request, ret_data: FindedCarsResponse.self)
        return data
    }
    
    static func searchTrackInfo(filters: SearchTrackInfoRequest) -> (FindedTrackInfoResponse?, Int) {
        let url_str = "\(APIUrl)\(trackInfoSearchPath)"
        guard let url = URL(string: "\(url_str)") else {
            print("Invalid URL")
            fatalError("Invalid URL: \(url_str)")
        }
        
        print(filters)
        
        guard let jsonData = try? JSONEncoder().encode(filters) else {
            print("Failed to encode JSON")
            fatalError("Failed to encode JSON")
        }
        
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        request.httpBody = jsonData

        let data = templateRequestToAPI(request: request, ret_data: FindedTrackInfoResponse.self)
        return data
    }
    
    static func searchCar(fio: SearchByFIORequest) -> (FindedCarsResponse?, Int) {
        let url_str = "\(APIUrl)\(carSearchPath)\(searchByFIOPath)"
        guard let url = URL(string: "\(url_str)") else {
            print("Invalid URL")
            fatalError("Invalid URL: \(url_str)")
        }
        
        print(fio)
        
        guard let jsonData = try? JSONEncoder().encode(fio) else {
            print("Failed to encode JSON")
            fatalError("Failed to encode JSON")
        }
        
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        request.httpBody = jsonData

        let data = templateRequestToAPI(request: request, ret_data: FindedCarsResponse.self)
        return data
    }
    
    static func searchTrackInfo(fio: SearchByFIORequest) -> (FindedTrackInfoResponse?, Int) {
        let url_str = "\(APIUrl)\(trackInfoSearchPath)\(searchByFIOPath)"
        guard let url = URL(string: "\(url_str)") else {
            print("Invalid URL")
            fatalError("Invalid URL: \(url_str)")
        }
        
        print(fio)
        
        guard let jsonData = try? JSONEncoder().encode(fio) else {
            print("Failed to encode JSON")
            fatalError("Failed to encode JSON")
        }
        
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        request.httpBody = jsonData

        let data = templateRequestToAPI(request: request, ret_data: FindedTrackInfoResponse.self)
        return data
    }
    
    static func searchCar(passport: SearchByPassportRequest) -> (FindedCarsResponse?, Int) {
        let url_str = "\(APIUrl)\(carSearchPath)\(searchByPassportPath)"
        guard let url = URL(string: "\(url_str)") else {
            print("Invalid URL")
            fatalError("Invalid URL: \(url_str)")
        }
        
        print(passport)
            
        guard let jsonData = try? JSONEncoder().encode(passport) else {
            print("Failed to encode JSON")
            fatalError("Failed to encode JSON")
        }
        
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        request.httpBody = jsonData

        let data = templateRequestToAPI(request: request, ret_data: FindedCarsResponse.self)
        return data
    }
    
    static func searchTrackInfo(passport: SearchByPassportRequest) -> (FindedTrackInfoResponse?, Int) {
        let url_str = "\(APIUrl)\(trackInfoSearchPath)\(searchByPassportPath)"
        guard let url = URL(string: "\(url_str)") else {
            print("Invalid URL")
            fatalError("Invalid URL: \(url_str)")
        }
        
        print(passport)
        
        guard let jsonData = try? JSONEncoder().encode(passport) else {
            print("Failed to encode JSON")
            fatalError("Failed to encode JSON")
        }
        
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        request.httpBody = jsonData

        let data = templateRequestToAPI(request: request, ret_data: FindedTrackInfoResponse.self)
        return data
    }
    
    static func searchCar(gos_num_mask: SearchByGosNumMaskRequest) -> (FindedCarsResponse?, Int) {
        let url_str = "\(APIUrl)\(carSearchPath)\(searchByGosNumMaskPath)"
        guard let url = URL(string: "\(url_str)") else {
            print("Invalid URL")
            fatalError("Invalid URL: \(url_str)")
        }
        
        print(gos_num_mask)
        
        guard let jsonData = try? JSONEncoder().encode(gos_num_mask) else {
            print("Failed to encode JSON")
            fatalError("Failed to encode JSON")
        }
        
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        request.httpBody = jsonData

        let data = templateRequestToAPI(request: request, ret_data: FindedCarsResponse.self)
        return data
    }
    
    static func searchTrackInfo(gos_num_mask: SearchByGosNumMaskRequest) -> (FindedTrackInfoResponse?, Int) {
        let url_str = "\(APIUrl)\(trackInfoSearchPath)\(searchByGosNumMaskPath)"
        guard let url = URL(string: "\(url_str)") else {
            print("Invalid URL")
            fatalError("Invalid URL: \(url_str)")
        }
        
        print(gos_num_mask)
        
        guard let jsonData = try? JSONEncoder().encode(gos_num_mask) else {
            print("Failed to encode JSON")
            fatalError("Failed to encode JSON")
        }
        
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        request.httpBody = jsonData

        let data = templateRequestToAPI(request: request, ret_data: FindedTrackInfoResponse.self)
        return data
    }
    
    static func searchTrackInfo(date: SearchByDateRequest) -> (FindedTrackInfoResponse?, Int) {
        let url_str = "\(APIUrl)\(trackInfoSearchPath)\(searchByDatePath)"
        guard let url = URL(string: "\(url_str)") else {
            print("Invalid URL")
            fatalError("Invalid URL: \(url_str)")
        }
        
        print(date)
        
        guard let jsonData = try? JSONEncoder().encode(date) else {
            print("Failed to encode JSON")
            fatalError("Failed to encode JSON")
        }
        
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        request.httpBody = jsonData

        let data = templateRequestToAPI(request: request, ret_data: FindedTrackInfoResponse.self)
        return data
    }
}
