import UIKit

class MainViewControllerFactory {
    static func mainView(user: User) -> UIViewController {
        switch user.role {
        case .user_:
            return UserMainViewController(user: user)
        case .operator_, .audit_:
            return EmployeeMainViewController(user: user)
        }
    }
}
