class CodeHandler {
    static func APICodeToMessage(code: Int) -> (String, String) {
        switch code {
        case 0:
            return ("Ok", "")
            
        case 1001:
            return ("Неверный формат даты", "date")
        case 1002:
            return ("Неверный формат гос.номер", "gos_num")
        case 1003:
            return ("Неверный формат маски гос.номера", "gos_num_mask")
        
        
        case 2000:
            return ("Неверный логин или пароль", "login or pswd")
        
        case 2001:
            return ("Неверный формат почты", "email")
        case 3001:
            return ("Пользователь с данной почтой уже существует", "email")
        case 3002:
            return ("Пользователь с данной почтой не найден", "email")
        
        case 2002:
            return ("Пароль должен быть не менее 8 символов", "pswd")
        case 2003:
            return ("Пароли не совпадают", "pswd")
            
        case 2004:
            return ("Некорректные пасспортные данные", "passport")
        case 2005:
            return ("Пасспортные данные уже зарегистрированы", "passport")
        case 2006:
            return ("Неверный формат даты", "date")
        
        case 4002:
            return ("Камера не найдена", "camera")
        case 4003:
            return ("Средняя скорость не найдена", "speed")
        
        default:
            return ("UNKNOWN ERROR", "all")
        }
    }
    
    static func serverCodeToMessage(code: Int) -> String {
        switch code {
        case 200:
            return "Ok"
        case 404:
            return "Не найдено"
        case 422:
            return "Необрабатываемый объект"
        case 500:
            return "Внутренняя ошибка сервера"
        case 503:
            return "Сервер не запущен"
        default:
            return "UNKNOWN ERROR"
        }
    }
}
