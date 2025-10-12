import UIKit
import MapKit

class RoutePointAnnotation: MKPointAnnotation {
    var icamera: Int = 0
    var pointData: PointData?
}

class RouteView: UIView {
    lazy var backButton: UIButton = {
        let button = UIButton()
        button.setImage(UIImage(named: "Arrow.back"), for: .normal)
        button.isEnabled = true
        return button
    }()
    
    lazy var dateButton: UIButton = {
        let button = UIButton()
        button.setImage(UIImage(named: "Calendar.button"), for: .normal)
        button.isEnabled = true
        return button
    }()
    
    private lazy var headerView: UIView = {
        let view = UIView()
        view.backgroundColor = UIColor(named: "FrameColor")?.withAlphaComponent(0.9)
        
        view.layer.cornerRadius = bigCornerRadius
        view.layer.maskedCorners = [.layerMaxXMaxYCorner, .layerMinXMaxYCorner]
        
        let textLabel = configureGosNum(gosNum: car.gos_num)
        backButton.translatesAutoresizingMaskIntoConstraints = false
        dateButton.translatesAutoresizingMaskIntoConstraints = false
        
        view.addSubview(textLabel)
        view.addSubview(dateButton)
        view.addSubview(backButton)
        NSLayoutConstraint.activate([
            backButton.bottomAnchor.constraint(equalTo: view.bottomAnchor, constant: -stdAligment),
            backButton.leadingAnchor.constraint(equalTo: view.leadingAnchor, constant: bigAligment),
            
            dateButton.bottomAnchor.constraint(equalTo: view.bottomAnchor, constant: -stdAligment),
            dateButton.trailingAnchor.constraint(equalTo: view.trailingAnchor, constant: -bigAligment),
            
            textLabel.centerXAnchor.constraint(equalTo: view.centerXAnchor),
            textLabel.centerYAnchor.constraint(equalTo: backButton.centerYAnchor)
        ])
        
        return view
    }()
    
    private let gosNumHeight: CGFloat = 39
    private let gosNumWidth: CGFloat = 123
    
    let car: Car
    let user: User
    var route: Route = Route(cords: [])
    var delegate: RouteViewController?
    
    private let mapView = MKMapView()
    
    let dateFormatter: DateFormatter = {
        let formatter = DateFormatter()
        formatter.dateFormat = "dd.MM.yyyy"
        return formatter
    }()
    var routeDate: Date = Date()
    lazy var dateLabel: UILabel = {
        let label = UILabel()
        label.text = "маршрут от \(dateFormatter.string(from: routeDate))"
        label.font = UIFont.systemFont(ofSize: subtitleFontSize, weight: .regular)
        label.textAlignment = .center
        
        label.layer.cornerRadius = 15
        label.clipsToBounds = true
        label.textColor = UIColor(named: "MainTextColor")
        label.backgroundColor = UIColor(named: "FrameColor")?.withAlphaComponent(0.9)
        return label
    }()
    
    init(car: Car, user: User) {
        self.car = car
        self.user = user
        
        super.init(frame: .zero)
        setupView()
    }
    
    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
    
    private func setupView() {
        backgroundColor = UIColor(named: "AccentColor")
        
        setupMap()
        setupHeader()
        setupDate()
    }
    
    private func setupMap() {
        addSubview(mapView)
        mapView.delegate = self
        mapView.overrideUserInterfaceStyle = .dark
        mapView.translatesAutoresizingMaskIntoConstraints = false
        NSLayoutConstraint.activate([
            mapView.topAnchor.constraint(equalTo: topAnchor),
            mapView.bottomAnchor.constraint(equalTo: bottomAnchor),
            mapView.leadingAnchor.constraint(equalTo: leadingAnchor),
            mapView.trailingAnchor.constraint(equalTo: trailingAnchor)
        ])
        
        let moscowCoordinates = CLLocationCoordinate2D(latitude: 55.7558, longitude: 37.6173)
        let region = MKCoordinateRegion(
            center: moscowCoordinates,
            span: MKCoordinateSpan(latitudeDelta: 0.3, longitudeDelta: 0.3)
        )
        mapView.setRegion(region, animated: false)
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
    
    private func setupDate() {
        addSubview(dateLabel)
        dateLabel.translatesAutoresizingMaskIntoConstraints = false
        
        NSLayoutConstraint.activate([
            dateLabel.widthAnchor.constraint(equalToConstant: 177),
            dateLabel.heightAnchor.constraint(equalToConstant: 30),
            dateLabel.centerXAnchor.constraint(equalTo: self.centerXAnchor),
//            dateLabel.bottomAnchor.constraint(equalTo: safeAreaLayoutGuide.bottomAnchor)
            dateLabel.topAnchor.constraint(equalTo: headerView.bottomAnchor, constant: smallAligment)
        ])
    }
}

extension RouteView {
    func updateMapPoints() {
        mapView.removeAnnotations(mapView.annotations)
        mapView.removeOverlays(mapView.overlays)
        
        var annotations: [MKPointAnnotation] = []
        var coordinates: [CLLocationCoordinate2D] = []
        
        for (index, point) in route.cords.enumerated() {
            let coordinate = CLLocationCoordinate2D(
                latitude: point.cords.latitude,
                longitude: point.cords.longitude
            )
            coordinates.append(coordinate)
            
            let annotation = RoutePointAnnotation()
            annotation.coordinate = coordinate
            annotation.title = "\(index + 1)"
            annotation.icamera = index
            annotation.pointData = point
            annotations.append(annotation)
        }
        
        mapView.addAnnotations(annotations)
        
        let polyline = MKPolyline(coordinates: coordinates, count: coordinates.count)
        mapView.addOverlay(polyline)
        
        if !coordinates.isEmpty {
            let rect = polyline.boundingMapRect
            mapView.setVisibleMapRect(
                rect,
                edgePadding: UIEdgeInsets(top: 60, left: 40, bottom: 60, right: 40),
                animated: true
            )
        }
    }
}

extension RouteView: MKMapViewDelegate {
    func mapView(_ mapView: MKMapView, viewFor annotation: MKAnnotation) -> MKAnnotationView? {
        if annotation is MKUserLocation {
            return nil
        }

        let identifier = "RoutePin"

        var view = mapView.dequeueReusableAnnotationView(withIdentifier: identifier) as? MKMarkerAnnotationView
        if view == nil {
            view = MKMarkerAnnotationView(annotation: annotation, reuseIdentifier: identifier)
            view?.markerTintColor = .red
            view?.glyphImage = UIImage(systemName: "camera")
            view?.canShowCallout = true
        } else {
            view?.annotation = annotation
        }

        return view
    }

