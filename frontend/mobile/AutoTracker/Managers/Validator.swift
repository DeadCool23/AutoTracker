import Foundation

class Validator {
    static func isValidEmail(for email: String) -> Bool {
        let email = email.trimmingCharacters(in: .whitespacesAndNewlines)
        let emailRegEx = "[A-Z0-9a-z._%+-]+@[A-Za-z0-9.-]+\\.{1}[A-Za-z]{2,64}"
        let emailPred = NSPredicate(format: "SELF MATCHES %@", emailRegEx)
        return emailPred.evaluate(with: email)
    }
    
    static func isValidDate(for birthday: String) -> Bool {
        let birthday = birthday.trimmingCharacters(in: .whitespacesAndNewlines)
        let birthdayRegEx = "[0-9]{2}\\.[0-9]{2}\\.[0-9]{4}"
        let birthdayPred = NSPredicate(format: "SELF MATCHES %@", birthdayRegEx)
        return birthdayPred.evaluate(with: birthday)
    }
    
    static let minPswdLen = 8
    static func isValidPswd(for pswd: String) -> Bool {
        let pswd = pswd.trimmingCharacters(in: .whitespacesAndNewlines)
        let pswdRegEx = ".{\(minPswdLen),}"
        let pswdPred = NSPredicate(format: "SELF MATCHES %@", pswdRegEx)
        return pswdPred.evaluate(with: pswd)
    }
    
    static func isValidGosNumMask(for gosNumMask: String) -> Bool {
        let gosNumMask = gosNumMask.trimmingCharacters(in: .whitespacesAndNewlines)
        let gosNumMaskRegEx = "^([АВЕКМНОРСТУХ*])(\\d|\\*){3}([АВЕКМНОРСТУХ*]{2})(\\d{2,3}|\\*{1})$"
        let gosNumMaskPred = NSPredicate(format: "SELF MATCHES %@", gosNumMaskRegEx)
        return gosNumMaskPred.evaluate(with: gosNumMask)
    }
    
    static func isValidPassport(for passport: Document) -> Bool {
        passport.number.count == 6 && passport.serial.count == 4
    }
    
    static func isValidGosNum(for gosNum: String) -> Bool {
        let gosNum = gosNum.trimmingCharacters(in: .whitespacesAndNewlines)
        let gosNumRegEx = "^[АВЕКМНОРСТУХ]\\d{3}[АВЕКМНОРСТУХ]{2}\\d{2,3}$"
        let gosNumPred = NSPredicate(format: "SELF MATCHES %@", gosNumRegEx)
        return gosNumPred.evaluate(with: gosNum)
    }
}
