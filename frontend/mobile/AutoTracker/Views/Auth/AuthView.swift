import UIKit

class AuthView: UIView {
    private let authFrameToHeaderAligment: CGFloat = 150
    lazy var errorMsgLabel = {
        let msg = UILabel()
        msg.numberOfLines = 0
        msg.textAlignment = .center
        msg.textColor = UIColor(named: "ErrorTextColor")
        msg.font = UIFont.systemFont(ofSize: errorFontSize, weight: .medium)
        
        msg.translatesAutoresizingMaskIntoConstraints = false
        
        return msg
    }()
    private lazy var headerView: UIView = {
        let view = UIView()
        view.backgroundColor = UIColor(named: "FrameColor")
        
        view.layer.cornerRadius = bigCornerRadius
        view.layer.maskedCorners = [
            .layerMaxXMaxYCorner,
            .layerMinXMaxYCorner
        ]
        
        let textLabel = UILabel()
        textLabel.text = "AUTO TRACKER"
        textLabel.textColor = UIColor(named: "MainTextColor")
        textLabel.font = UIFont.systemFont(ofSize: titleFontSize, weight: .bold)
        textLabel.translatesAutoresizingMaskIntoConstraints = false
        
        view.addSubview(textLabel)
        NSLayoutConstraint.activate([
            textLabel.centerXAnchor.constraint(equalTo: view.centerXAnchor),
            textLabel.bottomAnchor.constraint(equalTo: view.bottomAnchor, constant: -stdAligment)
        ])
        
        return view
    }()
    
    private lazy var authFrame: UIView = UIView()
    
    lazy var loginTextField: UITextField = configureTextField(placeholderText: "Логин")
    lazy var passwordTextField: UITextField = {
        let tf = configureTextField(placeholderText: "Пароль")
        tf.textContentType = .password
        tf.isSecureTextEntry = true
        
        return tf
    }()
    
    lazy var authButton = configureButton(text: "Войти")
    lazy var regButton = configureButton(text: "Регистрация")
    
    init() {
        super.init(frame: .zero)
        setupView()
    }
    
    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
    
    private func setupView() {
        backgroundColor = UIColor(named: "AccentColor")
        
        setupHeader()
        setupAuth()
        setupReg()
    }
    
    private func setupHeader() {
        addSubview(headerView)
        headerView.translatesAutoresizingMaskIntoConstraints = false
        
        NSLayoutConstraint.activate([
            headerView.topAnchor.constraint(equalTo: self.topAnchor),
            headerView.leadingAnchor.constraint(equalTo: self.leadingAnchor),
            headerView.trailingAnchor.constraint(equalTo: self.trailingAnchor),
            headerView.heightAnchor.constraint(equalToConstant: headerHeight)
        ])
    }
    
    private func setupAuth() {
        let authTitle = configureTitleLabel(text: "Авторизация")
        loginTextField.autocapitalizationType = .none
        loginTextField.keyboardType = .emailAddress
        let frame = configureFrame(authFrame, content: [
            authTitle,
            loginTextField,
            passwordTextField,
            errorMsgLabel,
            authButton
        ])
        
        addSubview(frame)
        NSLayoutConstraint.activate([
            frame.topAnchor.constraint(equalTo: headerView.bottomAnchor, constant: authFrameToHeaderAligment),
            frame.leadingAnchor.constraint(equalTo: leadingAnchor, constant: stdAligment),
            frame.trailingAnchor.constraint(equalTo: trailingAnchor, constant: -stdAligment),
        ])
    }
    
    private func setupReg() {
        let frame = configureFrame(UIView(), content: [
            regButton
        ])
        
        addSubview(frame)
        NSLayoutConstraint.activate([
            frame.topAnchor.constraint(equalTo: authFrame.bottomAnchor, constant: smallAligment),
            frame.leadingAnchor.constraint(equalTo: leadingAnchor, constant: stdAligment),
            frame.trailingAnchor.constraint(equalTo: trailingAnchor, constant: -stdAligment),
        ])
    }
}

extension AuthView {
    private func configureTitleLabel(text: String) -> UILabel {
        let textLabel = UILabel()
        addSubview(textLabel)
        
        textLabel.text = text
        textLabel.textColor = UIColor(named: "MainTextColor")
        textLabel.font = UIFont.systemFont(ofSize: titleFontSize, weight: .bold)
        textLabel.translatesAutoresizingMaskIntoConstraints = false
        return textLabel
    }
    
    private func configureFrame(_ frame: UIView, spacing: CGFloat = smallAligment, distToFrame: CGFloat = stdAligment, content: [UIView]) -> UIView {
        frame.layer.cornerRadius = frameCornerRadius
        frame.backgroundColor = UIColor(named: "FrameColor")
        frame.translatesAutoresizingMaskIntoConstraints = false
        
        let stackView = UIStackView(arrangedSubviews: content)
        
        stackView.spacing = spacing
        stackView.axis = .vertical
        stackView.alignment = .fill
        stackView.isUserInteractionEnabled = true
        stackView.translatesAutoresizingMaskIntoConstraints = false
        
        frame.addSubview(stackView)
        
        NSLayoutConstraint.activate([
            stackView.rightAnchor.constraint(equalTo: frame.rightAnchor, constant: -distToFrame),
            stackView.leftAnchor.constraint(equalTo: frame.leftAnchor, constant: distToFrame),
            stackView.topAnchor.constraint(equalTo: frame.topAnchor, constant: distToFrame),
            stackView.bottomAnchor.constraint(equalTo: frame.bottomAnchor, constant: -distToFrame)
        ])
        
        return frame
    }
    
    private func configureTextField(placeholderText: String) -> UITextField {
        let textField = UITextField()
        textField.backgroundColor = .clear
        textField.textColor = UIColor(named: "MainTextColor")!
        textField.font = UIFont.boldSystemFont(ofSize: stdFontSize)
        
        textField.attributedPlaceholder = NSAttributedString(string: placeholderText, attributes: [
            .foregroundColor: UIColor(named: "SubTextColor")!,
            .font: UIFont.systemFont(ofSize: stdFontSize, weight: .regular)
        ])
        textField.textAlignment = .left
        
        textField.leftView = UIView(frame: CGRect(x: 0, y: 0, width: smallAligment, height: textField.frame.height))
        textField.rightView = UIView(frame: CGRect(x: 0, y: 0, width: smallAligment, height: textField.frame.height))
        textField.rightViewMode = .always
        textField.leftViewMode = .always
        
        textField.layer.cornerRadius = intoFrameCornerRadius
        textField.layer.borderWidth = smallBorderWidth
        textField.layer.borderColor = UIColor(named: "MainTextColor")!.cgColor
        textField.translatesAutoresizingMaskIntoConstraints = false
        
        NSLayoutConstraint.activate([
            textField.heightAnchor.constraint(equalToConstant: textFieldHeight)
        ])
        
        return textField
    }
    
    private func configureButton(text: String) -> UIButton {
        let _button = UIButton(type: .system)
        _button.layer.cornerRadius = intoFrameCornerRadius
        _button.setTitle(text, for: .normal)
        _button.titleLabel?.font = UIFont.systemFont(ofSize: stdFontSize, weight: .bold)
        _button.setTitleColor(UIColor(named: "DarkTextColor"), for: .normal)
        _button.backgroundColor = UIColor(named: "MainTextColor")
        _button.translatesAutoresizingMaskIntoConstraints = false
        NSLayoutConstraint.activate([
            _button.heightAnchor.constraint(equalToConstant: textFieldHeight)
        ])
        
        return _button
    }
}

extension AuthView {
    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        self.endEditing(true)
    }
}
