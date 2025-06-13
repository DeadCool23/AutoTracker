import UIKit

class RegViewController: UIViewController {
    private var contentView = RegView()
    
    override func loadView() {
        view = contentView
    }
    
    override func viewDidLoad() {
        super.viewDidLoad()
        
        navigationController?.setNavigationBarHidden(false, animated: false)
        
        setupCustomBackButton()
        setupActions()
    }
    
    private func setupActions() {
        contentView.regButton.addTarget(self, action: #selector(regButtonTapped), for: .touchUpInside)
    }
}

extension RegViewController {
    private func setupCustomBackButton() {
        let backButton = UIBarButtonItem(
            image: UIImage(named: "Arrow.back"),
            style: .plain,
            target: self,
            action: #selector(backButtonTapped)
        )
        backButton.imageInsets = UIEdgeInsets(top: -7, left: 0, bottom: 0, right: 0)
        navigationItem.leftBarButtonItem = backButton
        
        navigationController?.navigationBar.tintColor = UIColor(named: "MainTextColor")
    }
    
    @objc
    private func backButtonTapped() {
        navigationController?.popViewController(animated: true)
    }
    
    @objc
    private func regButtonTapped() {
        for err in contentView.errors.values {
            err.text = ""
        }
        
        let (data, statusCode) = RegAPIManager.reg(data: RequestRegData(
            firstname: contentView.nameTextField.text ?? "",
            surname: contentView.surnameTextField.text ?? "",
            lastname: contentView.lastnameTextField.text ?? "",
            email: contentView.emailTextField.text ?? "",
            pswd: contentView.pswdTextField.text ?? "",
            rep_pswd: contentView.repPswdTextField.text ?? ""
        ))
        
        if data == nil {
            contentView.errors["all"]!.text = CodeHandler.serverCodeToMessage(code: statusCode)
        } else {
            let APICode = data?.status.code ?? -1
            if APICode != 0 {
                let (msg, label) = CodeHandler.APICodeToMessage(code: APICode)
                contentView.errors[label]!.text = msg
            } else {
                for err in contentView.errors.values {
                    err.text = ""
                }
                
                showSuccessAlert()
            }
        }
    }
}

extension RegViewController {
    private func showSuccessAlert() {
        let alert = UIAlertController(
            title: "Регистрация",
            message: "Пользователь с почтой \(contentView.emailTextField.text!) успешно зарегистрирован",
            preferredStyle: .alert
        )
        alert.overrideUserInterfaceStyle = .dark
        
        let okAction = UIAlertAction(title: "OK", style: .default) { [weak self] _ in
            self?.navigationController?.popViewController(animated: true)
        }
        
        okAction.setValue(UIColor.white, forKey: "titleTextColor")
        alert.addAction(okAction)
        present(alert, animated: true, completion: nil)
    }
}
