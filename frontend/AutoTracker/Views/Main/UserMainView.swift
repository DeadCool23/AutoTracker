import UIKit

class UserMainView: UIView {
    var user: User
    var cars: [Car] = []
    
    let exitButton: UIButton = {
        let button = UIButton()
        button.setImage(UIImage(named: "ExitButton"), for: .normal)
        button.setTitleColor(.white, for: .normal)
        button.isEnabled = true
        return button
    }()
    
    private lazy var headerView: UIView = {
        let view = UIView()
        view.backgroundColor = UIColor(named: "FrameColor")
        
        view.layer.cornerRadius = bigCornerRadius
        view.layer.maskedCorners = [
            .layerMaxXMaxYCorner,
            .layerMinXMaxYCorner
        ]
        
        let textLabel = configureTitleLabel(
            text: "\(user.surname) \(user.name) \(user.lastname ?? "")"
        )
        let emailLabel = configureSubTitleLabel(text: "\(user.email)")
        let stack_view = UIStackView(arrangedSubviews: [textLabel, emailLabel])
        stack_view.axis = .vertical
        stack_view.spacing = 0
        stack_view.translatesAutoresizingMaskIntoConstraints = false
        exitButton.translatesAutoresizingMaskIntoConstraints = false
        
        view.addSubview(stack_view)
        view.addSubview(exitButton)
        NSLayoutConstraint.activate([
            exitButton.bottomAnchor.constraint(equalTo: view.bottomAnchor, constant: -stdAligment),
            exitButton.trailingAnchor.constraint(equalTo: view.trailingAnchor, constant: -bigAligment),
            stack_view.leadingAnchor.constraint(equalTo: view.leadingAnchor, constant: bigAligment),
            stack_view.centerYAnchor.constraint(equalTo: exitButton.centerYAnchor),
        ])
        
        return view
    }()
    
    private lazy var scrollView: UIScrollView = UIScrollView()
    
    var updateActions: () -> () = {}
    private lazy var infoLabel: UILabel = configureSubTitleLabel(text: "")
    lazy var carButtons: [UserRouteButton] = []
    
    private lazy var verifyButtonFrame = UIView()
    lazy var verifyButton: UIButton = configureButton(text: "Указать паспортные данные")
    
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
        setupInfoLabel()
        setupScrollView()
        if user.is_verified {
            setupVerifiedView()
        } else {
            setupUnverifiedView()
        }
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
    
    private func setupInfoLabel() {
        addSubview(infoLabel)
        
        NSLayoutConstraint.activate([
            infoLabel.centerXAnchor.constraint(equalTo: centerXAnchor),
            infoLabel.centerYAnchor.constraint(equalTo: centerYAnchor)
        ])
    }
    
    private func setupScrollView() {
        addSubview(scrollView)
        
        scrollView.isScrollEnabled = true
        scrollView.isUserInteractionEnabled = true
        scrollView.translatesAutoresizingMaskIntoConstraints = false
        
        NSLayoutConstraint.activate([
            scrollView.topAnchor.constraint(equalTo: headerView.bottomAnchor),
            scrollView.leadingAnchor.constraint(equalTo: leadingAnchor),
            scrollView.trailingAnchor.constraint(equalTo: trailingAnchor),
            scrollView.bottomAnchor.constraint(equalTo: bottomAnchor)
        ])
    }
}

extension UserMainView {
    func setupVerifiedView() {
        infoLabel.text = ""
        
        let response = SearchAPIManager.searchCar(
            passport: SearchByPassportRequest(
                passport: user.passport!
            )
        )
        
        let serverCode = response.1
        let status = response.0?.status
        
        if status == nil {
            infoLabel.text = CodeHandler.serverCodeToMessage(code: serverCode)
            infoLabel.textColor = UIColor(named: "ErrorTextColor")
        } else {
            let (msg, _) = CodeHandler.APICodeToMessage(code: status!.code)
            if status?.code != 0 {
                infoLabel.text = msg
                infoLabel.textColor = UIColor(named: "ErrorTextColor")
            } else {
                cars = response.0!.cars
                setupCars()
            }
        }
    }
    
    private func setupCars() {
        if cars.isEmpty {
            infoLabel.text = "У Вас пока нет автомобилей"
            infoLabel.textColor = UIColor(named: "SubTextColor")
        } else {
            carButtons = cars.map { UserRouteButton($0) }
            setupCarsButtons()
        }
    }
    
    private func setupCarsButtons() {
        var previousButton: UserRouteButton?
        
        for button in carButtons {
            scrollView.addSubview(button)
            button.translatesAutoresizingMaskIntoConstraints = false
            
            NSLayoutConstraint.activate([
                button.leadingAnchor.constraint(equalTo: leadingAnchor, constant: stdAligment),
                button.trailingAnchor.constraint(equalTo: trailingAnchor, constant: -stdAligment)
            ])
            
            if let prev = previousButton {
                button.topAnchor.constraint(equalTo: prev.bottomAnchor, constant: smallAligment).isActive = true
            } else {
                button.topAnchor.constraint(equalTo: scrollView.topAnchor, constant: stdAligment).isActive = true
            }
            
            previousButton = button
        }
        
        previousButton?.bottomAnchor.constraint(equalTo: scrollView.bottomAnchor).isActive = true
        self.updateActions()
    }
}

extension UserMainView {
    func unsetUnverifiedView() {
        infoLabel.text = ""
        verifyButtonFrame.removeFromSuperview()
    }
    
    func setupUnverifiedView() {
        infoLabel.text = "Паспортные данные отсутствуют"
        
        let frame = configureFrame(verifyButtonFrame, content: [verifyButton])
        addSubview(frame)
        
        NSLayoutConstraint.activate([
            frame.leadingAnchor.constraint(equalTo: leadingAnchor, constant: stdAligment),
            frame.trailingAnchor.constraint(equalTo: trailingAnchor, constant: -stdAligment),
            frame.bottomAnchor.constraint(equalTo: safeAreaLayoutGuide.bottomAnchor)
        ])
    }
}

extension UserMainView {
    private func configureTitleLabel(text: String) -> UILabel {
        let textLabel = UILabel()
        addSubview(textLabel)
        
        textLabel.text = text
        textLabel.textColor = UIColor(named: "MainTextColor")
        textLabel.font = UIFont.systemFont(ofSize: stdFontSize, weight: .bold)
        textLabel.translatesAutoresizingMaskIntoConstraints = false
        return textLabel
    }
    
    private func configureSubTitleLabel(text: String) -> UILabel {
        let textLabel = UILabel()
        addSubview(textLabel)
        
        textLabel.text = text
        textLabel.textColor = UIColor(named: "SubTextColor")
        textLabel.font = UIFont.systemFont(ofSize: subtitleFontSize, weight: .regular)
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
    
    private func configureButton(text: String) -> UIButton {
        let _button = UIButton(type: .system)
        _button.layer.cornerRadius = intoFrameCornerRadius
        _button.setTitle(text, for: .normal)
        _button.titleLabel?.font = UIFont.systemFont(ofSize: stdFontSize, weight: .semibold)
        _button.setTitleColor(UIColor(named: "DarkTextColor"), for: .normal)
        _button.backgroundColor = UIColor(named: "MainTextColor")
        _button.translatesAutoresizingMaskIntoConstraints = false
        NSLayoutConstraint.activate([
            _button.heightAnchor.constraint(equalToConstant: textFieldHeight)
        ])
        
        return _button
    }
}

extension UserMainView {
    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        self.endEditing(true)
    }
}
