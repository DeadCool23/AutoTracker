enum UserRole: String, Codable {
    case audit_ = "audit"
    case user_ = "user"
    case operator_ = "operator"
    
    func roleToString() -> String {
        switch self {
        case .audit_:
            "Аудитор"
        case .operator_:
            "Оператор"
        case .user_:
            "Пользователь"
        default:
            "Неизвестная роль"
        }
    }
}

struct User: Codable {
    var name: String
    var surname: String
    var lastname: String?
    
    let email: String
    
    var role: UserRole
    
    var is_verified: Bool
    var passport: Document?
}
