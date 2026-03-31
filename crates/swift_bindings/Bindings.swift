import AppKit
import ApplicationServices

@_cdecl("NSAppearanceEffectiveAppearance")
public func CABI_NSAppearanceEffectiveAppearance() -> Int32 {
    let appearance = NSApp?.effectiveAppearance ?? NSAppearance.currentDrawing()
    let bestMatch = appearance.bestMatch(from: [.darkAqua, .aqua])

    switch bestMatch {
        case .darkAqua?:
            return 1
        default:
            return 0
    }
}