import UIKit

class FindedView : UIView {
    let backButton: UIButton = {
        let button = UIButton()
        button.setImage(UIImage(named: "Arrow.back"), for: .normal)
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
            text: "Найденные"
        )
        backButton.translatesAutoresizingMaskIntoConstraints = false
        
        view.addSubview(textLabel)
        view.addSubview(backButton)
        NSLayoutConstraint.activate([
            backButton.bottomAnchor.constraint(equalTo: view.bottomAnchor, constant: -stdAligment),
            backButton.leadingAnchor.constraint(equalTo: view.leadingAnchor, constant: bigAligment),
            
            textLabel.centerXAnchor.constraint(equalTo: view.centerXAnchor),
            textLabel.centerYAnchor.constraint(equalTo: backButton.centerYAnchor)
        ])
        
        return view
    }()
    
    let cars: [Car]?
    let trackInfo: [TrackInfo]?
    
    var carButtons: [OperatorRouteButton] = []
    
    var updateActions: () -> () = {}
    private lazy var rolesControl = UISegmentedControl(items: ["Все", "Операторы", "Пользователи"])
    private lazy var noFindedLabel = UILabel()
    var trackInfoButtons: [TrackInfoButton] = []
    
    private lazy var scrollView: UIScrollView = UIScrollView()
    
    init(cars: [Car]) {
        self.cars = cars
        self.trackInfo = nil
        
        super.init(frame: .zero)
        setupView()
    }
    
    init(trackInfo: [TrackInfo]) {
        self.trackInfo = trackInfo
        self.cars = nil
        
        super.init(frame: .zero)
        setupView()
    }
    
    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
    
    private func setupView() {
        backgroundColor = UIColor(named: "AccentColor")
        
        setupHeader()
        setupScrollView()
        if cars != nil {
            setupCarsView()
        } else if trackInfo != nil {
            setupTrackInfoView()
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
    
    private func setupNoFinded() {
        noFindedLabel = configureSubTitleLabel(text: "Ничего не найдено")
        addSubview(noFindedLabel)
        NSLayoutConstraint.activate([
            noFindedLabel.centerXAnchor.constraint(equalTo: centerXAnchor),
            noFindedLabel.centerYAnchor.constraint(equalTo: centerYAnchor)
        ])
    }
}

// MARK: Finded Cars
extension FindedView {
    private func setupCarsView() {
        carButtons = cars!.map { OperatorRouteButton($0) }
        if carButtons.count == 0 {
            setupNoFinded()
        } else {
            setupCarButtons()
        }
    }
    
    private func setupCarButtons() {
        var previousButton: OperatorRouteButton?
        
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
    }
}

// MARK: Finded TrackInfo
extension FindedView {
    private func setupTrackInfoView() {
        trackInfoButtons = trackInfo!.map { TrackInfoButton($0) }
        setupRoleControl()
        updateButtonsForSelectedRole()
    }
    
    private func setupRoleControl() {
        rolesControl.selectedSegmentIndex = 0
        rolesControl.backgroundColor = UIColor(named: "FrameColor")
        rolesControl.selectedSegmentTintColor = UIColor(named: "SubTextColor")?.withAlphaComponent(0.6)
        
        let normalTextAttributes: [NSAttributedString.Key: Any] = [
            .foregroundColor: UIColor(named: "MainTextColor")!,
            .font: UIFont.systemFont(ofSize: subtitleFontSize, weight: .regular)
        ]
        
        let selectedTextAttributes: [NSAttributedString.Key: Any] = [
            .foregroundColor: UIColor(named: "MainTextColor")!,
            .font: UIFont.systemFont(ofSize: subtitleFontSize, weight: .bold)
        ]
        
        rolesControl.setTitleTextAttributes(normalTextAttributes, for: .normal)
        rolesControl.setTitleTextAttributes(selectedTextAttributes, for: .selected)
        
        rolesControl.addTarget(self, action: #selector(roleFilterChanged), for: .valueChanged)
        rolesControl.translatesAutoresizingMaskIntoConstraints = false
        
        scrollView.addSubview(rolesControl)
        
        NSLayoutConstraint.activate([
            rolesControl.topAnchor.constraint(equalTo: scrollView.topAnchor, constant: stdAligment),
            rolesControl.leadingAnchor.constraint(equalTo: leadingAnchor, constant: stdAligment),
            rolesControl.trailingAnchor.constraint(equalTo: trailingAnchor, constant: -stdAligment),
        ])
    }
    
    @objc private func roleFilterChanged() {
            updateButtonsForSelectedRole()
        }
    
    private func updateButtonsForSelectedRole() {
        if noFindedLabel.superview != nil {
            noFindedLabel.removeFromSuperview()
        }
        
        trackInfoButtons.forEach { $0.removeFromSuperview() }
        let allTrackInfoButtons = trackInfo?.map { TrackInfoButton($0) } ?? []
    
        switch rolesControl.selectedSegmentIndex {
        case 1:
            trackInfoButtons = allTrackInfoButtons.filter { $0.trackInfo.user.role == .operator_ }
        case 2:
            trackInfoButtons = allTrackInfoButtons.filter { $0.trackInfo.user.role == .user_ }
        default:
            trackInfoButtons = allTrackInfoButtons
        }
        
        if trackInfoButtons.isEmpty {
            setupNoFinded()
        } else {
            setupTrackInfoButtons()
        }
    }
    
    private func setupTrackInfoButtons() {
        var previousButton: TrackInfoButton?
        
        let formatter = DateFormatter()
        formatter.dateFormat = "HH:mm dd.MM.yyyy"

        trackInfoButtons.sort {
            guard let date1 = formatter.date(from: $0.trackInfo.track_time),
                  let date2 = formatter.date(from: $1.trackInfo.track_time) else {
                return false
            }
            return date1 > date2
        }
        
        for button in trackInfoButtons {
            scrollView.addSubview(button)
            button.translatesAutoresizingMaskIntoConstraints = false
            
            NSLayoutConstraint.activate([
                button.leadingAnchor.constraint(equalTo: leadingAnchor, constant: stdAligment),
                button.trailingAnchor.constraint(equalTo: trailingAnchor, constant: -stdAligment)
            ])
            
            if let prev = previousButton {
                button.topAnchor.constraint(equalTo: prev.bottomAnchor, constant: smallAligment).isActive = true
            } else {
                button.topAnchor.constraint(equalTo: rolesControl.bottomAnchor, constant: stdAligment).isActive = true
            }
            
            previousButton = button
        }
        
        previousButton?.bottomAnchor.constraint(equalTo: scrollView.bottomAnchor).isActive = true
        self.updateActions()
    }
}

// MARK: Configurators
extension FindedView {
    private func configureTitleLabel(text: String) -> UILabel {
        let textLabel = UILabel()
        addSubview(textLabel)
        
        textLabel.text = text
        textLabel.textColor = UIColor(named: "MainTextColor")
        textLabel.font = UIFont.systemFont(ofSize: titleFontSize, weight: .bold)
        textLabel.translatesAutoresizingMaskIntoConstraints = false
        return textLabel
    }
    
    private func configureSubTitleLabel(text: String) -> UILabel {
        let textLabel = UILabel()
        addSubview(textLabel)
        
        textLabel.text = text
        textLabel.textColor = UIColor(named: "SubTextColor")
        textLabel.font = UIFont.systemFont(ofSize: stdFontSize, weight: .regular)
        textLabel.translatesAutoresizingMaskIntoConstraints = false
        return textLabel
    }
}