    func mapView(_ mapView: MKMapView, rendererFor overlay: MKOverlay) -> MKOverlayRenderer {
        if let polyline = overlay as? MKPolyline {
            let renderer = MKPolylineRenderer(polyline: polyline)
            renderer.strokeColor = .systemBlue
            renderer.lineWidth = 3
            return renderer
        }
        return MKOverlayRenderer(overlay: overlay)
    }
    
    func mapView(_ mapView: MKMapView, didSelect view: MKAnnotationView) {
        guard
            let annotation = view.annotation as? RoutePointAnnotation
        else { return }
        
        let calloutView = configureCalloutView(annotation)
        
        view.detailCalloutAccessoryView = calloutView
    }
}


extension RouteView {
    private func configureCalloutView(_ annotation: RoutePointAnnotation) -> UIView {
        let pointData = route.cords[annotation.icamera]
        
        let calloutView = UIView()
        calloutView.translatesAutoresizingMaskIntoConstraints = false
        
        let coordinateLabel = UILabel()
        coordinateLabel.text = String(format: "Широта: %.6f\nДолгота: %.6f",
                                    pointData.cords.latitude,
                                    pointData.cords.longitude)
        coordinateLabel.numberOfLines = 2
        
        let speedLabel = UILabel()
        var speedInfo = ""
        if let speed = pointData.speed {
            let speedTextTitle = "Скорость (км/ч):"
            let mainSpeedText =  "  текущая - \(speed)"
            let avgSpeed = getAvgSpeed(
                gosNum: car.gos_num,
                pointData: pointData
            )
            let avgSpeedText = "  средняя - " + (avgSpeed == nil ? "н/д" : String(format: "%.1f", avgSpeed!))
            speedInfo = "\(speedTextTitle)\n\(mainSpeedText)\n\(avgSpeedText)"
        } else {
            let mainSpeedText = "Скорость: н/д"
            speedInfo = "\(mainSpeedText)"
        }
        speedLabel.text = speedInfo
        speedLabel.numberOfLines = 3
        
        let stackView = UIStackView(arrangedSubviews: [
            coordinateLabel,
            speedLabel,
        ])
        stackView.axis = .vertical
        stackView.spacing = 4
        stackView.translatesAutoresizingMaskIntoConstraints = false
        
        calloutView.addSubview(stackView)
        
        NSLayoutConstraint.activate([
            stackView.topAnchor.constraint(equalTo: calloutView.topAnchor, constant: 8),
            stackView.bottomAnchor.constraint(equalTo: calloutView.bottomAnchor, constant: -8),
            stackView.leadingAnchor.constraint(equalTo: calloutView.leadingAnchor),
            stackView.trailingAnchor.constraint(equalTo: calloutView.trailingAnchor)
        ])
        
        [coordinateLabel, speedLabel].forEach {
            $0.font = UIFont.systemFont(ofSize: 14)
            $0.textColor = UIColor(named: "MainTextColor")
        }
        
        return calloutView
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

extension RouteView {
    private func getAvgSpeed(gosNum: String, pointData: PointData) -> Double? {
        let response = CameraAPIManager.getAvgSpeed(data: RequestAvgSpeedData(
            gos_num: gosNum,
            location: pointData.cords
        ))
        
        let serverCode = response.1
        let status = response.0?.status
        
        if status == nil {
            delegate!.showAlert(title: "Ошибка", message: CodeHandler.serverCodeToMessage(code: serverCode))
            return nil
        } else {
            let (msg, _) = CodeHandler.APICodeToMessage(code: status!.code)
            if status?.code != 0 {
                delegate!.showAlert(title: "Ошибка", message: msg)
                return nil
            } else {
                return response.0!.avg_speed
            }
        }
    }
}
