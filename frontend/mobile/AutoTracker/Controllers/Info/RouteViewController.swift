import UIKit

class RouteViewController: UIViewController {
    private var contentView: RouteView
    
    init(user:User, car: Car) {
        contentView = RouteView(car: car, user: user)
        super.init(nibName: nil, bundle: nil)
        contentView.delegate = self
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
        updateRoute()
    }
    
    private func setupActions() {
        contentView.backButton.addTarget(self, action: #selector(backButtonTapped), for: .touchUpInside)
        contentView.dateButton.addTarget(self, action: #selector(dateButtonTapped), for: .touchUpInside)
    }
}

extension RouteViewController {
    private func updateRoute() {
        let date = contentView.dateFormatter.string(from: contentView.routeDate)
        
        let response = RouteGetAPIManager.get(data: RequestRouteData(
            date: date,
            gos_num: contentView.car.gos_num,
            user_login: contentView.user.email
        ))
        
        let serverCode = response.1
        let status = response.0?.status
        
        if status == nil {
            self.showAlert(title: "Ошибка", message: CodeHandler.serverCodeToMessage(code: serverCode))
        } else {
            let (msg, _) = CodeHandler.APICodeToMessage(code: status!.code)
            if status?.code != 0 {
                self.showAlert(title: "Ошибка", message: msg)
            } else {
                let cords = response.0!.route
                contentView.route = Route(cords: cords ?? [])
                if cords == nil {
                    self.showAlert(message: "Не найден маршрут от \(date)")
                }
                contentView.updateMapPoints()
            }
        }
    }
}

extension RouteViewController {
    @objc
    private func dateButtonTapped() {
        let alert = UIAlertController(
            title: "Введите дату искомого маршрута",
            message: nil,
            preferredStyle: .alert
        )
        alert.overrideUserInterfaceStyle = .dark
        
        alert.addTextField { textField in
            textField.text = self.contentView.dateFormatter.string(from: Date())
            textField.tag = 0
            
            let datePicker = UIDatePicker()
            datePicker.datePickerMode = .date
            datePicker.preferredDatePickerStyle = .inline
            datePicker.maximumDate = Date()
            datePicker.locale = Locale(identifier: "ru_RU")
            datePicker.tintColor = .lightGray
            textField.inputView = datePicker

            datePicker.addTarget(self, action: #selector(self.dateChanged(_:)), for: .valueChanged)
        }
        
        let confirmAction = UIAlertAction(title: "Подтвердить", style: .default) { [weak self] _ in
            guard let self = self,
                          let dateField = alert.textFields?.first(where: { $0.tag == 0 }),
                          let dateText = dateField.text else { return }

            print("Выбрана дата: \(dateText)")
            
            let formatter = self.contentView.dateFormatter
            self.contentView.routeDate = formatter.date(from: dateText)!
            self.contentView.dateLabel.text = "маршрут от \(formatter.string(from: self.contentView.routeDate))"
            
            self.updateRoute()
        }
        
        let cancelAction = UIAlertAction(title: "Отмена", style: .cancel)
        
        confirmAction.setValue(UIColor.white, forKey: "titleTextColor")
        cancelAction.setValue(UIColor.white, forKey: "titleTextColor")
        alert.addAction(confirmAction)
        alert.addAction(cancelAction)
        
        present(alert, animated: true)
    }
    
    @objc
    private func dateChanged(_ sender: UIDatePicker) {
        if let alertController = presentedViewController as? UIAlertController,
            let textField = alertController.textFields?.first(where: { $0.tag == 0 }) {
            textField.text = contentView.dateFormatter.string(from: sender.date)
        }
    }

    
    @objc
    private func backButtonTapped() {
        navigationController?.popViewController(animated: true)
    }
}

extension RouteViewController {
    func showAlert(title: String? = nil, message: String?) {
        let alert = UIAlertController(
            title: title,
            message: message,
            preferredStyle: .alert
        )
        alert.overrideUserInterfaceStyle = .dark
        
        let okAction = UIAlertAction(title: "OK", style: .default)
        okAction.setValue(UIColor.white, forKey: "titleTextColor")
        alert.addAction(okAction)
        present(alert, animated: true)
    }
}
