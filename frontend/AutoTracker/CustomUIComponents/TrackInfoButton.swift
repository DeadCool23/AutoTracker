import UIKit

class TrackInfoButton : UIControl {
    let trackInfo: TrackInfo
    let buttonCornerRadius: CGFloat = 25
    
    let gosNumHeight: CGFloat = 39
    let gosNumWidth: CGFloat = 123
    
    init(_ trackInfo: TrackInfo) {
        self.trackInfo = trackInfo
        
        super.init(frame: .zero)
        
        setupButtonActions(self)
        setupView()
    }
    
    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
    
    private func setupView() {
        backgroundColor = UIColor(named: "FrameColor")
        
        layer.cornerRadius = buttonCornerRadius
        
        let separatorView = UIView()
        separatorView.backgroundColor = UIColor(named: "SubTextColor")!
        separatorView.translatesAutoresizingMaskIntoConstraints = false
        separatorView.heightAnchor.constraint(equalToConstant: smallBorderWidth).isActive = true
        
        let dateLabel = configureLabel(
            text: "Время отслеживания: \(trackInfo.track_time)",
            color: UIColor(named: "SubTextColor")!,
            fontSize: carFontSize,
            weight: .semibold
        )
        let fioLabel = configureLabel(
            text: "\(trackInfo.user.surname.capitalized) \(trackInfo.user.name.capitalized)",
            color: UIColor(named: "MainTextColor")!,
            fontSize: fioFontSize,
            weight: .semibold
        )
        let carLabel = configureLabel(
            text: "\(trackInfo.user.role.roleToString())",
            color: UIColor(named: "SubTextColor")!,
            fontSize: carFontSize,
            weight: .semibold
        )
        let gosNumLabel = configureGosNum(gosNum: trackInfo.car.gos_num)
        
        addSubview(dateLabel)
        addSubview(separatorView)
        addSubview(fioLabel)
        addSubview(carLabel)
        addSubview(gosNumLabel)
        
        let alligmetToGosNum: CGFloat = 4
        let dateAlligment: CGFloat = 5
        
        NSLayoutConstraint.activate([
            dateLabel.centerXAnchor.constraint(equalTo: centerXAnchor),
            dateLabel.topAnchor.constraint(equalTo: topAnchor, constant: dateAlligment),
            
            separatorView.topAnchor.constraint(equalTo: dateLabel.bottomAnchor, constant: dateAlligment),
            separatorView.leadingAnchor.constraint(equalTo: leadingAnchor),
            separatorView.trailingAnchor.constraint(equalTo: trailingAnchor),
            
            gosNumLabel.topAnchor.constraint(equalTo: separatorView.bottomAnchor, constant: dateAlligment * 2),
            gosNumLabel.bottomAnchor.constraint(equalTo: bottomAnchor, constant: -inMiddleFrameAligment),
            gosNumLabel.trailingAnchor.constraint(equalTo: trailingAnchor, constant: -inMiddleFrameAligment),
            
            fioLabel.leadingAnchor.constraint(equalTo: leadingAnchor, constant: inMiddleFrameAligment),
            fioLabel.topAnchor.constraint(equalTo: gosNumLabel.topAnchor, constant: alligmetToGosNum),
            
            carLabel.leadingAnchor.constraint(equalTo: leadingAnchor, constant: inMiddleFrameAligment),
            carLabel.bottomAnchor.constraint(equalTo: gosNumLabel.bottomAnchor, constant: -alligmetToGosNum),
        ])
    }
}

extension TrackInfoButton {
    private func configureLabel(text: String, color: UIColor, fontSize: CGFloat, weight: UIFont.Weight) -> UILabel {
        let textLabel = UILabel()
        
        textLabel.text = text
        textLabel.textColor = color
        textLabel.font = UIFont.systemFont(ofSize: fontSize, weight: weight)
        textLabel.translatesAutoresizingMaskIntoConstraints = false
        return textLabel
    }
    
    private func configureGosNum(gosNum: String) -> UIView {
        let textLabel = UILabel()
        let view = UIView()
        textLabel.text = gosNum
        textLabel.textAlignment = .center
        
        let textColor = UIColor(named: "DarkTextColor")!
        let backgroundColor = UIColor(named: "MainTextColor")!
        
        view.backgroundColor = backgroundColor
        textLabel.textColor = textColor
        textLabel.backgroundColor = backgroundColor
        
        textLabel.layer.cornerRadius = smallCornerRadius - 1
        view.layer.cornerRadius = smallCornerRadius
        textLabel.layer.borderWidth = smallBorderWidth
        textLabel.layer.borderColor = textColor.cgColor
        
        textLabel.clipsToBounds = true
        
        textLabel.font = UIFont.systemFont(ofSize: stdFontSize, weight: .semibold)
        view.translatesAutoresizingMaskIntoConstraints = false
        textLabel.translatesAutoresizingMaskIntoConstraints = false
        
        view.addSubview(textLabel)
        NSLayoutConstraint.activate([
            view.widthAnchor.constraint(equalToConstant: gosNumWidth),
            view.heightAnchor.constraint(equalToConstant: gosNumHeight),
            
            textLabel.topAnchor.constraint(equalTo: view.topAnchor, constant: smallBorderWidth),
            textLabel.bottomAnchor.constraint(equalTo: view.bottomAnchor, constant: -smallBorderWidth),
            textLabel.leadingAnchor.constraint(equalTo: view.leadingAnchor, constant: smallBorderWidth),
            textLabel.trailingAnchor.constraint(equalTo: view.trailingAnchor, constant: -smallBorderWidth),
        ])
        
        return view
    }
}

extension TrackInfoButton {
    private func setupButtonActions(_ sender: UIControl) {
        sender.addTarget(self, action: #selector(buttonTouchDown(_:)), for: .touchDown)
        sender.addTarget(self, action: #selector(buttonTouchUp(_:)), for: [.touchUpInside, .touchUpOutside, .touchCancel])
    }

    @objc
    private func buttonTouchDown(_ sender: UIControl) {
        UIView.animate(withDuration: 0.1) {
            sender.alpha = 0.5
        }
    }

    @objc
    private func buttonTouchUp(_ sender: UIControl) {
        UIView.animate(withDuration: 0.1) {
            sender.alpha = 1.0
        }
    }
}
