import UIKit

class TrackInfoView: UIView {
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
        
        let textLabel = configureLabel(
            text: "Информация",
            color: UIColor(named: "MainTextColor")!,
            fontSize: titleFontSize,
            weight: .bold
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
    
    private var fioInfoFrame: UIView = UIView()
    private var carInfoFrame: UIView = UIView()
    private var dateInfoFrame: UIView = UIView()
    
    private let trackInfo: TrackInfo
    
    init(trackInfo: TrackInfo) {
        self.trackInfo = trackInfo
        
        super.init(frame: .zero)
        setupView()
    }
    
    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
    
    private func setupView() {
        backgroundColor = UIColor(named: "AccentColor")
        
        setupHeader()
        
        setupUserInfo()
        setupCarInfo()
        setupTrackInfo()
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
    private func setupUserInfo() {
        let userTitleLabel = configureLabel(
            text: "Информация о пользователе",
            color: UIColor(named: "MainTextColor")!,
            fontSize: stdFontSize,
            weight: .bold
        )
        userTitleLabel.textAlignment = .left
        
        let FIOInfoLabel = configureInfo(
            title: "ФИО",
            info: "\(trackInfo.user.surname) \(trackInfo.user.name)"
            + (trackInfo.user.lastname != nil ? " \(trackInfo.user.lastname!)" : "")
        )
        let passportInfoLabel = configureInfo(
            title: "Паспортные данные",
            info: trackInfo.user.passport == nil
                    ? "Пасспортные данные отсутствуют"
                    : "\(trackInfo.user.passport!.serial) \(trackInfo.user.passport!.number)"
        )
        let roleInfoLabel = configureInfo(
            title: "Роль",
            info: "\(trackInfo.user.role.roleToString())"
        )
        
        let stackView = UIStackView(arrangedSubviews: [
            FIOInfoLabel,
            passportInfoLabel,
            roleInfoLabel
        ])
        stackView.axis = .vertical
        stackView.spacing = 0
        
        fioInfoFrame = stackView
        
        addSubview(userTitleLabel)
        addSubview(stackView)
        
        stackView.translatesAutoresizingMaskIntoConstraints = false
        userTitleLabel.translatesAutoresizingMaskIntoConstraints = false
        
        NSLayoutConstraint.activate([
            userTitleLabel.topAnchor.constraint(equalTo: headerView.bottomAnchor, constant: stdAligment),
            userTitleLabel.leadingAnchor.constraint(equalTo: leadingAnchor, constant: stdAligment),
            userTitleLabel.trailingAnchor.constraint(equalTo: trailingAnchor, constant: -stdAligment),
            
            stackView.topAnchor.constraint(equalTo: userTitleLabel.bottomAnchor, constant: smallAligment),
            stackView.leadingAnchor.constraint(equalTo: leadingAnchor, constant: stdAligment),
            stackView.trailingAnchor.constraint(equalTo: trailingAnchor, constant: -stdAligment),
        ])
    }
    
    private func setupCarInfo() {
        let carTitleLabel = configureLabel(
            text: "Информация об автомобиле",
            color: UIColor(named: "MainTextColor")!,
            fontSize: stdFontSize,
            weight: .bold
        )
        carTitleLabel.textAlignment = .left
        
        let gosNumInfoLabel = configureInfo(
            title: "Гос.номер",
            info: "\(trackInfo.car.gos_num)"
        )
        let markInfoLabel = configureInfo(
            title: "Марка",
            info: "\(trackInfo.car.mark)"
        )
        let modelInfoLabel = configureInfo(
            title: "Модель",
            info: "\(trackInfo.car.model)"
        )
        let ownerInfoLabel = configureInfo(
            title: "ФИО Владельца",
            info: "\(trackInfo.car.owner_fio.surname) \(trackInfo.car.owner_fio.name)"
            + (trackInfo.car.owner_fio.lastname != nil ? " \(trackInfo.car.owner_fio.lastname!)" : "")
        )
        let stsInfoLabel = configureInfo(
            title: "СТС",
            info: "\(trackInfo.car.sts.serial) \(trackInfo.car.sts.number)"
        )
        let ptsInfoLabel = configureInfo(
            title: "ПТС",
            info: "\(trackInfo.car.pts.serial) \(trackInfo.car.pts.number)"
        )
        let vinInfoLabel = configureInfo(
            title: "VIN",
            info: "\(trackInfo.car.vin)"
        )
        
        let stackView = UIStackView(arrangedSubviews: [
            ownerInfoLabel,
            markInfoLabel,
            modelInfoLabel,
            gosNumInfoLabel,
            stsInfoLabel,
            ptsInfoLabel,
            vinInfoLabel
        ])
        stackView.axis = .vertical
        stackView.spacing = 0
        
        carInfoFrame = stackView
        
        addSubview(carTitleLabel)
        addSubview(stackView)
        
        stackView.translatesAutoresizingMaskIntoConstraints = false
        carTitleLabel.translatesAutoresizingMaskIntoConstraints = false
        
        NSLayoutConstraint.activate([
            carTitleLabel.topAnchor.constraint(equalTo: fioInfoFrame.bottomAnchor, constant: stdAligment),
            carTitleLabel.leadingAnchor.constraint(equalTo: leadingAnchor, constant: stdAligment),
            carTitleLabel.trailingAnchor.constraint(equalTo: trailingAnchor, constant: -stdAligment),
            
            stackView.topAnchor.constraint(equalTo: carTitleLabel.bottomAnchor, constant: smallAligment),
            stackView.leadingAnchor.constraint(equalTo: leadingAnchor, constant: stdAligment),
            stackView.trailingAnchor.constraint(equalTo: trailingAnchor, constant: -stdAligment),
        ])
    }
    
    private func setupTrackInfo() {
        let dateTitleLabel = configureLabel(
            text: "Информация об отслеживании",
            color: UIColor(named: "MainTextColor")!,
            fontSize: stdFontSize,
            weight: .bold
        )
        dateTitleLabel.textAlignment = .left
        
        let trackInfoLabel = configureInfo(
            title: "Время отслеживания",
            info: "\(trackInfo.track_time)"
        )
        let routeInfoLabel = configureInfo(
            title: "Дата маршрута",
            info: "\(trackInfo.route_date)"
        )
        
        let stackView = UIStackView(arrangedSubviews: [
            trackInfoLabel,
            routeInfoLabel
        ])
        stackView.axis = .vertical
        stackView.spacing = 0
        
        dateInfoFrame = stackView
        
        addSubview(dateTitleLabel)
        addSubview(stackView)
        
        stackView.translatesAutoresizingMaskIntoConstraints = false
        dateTitleLabel.translatesAutoresizingMaskIntoConstraints = false
        
        NSLayoutConstraint.activate([
            dateTitleLabel.topAnchor.constraint(equalTo: carInfoFrame.bottomAnchor, constant: stdAligment),
            dateTitleLabel.leadingAnchor.constraint(equalTo: leadingAnchor, constant: stdAligment),
            dateTitleLabel.trailingAnchor.constraint(equalTo: trailingAnchor, constant: -stdAligment),
            
            stackView.topAnchor.constraint(equalTo: dateTitleLabel.bottomAnchor, constant: smallAligment),
            stackView.leadingAnchor.constraint(equalTo: leadingAnchor, constant: stdAligment),
            stackView.trailingAnchor.constraint(equalTo: trailingAnchor, constant: -stdAligment),
        ])
    }
}

extension TrackInfoView {
    
}

extension TrackInfoView {
    private func configureLabel(text: String, color: UIColor, fontSize: CGFloat, weight: UIFont.Weight) -> UILabel {
        let textLabel = UILabel()
        
        textLabel.text = text
        textLabel.textColor = color
        textLabel.font = UIFont.systemFont(ofSize: fontSize, weight: weight)
        textLabel.translatesAutoresizingMaskIntoConstraints = false
        return textLabel
    }
    
    private func configureInfo(title: String, info: String) -> UIView {
        let titleLabel = configureLabel(
            text: "\(title):",
            color: UIColor(named: "MainTextColor")!,
            fontSize: subtitleFontSize,
            weight: .bold
        )
        titleLabel.textAlignment = .left
        let infoLabel = configureLabel(
            text: info,
            color: UIColor(named: "MainTextColor")!,
            fontSize: subtitleFontSize,
            weight: .regular
        )
        infoLabel.textAlignment = .left
        
        let view = UIStackView(arrangedSubviews: [titleLabel, infoLabel])
        view.axis = .horizontal
        view.distribution = .equalCentering
        view.spacing = 5
        view.translatesAutoresizingMaskIntoConstraints = false
        return view
    }
}
