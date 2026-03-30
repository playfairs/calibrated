import AppKit
import ApplicationServices

public enum AppleUIInterfaceStyle: Int32 {
    case light = 0
    case dark = 1
}

@_cdecl("NSUIInterfaceStyleCollect")
public func CollectAppleUIInterfaceStyle() -> Int32 {
    let appearance = NSApp?.effectiveAppearance ?? NSAppearance.currentDrawing()
    let bestMatch = appearance.bestMatch(from: [.darkAqua, .aqua])

    switch bestMatch {
    case .darkAqua?:
        return 1
    default:
        return 0
    }
}

@_cdecl("testing_func")
public func testingFunc() -> UnsafePointer<CChar>? {
    let str = "Hello AX"
    let mutablePtr = strdup(str)
    return UnsafePointer(mutablePtr)
}
