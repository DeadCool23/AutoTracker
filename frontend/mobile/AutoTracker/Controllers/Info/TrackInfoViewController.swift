import UIKit

class TrackInfoViewController: UIViewController {
    private var contentView: TrackInfoView
    
    init(trackInfo: TrackInfo) {
        contentView = TrackInfoView(trackInfo: trackInfo)
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
        contentView.backButton.addTarget(self, action: #selector(backButtonTapped), for: .touchUpInside)
    }
}

extension TrackInfoViewController {
    @objc
    private func backButtonTapped() {
        navigationController?.popViewController(animated: true)
    }
}
