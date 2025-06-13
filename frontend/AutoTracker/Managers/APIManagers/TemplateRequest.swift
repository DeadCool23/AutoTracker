import Foundation

func templateRequestToAPI<T>(request: URLRequest, ret_data: T.Type) -> (T?, Int) where T : Decodable {
    
    var local_request = request
    local_request.timeoutInterval = 5
    
    let semaphore = DispatchSemaphore(value: 0)
    var responseStatusCode = 503;
    var responseData: T? = nil
    var responseError: Error? = nil

    let task = URLSession.shared.dataTask(with: local_request) { data, response, error in
        defer { semaphore.signal() }

        if let error = error {
            responseError = error
            return
        }

        if let httpResponse = response as? HTTPURLResponse {
            responseStatusCode = httpResponse.statusCode
        }

        if let data = data {
            do {
                responseData = try JSONDecoder().decode(T.self, from: data)
            } catch {
                responseData = nil
                print("Ошибка декодирования JSON: \(error)")
            }
        }
        responseStatusCode = (response as! HTTPURLResponse).statusCode
    }

    task.resume()
    semaphore.wait()

    if responseError != nil {
        print(responseError!)
        return (nil, responseStatusCode)
    }
    
    print(responseData)
    print(responseStatusCode)

    return (responseData, responseStatusCode)
}
