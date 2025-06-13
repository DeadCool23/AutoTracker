import Foundation

let APIUrl: String = {
    guard let path = Bundle.main.path(forResource: "ApiConfig", ofType: "plist"),
          let dict = NSDictionary(contentsOfFile: path) as? [String: Any],
          let apiUrl = dict["APIUrl"] as? String else {
        fatalError("APIUrl not found in ApiConfig.plist")
    }
    return apiUrl
}()

let headerHeight: CGFloat = 100
let mainHeaderHeight: CGFloat = 120
let carButtonHeight: CGFloat = 59
let keyboardHeight: CGFloat = 350

let bigAligment: CGFloat = 30
let stdAligment: CGFloat = 16
let inMiddleFrameAligment: CGFloat = 15
let smallAligment: CGFloat = 10


let titleFontSize: CGFloat = 20
let stdFontSize: CGFloat = 16
let subtitleFontSize: CGFloat = 14
let errorFontSize: CGFloat = 14
let fioFontSize: CGFloat = 14
let carFontSize: CGFloat = 11
let smallFontSize: CGFloat = 10

let smallBorderWidth: CGFloat = 1

let textFieldHeight: CGFloat = 40

let bigCornerRadius: CGFloat = 40
let frameCornerRadius: CGFloat = 30
let intoFrameCornerRadius: CGFloat = frameCornerRadius - stdAligment
let smallCornerRadius: CGFloat = 10
