import UIKit

class EmployeeMainViewController: UIViewController {
    private var contentView: EmployeeMainView
    
    init(user: User) {
        contentView = EmployeeMainView(user: user)
        super.init(nibName: nil, bundle: nil)
    }
    
    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
    
    override func loadView() {
        view = contentView
    }
    
    override func viewDidLoad() {
        super.viewDidLoad()
        
        navigationController?.setNavigationBarHidden(true, animated: false)
        setupActions()
    }
    
    private func setupActions() {
        contentView.exitButton.addTarget(self, action: #selector(exitButtonTapped), for: .touchUpInside)
        contentView.fioSearchButton.addTarget(self, action: #selector(fioSearchButtonTapped), for: .touchUpInside)
        contentView.dateSearchButton.addTarget(self, action: #selector(dateSearchButtonTapped), for: .touchUpInside)
        contentView.gosNumSearchButton.addTarget(self, action: #selector(gosNumSearchButtonTapped), for: .touchUpInside)
        contentView.passportSearchButton.addTarget(self, action: #selector(passportSearchButtonTapped), for: .touchUpInside)
        contentView.filterSearchButton.addTarget(self, action: #selector(filterSearchButtonTapped), for: .touchUpInside)
        contentView.datePicker.addTarget(self, action: #selector(self.dateChanged(_:)), for: .valueChanged)
        contentView.dateSearchControl.addTarget(self, action: #selector(dateSearchControlChanged), for: .valueChanged)
    }
}

extension EmployeeMainViewController {
    @objc
    private func dateChanged(_ sender: UIDatePicker) {
        contentView.dateTextField.text = contentView.dateFormatter.string(from: sender.date)
    }
    
    @objc
    private func dateSearchControlChanged(_ sender: UISwitch) {
        let color = sender.isOn ? UIColor(named: "MainTextColor") : UIColor(named: "SubTextColor")
        contentView.dateTextField.isUserInteractionEnabled = sender.isOn
        contentView.dateTextField.alpha = sender.isOn ? 1.0 : 0.5
        contentView.dateSearchButton.backgroundColor = color
        contentView.dateSearchButton.isEnabled = sender.isOn
        contentView.updateButtons()
    }
}

extension EmployeeMainViewController {
    private enum SearchType {
        case car
        case trackInfo
    }
    
    @objc
    private func filterSearchButtonTapped() {
        contentView.errors.forEach { key, value in
            value.text = ""
        }
        
        var cars: [Car]? = nil
        var trackInfo: [TrackInfo]? = nil
        let searchType: SearchType
        let serverCode: Int
        let status: ResponseStatus?
        
        switch contentView.user.role {
        case .operator_:
            let searchInfo = SearchCarRequest(
                lastname: contentView.lastnameTextField.text!.isEmpty ? nil : contentView.lastnameTextField.text!.capitalized,
                surname: contentView.surnameTextField.text!.isEmpty ? nil : contentView.surnameTextField.text!.capitalized,
                name: contentView.nameTextField.text!.isEmpty ? nil : contentView.nameTextField.text!.capitalized,
                passport: contentView.passportSerialTextField.text!.isEmpty || contentView.passportSerialTextField.text!.isEmpty ? nil :
                    Document(serial: contentView.passportSerialTextField.text!, number: contentView.passportNumTextField.text!),
                gos_num: contentView.gosNumTextField.text!.isEmpty ? nil : contentView.gosNumTextField.text!
            )
            
            let response = SearchAPIManager.searchCar(filters: searchInfo)
            serverCode = response.1
            status = response.0?.status
            cars = response.0?.cars
            searchType = .car
        case .audit_:
            let searchInfo = SearchTrackInfoRequest(
                lastname: contentView.lastnameTextField.text!.isEmpty ? nil : contentView.lastnameTextField.text!.capitalized,
                surname: contentView.surnameTextField.text!.isEmpty ? nil : contentView.surnameTextField.text!.capitalized,
                name: contentView.nameTextField.text!.isEmpty ? nil : contentView.nameTextField.text!.capitalized,
                passport: contentView.passportSerialTextField.text!.isEmpty || contentView.passportNumTextField.text!.isEmpty ? nil :
                    Document(serial: contentView.passportSerialTextField.text!, number: contentView.passportNumTextField.text!),
                date: contentView.dateTextField.text!.isEmpty ? nil : contentView.dateTextField.text!,
                gos_num: contentView.gosNumTextField.text!.isEmpty ? nil : contentView.gosNumTextField.text!
            )
            
            let response = SearchAPIManager.searchTrackInfo(filters: searchInfo)
            serverCode = response.1
            status = response.0?.status
            trackInfo = response.0?.track_info
            searchType = .trackInfo
        default:
            fatalError("unknown role")
        }
        
        if status == nil {
            contentView.errors["all"]?.text = CodeHandler.serverCodeToMessage(code: serverCode)
        } else {
            let (msg, errLabel) = CodeHandler.APICodeToMessage(code: status!.code)
            if status?.code != 0 {
                contentView.errors[errLabel]?.text = msg
            } else {
                switch searchType {
                case .car:
                    navigationController?.pushViewController(FindedViewController(cars: cars!), animated: true)
                case .trackInfo:
                    navigationController?.pushViewController(FindedViewController(trackInfo: trackInfo!), animated: true)
                default:
                    fatalError("unknown search type")
                }
            }
        }
    }
    
    @objc
    private func fioSearchButtonTapped() {
        contentView.errors["all"]?.text = ""
        contentView.errors["fio"]?.text = ""
        let searchInfo = SearchByFIORequest(
            lastname: contentView.lastnameTextField.text!.isEmpty ? nil : contentView.lastnameTextField.text!.capitalized,
            surname: contentView.surnameTextField.text!.isEmpty ? nil : contentView.surnameTextField.text!.capitalized,
            name: contentView.nameTextField.text!.isEmpty ? nil : contentView.nameTextField.text!.capitalized
            
        )
        
        var cars: [Car]? = nil
        var trackInfo: [TrackInfo]? = nil
        let searchType: SearchType
        let serverCode: Int
        let status: ResponseStatus?
        
        switch contentView.user.role {
        case .operator_:
            let response = SearchAPIManager.searchCar(fio: searchInfo)
            serverCode = response.1
            status = response.0?.status
            cars = response.0?.cars
            searchType = .car
        case .audit_:
            let response = SearchAPIManager.searchTrackInfo(fio: searchInfo)
            serverCode = response.1
            status = response.0?.status
            trackInfo = response.0?.track_info
            searchType = .trackInfo
        default:
            fatalError("unknown role")
        }
        
        if status == nil {
            contentView.errors["all"]?.text = CodeHandler.serverCodeToMessage(code: serverCode)
        } else {
            let (msg, errLabel) = CodeHandler.APICodeToMessage(code: status!.code)
            if status?.code != 0 {
                contentView.errors[errLabel]?.text = msg
            } else {
                switch searchType {
                case .car:
                    navigationController?.pushViewController(FindedViewController(cars: cars!), animated: true)
                case .trackInfo:
                    navigationController?.pushViewController(FindedViewController(trackInfo: trackInfo!), animated: true)
                default:
                    fatalError("unknown search type")
                }
            }
        }
    }
    
    @objc
    private func dateSearchButtonTapped() {
        contentView.errors["all"]?.text = ""
        contentView.errors["date"]?.text = ""
        let searchInfo = SearchByDateRequest(
            date: contentView.dateTextField.text ?? ""
        )
        
        let (result, serverCode) = SearchAPIManager.searchTrackInfo(date: searchInfo)

        let status = result?.status
        let trackInfo = result?.track_info
        
        if status == nil {
            contentView.errors["all"]?.text = CodeHandler.serverCodeToMessage(code: serverCode)
        } else {
            let (msg, errLabel) = CodeHandler.APICodeToMessage(code: status!.code)
            if status?.code != 0 {
                contentView.errors[errLabel]?.text = msg
            } else {
                navigationController?.pushViewController(FindedViewController(trackInfo: trackInfo!), animated: true)
            }
        }
    }
    
    @objc
    private func gosNumSearchButtonTapped() {
        contentView.errors["all"]?.text = ""
        contentView.errors["gos_num_mask"]?.text = ""
        let searchInfo = SearchByGosNumMaskRequest(
            gos_num: contentView.gosNumTextField.text ?? ""
        )
        
        var cars: [Car]? = nil
        var trackInfo: [TrackInfo]? = nil
        let searchType: SearchType
        let serverCode: Int
        let status: ResponseStatus?
        
        switch contentView.user.role {
        case .operator_:
            let response = SearchAPIManager.searchCar(gos_num_mask: searchInfo)
            serverCode = response.1
            status = response.0?.status
            cars = response.0?.cars
            searchType = .car
        case .audit_:
            let response = SearchAPIManager.searchTrackInfo(gos_num_mask: searchInfo)
            serverCode = response.1
            status = response.0?.status
            trackInfo = response.0?.track_info
            searchType = .trackInfo
        default:
            fatalError("unknown role")
        }
        
        if status == nil {
            contentView.errors["all"]?.text = CodeHandler.serverCodeToMessage(code: serverCode)
        } else {
            let (msg, errLabel) = CodeHandler.APICodeToMessage(code: status!.code)
            if status?.code != 0 {
                contentView.errors[errLabel]?.text = msg
            } else {
                switch searchType {
                case .car:
                    navigationController?.pushViewController(FindedViewController(cars: cars!), animated: true)
                case .trackInfo:
                    navigationController?.pushViewController(FindedViewController(trackInfo: trackInfo!), animated: true)
                default:
                    fatalError("unknown search type")
                }
            }
        }
    }
    
    @objc
    private func passportSearchButtonTapped() {
        contentView.errors["all"]?.text = ""
        contentView.errors["passport"]?.text = ""
        let searchInfo = SearchByPassportRequest(passport: Document(
            serial: contentView.passportSerialTextField.text ?? "",
            number: contentView.passportNumTextField.text ?? ""
        ))
        
        var cars: [Car]? = nil
        var trackInfo: [TrackInfo]? = nil
        let searchType: SearchType
        let serverCode: Int
        let status: ResponseStatus?
        
        switch contentView.user.role {
        case .operator_:
            let response = SearchAPIManager.searchCar(passport: searchInfo)
            serverCode = response.1
            status = response.0?.status
            cars = response.0?.cars
            searchType = .car
        case .audit_:
            let response = SearchAPIManager.searchTrackInfo(passport: searchInfo)
            serverCode = response.1
            status = response.0?.status
            trackInfo = response.0?.track_info
            searchType = .trackInfo
        default:
            fatalError("unknown role")
        }
        
        if status == nil {
            contentView.errors["all"]?.text = CodeHandler.serverCodeToMessage(code: serverCode)
        } else {
            let (msg, errLabel) = CodeHandler.APICodeToMessage(code: status!.code)
            if status?.code != 0 {
                contentView.errors[errLabel]?.text = msg
            } else {
                switch searchType {
                case .car:
                    navigationController?.pushViewController(FindedViewController(cars: cars!), animated: true)
                case .trackInfo:
                    navigationController?.pushViewController(FindedViewController(trackInfo: trackInfo!), animated: true)
                default:
                    fatalError("unknown search type")
                }
            }
        }
    }
}

extension EmployeeMainViewController {
    @objc
    private func exitButtonTapped() {
        showAcceptAlert()
        UserDefaults.standard.removeObject(forKey: "currentUser")
        UserDefaults.standard.synchronize()
        
        navigationController?.setViewControllers([AuthViewController()], animated: true)
    }
    
    private func showAcceptAlert() {
        let alert = UIAlertController(
            title: "Выйти из AutoTracker?",
            message: nil,
            preferredStyle: .alert
        )
        alert.overrideUserInterfaceStyle = .dark
        
        let cancelAction = UIAlertAction(title: "Отмена", style: .cancel)
        cancelAction.setValue(UIColor.white, forKey: "titleTextColor")
        alert.addAction(cancelAction)
        alert.addAction(UIAlertAction(title: "Выйти", style: .destructive) { [weak self] _ in
            UserDefaults.standard.removeObject(forKey: "currentUser")
            UserDefaults.standard.synchronize()
            
            self?.navigationController?.setViewControllers([AuthViewController()], animated: true)
        })
        present(alert, animated: true)
    }
}
