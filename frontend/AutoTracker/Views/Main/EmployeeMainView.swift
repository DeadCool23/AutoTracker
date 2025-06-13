import UIKit

class EmployeeMainView: UIView {
    let user: User
    let exitButton: UIButton = {
        let button = UIButton()
        button.setImage(UIImage(named: "ExitButton"), for: .normal)
        button.isEnabled = true
        return button
    }()
    
    private lazy var headerView: UIView = {
        let view = UIView()
        view.backgroundColor = UIColor(named: "FrameColor")
        
        view.layer.cornerRadius = bigCornerRadius
        view.layer.maskedCorners = [.layerMaxXMaxYCorner, .layerMinXMaxYCorner]
        
        let textLabel = configureTitleLabel(
            text: getTitleByRole()
        )
        exitButton.translatesAutoresizingMaskIntoConstraints = false
        let errorLabel = errors["all"]!
        
        view.addSubview(textLabel)
        view.addSubview(exitButton)
        view.addSubview(errorLabel)
        NSLayoutConstraint.activate([
            exitButton.bottomAnchor.constraint(equalTo: view.bottomAnchor, constant: -stdAligment),
            exitButton.trailingAnchor.constraint(equalTo: view.trailingAnchor, constant: -bigAligment),
            
            textLabel.centerXAnchor.constraint(equalTo: view.centerXAnchor),
            textLabel.centerYAnchor.constraint(equalTo: exitButton.centerYAnchor),
            
            errorLabel.centerXAnchor.constraint(equalTo: view.centerXAnchor),
            errorLabel.topAnchor.constraint(equalTo: textLabel.bottomAnchor)
        ])
        
        return view
    }()
    
    lazy var scrollView = UIScrollView()
    
    lazy var FIOFrame = UIView()
    lazy var nameTextField = UITextField()
    lazy var surnameTextField = UITextField()
    lazy var lastnameTextField = UITextField()
    lazy var fioSearchButton = configureButton(text: "Поиск")
    
    lazy var passportFrame = UIView()
    lazy var passportSerialTextField = UITextField()
    lazy var passportNumTextField = UITextField()
    lazy var passportSearchButton = configureButton(text: "Поиск")
    
    lazy var dateFrame = UIView()
    lazy var dateTextField = UITextField()
    let dateSearchControl = UISwitch()
    let dateFormatter: DateFormatter = {
        let formatter = DateFormatter()
        formatter.dateFormat = "dd.MM.yyyy"
        return formatter
    }()
    lazy var dateSearchButton = configureButton(text: "Поиск")
    lazy var datePicker = UIDatePicker()
    
    lazy var gosNumFrame = UIView()
    lazy var gosNumTextField = GosNumUITextField(isMask: true)
    lazy var gosNumSearchButton = configureButton(text: "Поиск")
    
    lazy var filterFrame = UIView()
    lazy var filterSearchButton = configureButton(text: "Поиск по фильтрам", isStdHeight: false)
    
    lazy var errors: Dictionary<String, UILabel> = [
        "fio" : configureErrorLabel(),
        "date" : configureErrorLabel(),
        "passport" : configureErrorLabel(),
        "gos_num_mask": configureErrorLabel(),
        "all": configureErrorLabel()
    ]
    
    init(user: User) {
        self.user = user
        
        super.init(frame: .zero)
        setupView()
    }
    
    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
    
    private func setupView() {
        backgroundColor = UIColor(named: "AccentColor")
        
        setupHeader()
        setupScrollView(role: user.role)
        setupSearchButton()
    }
    
    private func setupHeader() {
        addSubview(headerView)
        headerView.translatesAutoresizingMaskIntoConstraints = false
        
        NSLayoutConstraint.activate([
            headerView.topAnchor.constraint(equalTo: self.topAnchor),
            headerView.leadingAnchor.constraint(equalTo: self.leadingAnchor),
            headerView.trailingAnchor.constraint(equalTo: self.trailingAnchor),
            headerView.heightAnchor.constraint(equalToConstant: mainHeaderHeight)
        ])
    }
    
