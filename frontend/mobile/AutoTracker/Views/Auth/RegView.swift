import UIKit

class RegView: UIView {
    private lazy var headerView: UIView = {
        let view = UIView()
        view.backgroundColor = UIColor(named: "FrameColor")
        
        view.layer.cornerRadius = bigCornerRadius
        view.layer.maskedCorners = [
            .layerMaxXMaxYCorner,
            .layerMinXMaxYCorner
        ]
        
        let textLabel = UILabel()
        textLabel.text = "Регистрация"
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
    
    lazy var scrollView = UIScrollView()
    
    lazy var FIOFrame = UIView()
    lazy var nameTextField = UITextField()
    lazy var surnameTextField = UITextField()
    lazy var lastnameTextField = UITextField()
    
    lazy var emailFrame = UIView()
    lazy var emailTextField = UITextField()
    
    lazy var pswdsFrame = UIView()
    lazy var pswdTextField = UITextField()
    lazy var repPswdTextField = UITextField()
    
    private lazy var underView = UIView()
    lazy var regButton = configureButton(text: "Зарегестрироваться")
    
    lazy var errors: Dictionary<String, UILabel> = [
        "fio" : configureErrorLabel(),
        "email" : configureErrorLabel(),
        "pswd" : configureErrorLabel(),
        "all": configureErrorLabel()
    ]
    
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
        setupRegButton()
        setupScrollView()
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
    
    private func setupScrollView() {
        addSubview(scrollView)
        
        scrollView.isScrollEnabled = true
        
        scrollView.isUserInteractionEnabled = true
        
        let tapGesture = UITapGestureRecognizer(target: self, action: #selector(didTapScreen))
        scrollView.addGestureRecognizer(tapGesture)
        
        scrollView.translatesAutoresizingMaskIntoConstraints = false
        
        NSLayoutConstraint.activate([
            scrollView.topAnchor.constraint(equalTo: headerView.bottomAnchor),
            scrollView.leadingAnchor.constraint(equalTo: leadingAnchor),
            scrollView.trailingAnchor.constraint(equalTo: trailingAnchor),
            scrollView.bottomAnchor.constraint(equalTo: underView.topAnchor)
        ])
        
        setupFIO()
        setupEmail()
        setupPswds()
    }
    
    private func setupFIO() {
        let FIOTitle = configureTitleLabel(text: "ФИО")
        nameTextField = configureTextField(placeholderText: "Имя")
        surnameTextField = configureTextField(placeholderText: "Фамилия")
        lastnameTextField = configureTextField(placeholderText: "Отчество(Необязательно)")
        let frame = configureFrame(FIOFrame, content: [
            FIOTitle,
            nameTextField,
            surnameTextField,
            lastnameTextField,
            errors["fio"]!
        ])
        
        scrollView.addSubview(frame)
        NSLayoutConstraint.activate([
            frame.topAnchor.constraint(equalTo: scrollView.topAnchor, constant: stdAligment),
            frame.leadingAnchor.constraint(equalTo: leadingAnchor, constant: stdAligment),
            frame.trailingAnchor.constraint(equalTo: trailingAnchor, constant: -stdAligment),
        ])
    }
    
    private func setupEmail() {
        let emailTitle = configureTitleLabel(text: "Логин")
        emailTextField = configureTextField(placeholderText: "example@example.com")
        emailTextField.autocapitalizationType = .none
        emailTextField.keyboardType = .emailAddress
        let frame = configureFrame(emailFrame, content: [
            emailTitle,
            emailTextField,
            errors["email"]!
        ])
        
        scrollView.addSubview(frame)
        NSLayoutConstraint.activate([
            frame.topAnchor.constraint(equalTo: FIOFrame.bottomAnchor, constant: smallAligment),
            frame.leadingAnchor.constraint(equalTo: leadingAnchor, constant: stdAligment),
            frame.trailingAnchor.constraint(equalTo: trailingAnchor, constant: -stdAligment),
        ])
    }
    
    private func setupPswds() {
        let pswdsTitle = configureTitleLabel(text: "Пароль")
        
        pswdTextField = configureTextField(placeholderText: "Пароль")
        pswdTextField.textContentType = .password
        pswdTextField.isSecureTextEntry = true
        
        repPswdTextField = configureTextField(placeholderText: "Подтверждение пароля")
        repPswdTextField.textContentType = .password
        repPswdTextField.isSecureTextEntry = true
        
        let frame = configureFrame(pswdsFrame, content: [
            pswdsTitle,
            pswdTextField,
            repPswdTextField,
            errors["pswd"]!
        ])
        
        scrollView.addSubview(frame)
        NSLayoutConstraint.activate([
            frame.topAnchor.constraint(equalTo: emailFrame.bottomAnchor, constant: smallAligment),
            frame.leadingAnchor.constraint(equalTo: leadingAnchor, constant: stdAligment),
            frame.trailingAnchor.constraint(equalTo: trailingAnchor, constant: -stdAligment),
            frame.bottomAnchor.constraint(
                equalTo: scrollView.bottomAnchor,
                constant: -(keyboardHeight + stdAligment)
            )
        ])
    }
    
    private func setupRegButton() {
        let frame = configureFrame(underView, content: [
            regButton,
            errors["all"]!
        ])
        frame.translatesAutoresizingMaskIntoConstraints = false
        
        addSubview(frame)
        NSLayoutConstraint.activate([
            frame.bottomAnchor.constraint(equalTo: safeAreaLayoutGuide.bottomAnchor),
            frame.leadingAnchor.constraint(equalTo: leadingAnchor, constant: stdAligment),
            frame.trailingAnchor.constraint(equalTo: trailingAnchor, constant: -stdAligment),
        ])
    }
}

extension RegView {
    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        self.endEditing(true)
    }
    
    private func updateRegButton() {
        let isValid = !nameTextField.text!.isEmpty &&
        !surnameTextField.text!.isEmpty &&
        !emailTextField.text!.isEmpty &&
        !pswdTextField.text!.isEmpty &&
        !repPswdTextField.text!.isEmpty
        
        if isValid {
            regButton.backgroundColor = UIColor(named: "MainTextColor")
        } else {
            regButton.backgroundColor = UIColor(named: "SubTextColor")
        }
        regButton.isEnabled = isValid
    }
    
    @objc
    private func textFieldChanged() {
        updateRegButton()
    }
    
    @objc
    private func didTapScreen() {
        nameTextField.resignFirstResponder()
        surnameTextField.resignFirstResponder()
        lastnameTextField.resignFirstResponder()
        emailTextField.resignFirstResponder()
        pswdTextField.resignFirstResponder()
        repPswdTextField.resignFirstResponder()
    }
}

extension RegView {
    private func configureErrorLabel(text: String = "") -> UILabel {
        let msg = UILabel()
        
        msg.text = text
        msg.numberOfLines = 0
        msg.textAlignment = .center
        msg.textColor = UIColor(named: "ErrorTextColor")
        msg.font = UIFont.systemFont(ofSize: errorFontSize, weight: .medium)
        
        msg.translatesAutoresizingMaskIntoConstraints = false
        
        return msg
    }
    
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
        
        textField.addTarget(self, action: #selector(textFieldChanged), for: .editingChanged)
        
        return textField
    }
    
    private func configureButton(text: String) -> UIButton {
        let _button = UIButton(type: .system)
        _button.layer.cornerRadius = intoFrameCornerRadius
        _button.setTitle(text, for: .normal)
        _button.titleLabel?.font = UIFont.systemFont(ofSize: stdFontSize, weight: .bold)
        _button.setTitleColor(UIColor(named: "DarkTextColor"), for: .normal)
        _button.backgroundColor = UIColor(named: "SubTextColor")
        _button.isEnabled = false
        _button.translatesAutoresizingMaskIntoConstraints = false
        NSLayoutConstraint.activate([
            _button.heightAnchor.constraint(equalToConstant: textFieldHeight)
        ])
        
        return _button
    }
}
