import UIKit

class FindedViewController: UIViewController {
    private var contentView: FindedView
    
    init(cars: [Car]) {
        contentView = FindedView(cars: cars)
        super.init(nibName: nil, bundle: nil)
    }
    
    init(trackInfo: [TrackInfo]) {
        contentView = FindedView(trackInfo: trackInfo)
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
        contentView.backButton.addTarget(self, action: #selector(backButtonTapped), for: .touchUpInside)
        contentView.carButtons.forEach {
            $0.addTarget(self, action: #selector(carButtonTapped(_:)), for: .touchUpInside)
        }
        contentView.trackInfoButtons.forEach {
            $0.addTarget(self, action: #selector(trackInfoButtonTapped(_:)), for: .touchUpInside)
        }
    }
}

extension FindedViewController {
    @objc
    private func backButtonTapped() {
        navigationController?.popViewController(animated: true)
    }
    
    @objc
    private func trackInfoButtonTapped(_ sender: TrackInfoButton) {
        navigationController?.pushViewController(TrackInfoViewController(trackInfo: sender.trackInfo), animated: true)
    }
    
    @objc
    private func carButtonTapped(_ sender: OperatorRouteButton) {
        if let savedUserData = UserDefaults.standard.data(forKey: "currentUser") {
            do {
                let decoder = JSONDecoder()
                let user = try decoder.decode(User.self, from: savedUserData)
                let vC = RouteViewController(user: user, car: sender.car)
                navigationController?.pushViewController(vC, animated: true)
            } catch {
                print("Ошибка при чтении пользователя: \(error)")
                navigationController?.setViewControllers([AuthViewController()], animated: true)
            }
        } else {
            navigationController?.setViewControllers([AuthViewController()], animated: true)
        }
    }
}