    private func setupScrollView(role: UserRole) {
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
            scrollView.bottomAnchor.constraint(equalTo: bottomAnchor)
        ])
        
        setupFIO()
        setupPassport()
        var topFrame = passportFrame
        if user.role == .audit_ {
            setupDate()
            topFrame = dateFrame
        }
        setupGosNum(topFrame: topFrame)
    }
    
    private func setupSearchButton() {
        filterFrame.backgroundColor = UIColor(named: "FrameColor")
        filterFrame.layer.cornerRadius = bigCornerRadius
        filterFrame.layer.maskedCorners = [
            .layerMaxXMinYCorner,
            .layerMinXMinYCorner
        ]
        
        filterSearchButton.layer.cornerRadius = bigCornerRadius - stdAligment
        filterFrame.translatesAutoresizingMaskIntoConstraints = false
        
        filterFrame.addSubview(filterSearchButton)
        NSLayoutConstraint.activate([
            filterSearchButton.heightAnchor.constraint(equalToConstant: textFieldHeight + 10),
            filterSearchButton.leadingAnchor.constraint(equalTo: filterFrame.leadingAnchor, constant: stdAligment),
            filterSearchButton.trailingAnchor.constraint(equalTo: filterFrame.trailingAnchor, constant: -stdAligment),
            filterSearchButton.topAnchor.constraint(equalTo: filterFrame.topAnchor, constant: stdAligment),
            filterSearchButton.bottomAnchor.constraint(equalTo: filterFrame.bottomAnchor, constant: -2 * stdAligment),
        ])
        
        addSubview(filterFrame)
        NSLayoutConstraint.activate([
            filterFrame.bottomAnchor.constraint(equalTo: bottomAnchor),
            filterFrame.leadingAnchor.constraint(equalTo: leadingAnchor),
            filterFrame.trailingAnchor.constraint(equalTo: trailingAnchor),
        ])
    }
    
    private func setupFIO() {
        let FIOTitle = configureTitleLabel(text: "Поиск по ФИО")
        nameTextField = configureTextField(placeholderText: "Имя")
        surnameTextField = configureTextField(placeholderText: "Фамилия")
        lastnameTextField = configureTextField(placeholderText: "Отчество")
        let frame = configureFrame(FIOFrame, content: [
            FIOTitle,
            nameTextField,
            surnameTextField,
            lastnameTextField,
            errors["fio"]!,
            fioSearchButton
        ])
        
        scrollView.addSubview(frame)
        NSLayoutConstraint.activate([
            frame.topAnchor.constraint(equalTo: scrollView.topAnchor, constant: stdAligment),
            frame.leadingAnchor.constraint(equalTo: leadingAnchor, constant: stdAligment),
            frame.trailingAnchor.constraint(equalTo: trailingAnchor, constant: -stdAligment),
        ])
    }
    
    private func setupPassport() {
        let passportTitle = configureTitleLabel(text: "Поиск по паспортным данным")
        passportSerialTextField = configureTextField(placeholderText: "1111")
        passportNumTextField = configureTextField(placeholderText: "111111")
        
        passportNumTextField.layer.maskedCorners = [.layerMaxXMinYCorner, .layerMaxXMaxYCorner]
        passportNumTextField.layer.cornerRadius = 8
        passportNumTextField.layer.masksToBounds = true
        passportNumTextField.textAlignment = .center
        passportNumTextField.keyboardType = .numberPad
        
        passportSerialTextField.layer.maskedCorners = [.layerMinXMinYCorner, .layerMinXMaxYCorner]
        passportSerialTextField.layer.cornerRadius = 8
        passportSerialTextField.layer.masksToBounds = true
        passportSerialTextField.textAlignment = .center
        passportSerialTextField.keyboardType = .numberPad
        
        let passportDataFrame = UIView()
        passportDataFrame.translatesAutoresizingMaskIntoConstraints = false
        
        passportDataFrame.addSubview(passportSerialTextField)
        passportDataFrame.addSubview(passportNumTextField)
        NSLayoutConstraint.activate([
            passportSerialTextField.leadingAnchor.constraint(equalTo: passportDataFrame.leadingAnchor),
            passportSerialTextField.topAnchor.constraint(equalTo: passportDataFrame.topAnchor),
            passportSerialTextField.bottomAnchor.constraint(equalTo: passportDataFrame.bottomAnchor),
            passportSerialTextField.widthAnchor.constraint(equalToConstant: 140),
            
            passportNumTextField.leadingAnchor.constraint(equalTo: passportSerialTextField.trailingAnchor),
            passportNumTextField.trailingAnchor.constraint(equalTo: passportDataFrame.trailingAnchor),
            passportNumTextField.topAnchor.constraint(equalTo: passportDataFrame.topAnchor),
            passportNumTextField.bottomAnchor.constraint(equalTo: passportDataFrame.bottomAnchor),
        ])
        
        let frame = configureFrame(passportFrame, content: [
            passportTitle,
            passportDataFrame,
            errors["passport"]!,
            passportSearchButton
        ])
        
        scrollView.addSubview(frame)
        NSLayoutConstraint.activate([
            frame.topAnchor.constraint(equalTo: FIOFrame.bottomAnchor, constant: smallAligment),
            frame.leadingAnchor.constraint(equalTo: leadingAnchor, constant: stdAligment),
            frame.trailingAnchor.constraint(equalTo: trailingAnchor, constant: -stdAligment),
        ])
    }
    
    private func setupDate() {
        let dateTitle = configureTitleLabel(text: "Поиск по дате")
        dateTextField = configureTextField(placeholderText: "dd.MM.yyyy")
        dateSearchControl.addTarget(self, action: #selector(textFieldChanged), for: .valueChanged)
        let stackView = UIStackView(arrangedSubviews: [dateTextField, dateSearchControl])
        stackView.axis = .horizontal
        stackView.spacing = 10
        stackView.alignment = .center
        stackView.translatesAutoresizingMaskIntoConstraints = false
        
        let frame = configureFrame(dateFrame, content: [
            dateTitle,
            stackView,
            errors["date"]!,
            dateSearchButton
        ])
        
        dateTextField.text = dateFormatter.string(from: Date())
        datePicker = UIDatePicker()
        datePicker.datePickerMode = .date
        datePicker.preferredDatePickerStyle = .wheels
        datePicker.maximumDate = Date()
        datePicker.locale = Locale(identifier: "ru_RU")
        dateTextField.inputView = datePicker
        
        dateSearchControl.isOn = true
        dateSearchControl.onTintColor = UIColor(named: "SubTextColor")
        dateSearchControl.translatesAutoresizingMaskIntoConstraints = false
        
        dateSearchButton.isEnabled = true
        dateSearchButton.backgroundColor = UIColor(named: "MainTextColor")
        
        filterSearchButton.isEnabled = true
        filterSearchButton.backgroundColor = UIColor(named: "MainTextColor")
        
        scrollView.addSubview(frame)
        NSLayoutConstraint.activate([
            frame.topAnchor.constraint(equalTo: passportFrame.bottomAnchor, constant: smallAligment),
            frame.leadingAnchor.constraint(equalTo: leadingAnchor, constant: stdAligment),
            frame.trailingAnchor.constraint(equalTo: trailingAnchor, constant: -stdAligment),
        ])
    }
    
    private func setupGosNum(topFrame: UIView) {
        let gosNumTitle = configureTitleLabel(text: "Поиск по гос.номеру")
        
        gosNumTextField.attributedPlaceholder = NSAttributedString(string: "Гос.номер", attributes: [
            .foregroundColor: UIColor(named: "DarkTextColor")!.withAlphaComponent(0.6),
            .font: UIFont.systemFont(ofSize: stdFontSize, weight: .bold)
        ])
        gosNumTextField.addTarget(self, action: #selector(textFieldChanged), for: .editingChanged)
        
        let view = UIView()
        view.backgroundColor = UIColor(named: "MainTextColor")
        view.layer.cornerRadius = smallCornerRadius
        view.translatesAutoresizingMaskIntoConstraints = false
        gosNumTextField.translatesAutoresizingMaskIntoConstraints = false
        gosNumTextField.layer.cornerRadius = smallCornerRadius - smallBorderWidth
        
        view.addSubview(gosNumTextField)
        NSLayoutConstraint.activate([
            view.heightAnchor.constraint(equalToConstant: textFieldHeight),
            
            gosNumTextField.topAnchor.constraint(equalTo: view.topAnchor, constant: smallBorderWidth),
            gosNumTextField.bottomAnchor.constraint(equalTo: view.bottomAnchor, constant: -smallBorderWidth),
            gosNumTextField.leadingAnchor.constraint(equalTo: view.leadingAnchor, constant: smallBorderWidth),
            gosNumTextField.trailingAnchor.constraint(equalTo: view.trailingAnchor, constant: -smallBorderWidth),
        ])
        
        let frame = configureFrame(gosNumFrame, content: [
            gosNumTitle,
            view,
            errors["gos_num_mask"]!,
            gosNumSearchButton
        ])
        
        scrollView.addSubview(frame)
        NSLayoutConstraint.activate([
            frame.topAnchor.constraint(equalTo: topFrame.bottomAnchor, constant: smallAligment),
            frame.leadingAnchor.constraint(equalTo: leadingAnchor, constant: stdAligment),
            frame.trailingAnchor.constraint(equalTo: trailingAnchor, constant: -stdAligment),
            frame.bottomAnchor.constraint(
                equalTo: scrollView.bottomAnchor,
                constant: -(keyboardHeight + stdAligment)
            )
        ])
    }
}

extension EmployeeMainView {
    private func configureTitleLabel(text: String) -> UILabel {
        let textLabel = UILabel()
        addSubview(textLabel)
        
        textLabel.text = text
        textLabel.textColor = UIColor(named: "MainTextColor")
        textLabel.font = UIFont.systemFont(ofSize: titleFontSize, weight: .bold)
        textLabel.translatesAutoresizingMaskIntoConstraints = false
        return textLabel
    }
    
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
    
    private func configureButton(text: String, isStdHeight: Bool = true) -> UIButton {
        let _button = UIButton(type: .system)
        _button.layer.cornerRadius = intoFrameCornerRadius
        _button.setTitle(text, for: .normal)
        _button.titleLabel?.font = UIFont.systemFont(ofSize: stdFontSize, weight: .bold)
        _button.setTitleColor(UIColor(named: "DarkTextColor"), for: .normal)
        _button.backgroundColor = UIColor(named: "SubTextColor")
        _button.isEnabled = false
        _button.translatesAutoresizingMaskIntoConstraints = false
        if isStdHeight {
            NSLayoutConstraint.activate([
                _button.heightAnchor.constraint(equalToConstant: textFieldHeight)
            ])
        }
        
        return _button
    }
}

extension EmployeeMainView {
    private func getTitleByRole() -> String {
        switch user.role {
        case .operator_:
            return "Поиск автомобилей"
        case .audit_:
            return "Поиск отслеживаний"
        default:
            return "Поиск поисков"
        }
    }
}

extension EmployeeMainView {
    func updateButtons() {
        let isValidFIO =
            !nameTextField.text!.isEmpty ||
            !surnameTextField.text!.isEmpty ||
            !lastnameTextField.text!.isEmpty
        
        if isValidFIO {
            fioSearchButton.backgroundColor = UIColor(named: "MainTextColor")
        } else {
            fioSearchButton.backgroundColor = UIColor(named: "SubTextColor")
        }
        fioSearchButton.isEnabled = isValidFIO
        
        let isValidPassport =
            !passportNumTextField.text!.isEmpty &&
            !passportSerialTextField.text!.isEmpty
        
        if isValidPassport {
            passportSearchButton.backgroundColor = UIColor(named: "MainTextColor")
        } else {
            passportSearchButton.backgroundColor = UIColor(named: "SubTextColor")
        }
        passportSearchButton.isEnabled = isValidPassport
        
        let isValidGosNum = Validator.isValidGosNumMask(for: gosNumTextField.text ?? "")
        
        if isValidGosNum {
            gosNumSearchButton.backgroundColor = UIColor(named: "MainTextColor")
        } else {
            gosNumSearchButton.backgroundColor = UIColor(named: "SubTextColor")
        }
        gosNumSearchButton.isEnabled = isValidGosNum
        
        if isValidFIO || isValidPassport || isValidGosNum || dateSearchControl.isOn {
            filterSearchButton.isEnabled = true
            filterSearchButton.backgroundColor = UIColor(named: "MainTextColor")
        } else {
            filterSearchButton.isEnabled = false
            filterSearchButton.backgroundColor = UIColor(named: "SubTextColor")
        }
    }
    
    @objc
    private func textFieldChanged() {
        updateButtons()
    }
    
    @objc
    private func didTapScreen() {
        nameTextField.resignFirstResponder()
        surnameTextField.resignFirstResponder()
        lastnameTextField.resignFirstResponder()
        
        passportNumTextField.resignFirstResponder()
        passportSerialTextField.resignFirstResponder()
        
        dateTextField.resignFirstResponder()
        
        gosNumTextField.resignFirstResponder()
    }
    
    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        self.endEditing(true)
    }
}
