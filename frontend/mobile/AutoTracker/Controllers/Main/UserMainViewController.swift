import UIKit

class UserMainViewController: UIViewController {
    private var contentView: UserMainView
    
    init(user: User) {
        contentView = UserMainView(user: user)
        super.init(nibName: nil, bundle: nil)
        
        contentView.updateActions = setupActions
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
        contentView.verifyButton.addTarget(self, action: #selector(verifyButtonTapped), for: .touchUpInside)
        contentView.carButtons.forEach {
            $0.addTarget(self, action: #selector(carRouteButtonTapped(_:)), for: .touchUpInside)
        }
    }
}

extension UserMainViewController {
    @objc
    private func exitButtonTapped() {
        showAcceptAlert()
    }
    
    @objc
    private func carRouteButtonTapped(_ sender: UserRouteButton) {
        let vC = RouteViewController(user: self.contentView.user, car: sender.car)
        navigationController?.pushViewController(vC, animated: true)
    }
    
    @objc private func verifyButtonTapped() {
        let alert = UIAlertController(
            title: "Введите паспортные данные",
            message: nil,
            preferredStyle: .alert
        )
        alert.overrideUserInterfaceStyle = .dark
        
        alert.addTextField { textField in
            textField.placeholder = "Серия (4 цифры)"
            textField.keyboardType = .numberPad
            textField.tag = 0
        }
        
        alert.addTextField { textField in
            textField.placeholder = "Номер (6 цифр)"
            textField.keyboardType = .numberPad
            textField.tag = 1
        }
        
        let confirmAction = UIAlertAction(title: "Подтвердить", style: .default) { [weak self] _ in
            guard let self = self,
                  let seriesField = alert.textFields?.first(where: { $0.tag == 0 }),
                  let numberField = alert.textFields?.first(where: { $0.tag == 1 }) else { return }
            
            let passport = Document(
                serial: seriesField.text ?? "",
                number: numberField.text ?? ""
            )
            
            if Validator.isValidPassport(for: passport) {
                self.verifyPassport(passport: passport)
            } else {
                self.showErrorAlert(message: "Неверный формат пасспортных данных")
            }
        }
        
        let cancelAction = UIAlertAction(title: "Отмена", style: .cancel)
        
        confirmAction.setValue(UIColor.white, forKey: "titleTextColor")
        cancelAction.setValue(UIColor.white, forKey: "titleTextColor")
        alert.addAction(confirmAction)
        alert.addAction(cancelAction)
        
        present(alert, animated: true)
    }

    private func verifyPassport(passport: Document) {
        let response = PassportConfirmAPIManager.confirmPassport(data: PassportConfirmRequest(
            email: contentView.user.email,
            passport: passport
        ))
        
        let serverCode = response.1
        let status = response.0?.status
        
        if status == nil {
            self.showErrorAlert(message: CodeHandler.serverCodeToMessage(code: serverCode))
        } else {
            let (msg, _) = CodeHandler.APICodeToMessage(code: status!.code)
            if status?.code != 0 {
                self.showErrorAlert(message: msg)
            } else {
                contentView.user.is_verified = true
                contentView.user.passport = passport
                
                contentView.unsetUnverifiedView()
                contentView.setupVerifiedView()
                
                let alert = UIAlertController(
                    title: "Пасспорт успешно подтверждён",
                    message: nil,
                    preferredStyle: .alert
                )
                alert.overrideUserInterfaceStyle = .dark
                let okAction = UIAlertAction(title: "OK", style: .default)
                okAction.setValue(UIColor.white, forKey: "titleTextColor")
                
                alert.addAction(okAction)
                present(alert, animated: true)
                
                do {
                    let encoder = JSONEncoder()
                    let encodedUser = try encoder.encode(contentView.user)
                    UserDefaults.standard.set(encodedUser, forKey: "currentUser")
                    UserDefaults.standard.synchronize()
                } catch {
                    print("Ошибка при сохранении пользователя: \(error)")
                }
            }
        }
    }

    private func showErrorAlert(message: String) {
        let alert = UIAlertController(
            title: "Ошибка",
            message: message,
            preferredStyle: .alert
        )
        alert.overrideUserInterfaceStyle = .dark
        
        let okAction = UIAlertAction(title: "OK", style: .default)
        okAction.setValue(UIColor.white, forKey: "titleTextColor")
        alert.addAction(okAction)
        present(alert, animated: true)
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
