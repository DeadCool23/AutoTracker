import UIKit

class AuthViewController: UIViewController {
    private var contentView = AuthView()
    
    override func loadView() {
        view = contentView
    }
    
    override func viewDidLoad() {
        super.viewDidLoad()
        
        setupActions()
    }
    
    private func setupActions() {
        contentView.authButton.addTarget(self, action: #selector(authButtonTapped), for: .touchUpInside)
        contentView.regButton.addTarget(self, action:  #selector(regButtonTapped), for: .touchUpInside)
    }
}

extension AuthViewController {
    @objc
    private func authButtonTapped() {
        contentView.errorMsgLabel.text = ""
        
        let (user, statuCode) = AuthAPIManager.auth(
            login: contentView.loginTextField.text ?? "",
            password: contentView.passwordTextField.text ?? ""
        )
        if user == nil {
            contentView.errorMsgLabel.text = CodeHandler.serverCodeToMessage(code: statuCode)
        } else {
            let APICode = user?.status.code ?? -1
            if APICode != 0 {
                contentView.errorMsgLabel.text = CodeHandler.APICodeToMessage(code: APICode).0
            } else {
                contentView.errorMsgLabel.text = ""
                
                if let userData = user?.user {
                   do {
                       let encoder = JSONEncoder()
                       let encodedUser = try encoder.encode(userData)
                       UserDefaults.standard.set(encodedUser, forKey: "currentUser")
                       UserDefaults.standard.synchronize()
                       
                       navigationController?.setViewControllers([MainViewControllerFactory.mainView(user: userData)], animated: true)
                   } catch {
                       print("Ошибка при сохранении пользователя: \(error)")
                       contentView.errorMsgLabel.text = "Ошибка сохранения данных"
                   }
               }
            }
        }
    }
    
    @objc
    private func regButtonTapped() {
        navigationController?.pushViewController(RegViewController(), animated: true)
    }
}
