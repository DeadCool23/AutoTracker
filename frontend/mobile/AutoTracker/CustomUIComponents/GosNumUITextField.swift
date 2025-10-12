import UIKit

class GosNumUITextField: UITextField, UITextFieldDelegate {
    let isMask: Bool
    
    init(isMask: Bool) {
        self.isMask = isMask
        super.init(frame: .zero)
        setupTextField()
    }
    
    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
    
    private func setupTextField() {
        tintColor = .black
        backgroundColor = UIColor(named: "MainTextColor")
        textColor = UIColor(named: "DarkTextColor")!
        font = UIFont.boldSystemFont(ofSize: stdFontSize)
        autocapitalizationType = .allCharacters
        
        layer.cornerRadius = smallCornerRadius
        layer.borderWidth = smallBorderWidth
        layer.borderColor = UIColor(named: "DarkTextColor")!.cgColor
        
        textAlignment = .center
        
        let paddingView = UIView(frame: CGRect(x: 0, y: 0, width: smallAligment, height: smallAligment))
        leftView = paddingView
        rightView = paddingView
        leftViewMode = .always
        rightViewMode = .always
        
        delegate = self
    }
}

extension GosNumUITextField {
    func textField(_ textField: UITextField, shouldChangeCharactersIn range: NSRange, replacementString string: String) -> Bool {
        let currentText = textField.text ?? ""
        let newText = (currentText as NSString).replacingCharacters(in: range, with: string)
        
        if newText.isEmpty { return true }
        
        let pattern: String
        if isMask {
            pattern = "^([АВЕКМНОРСТУХ*]{0,1}|[АВЕКМНОРСТУХ*][\\d*]{0,3}|[АВЕКМНОРСТУХ*][\\d*]{3}[АВЕКМНОРСТУХ*]{0,2}|[АВЕКМНОРСТУХ*][\\d*]{3}[АВЕКМНОРСТУХ*]{2}(\\d{0,3}|\\*{0,1}))$"
        } else {
            pattern = "^([АВЕКМНОРСТУХ]{0,1}|[АВЕКМНОРСТУХ]\\d{0,3}|[АВЕКМНОРСТУХ]\\d{3}[АВЕКМНОРСТУХ]{0,2}|[АВЕКМНОРСТУХ]\\d{3}[АВЕКМНОРСТУХ]{2}\\d{0,3})$"
        }
        
        let regex = try! NSRegularExpression(pattern: pattern, options: .caseInsensitive)
        let range = NSRange(location: 0, length: newText.count)
        return regex.firstMatch(in: newText, options: [], range: range) != nil
    }
    
    override func textRect(forBounds bounds: CGRect) -> CGRect {
        return bounds.inset(by: UIEdgeInsets(top: smallAligment, left: smallAligment, bottom: smallAligment, right: smallAligment))
    }
    
    override func editingRect(forBounds bounds: CGRect) -> CGRect {
        return bounds.inset(by: UIEdgeInsets(top: smallAligment, left: smallAligment, bottom: smallAligment, right: smallAligment))
    }
    
    override func placeholderRect(forBounds bounds: CGRect) -> CGRect {
        return bounds.inset(by: UIEdgeInsets(top: smallAligment, left: smallAligment, bottom: smallAligment, right: smallAligment))
    }
}
