import ApplicationServices

@_cdecl("testing_func")
public func testingFunc() -> UnsafePointer<CChar>? {
    let str = "Hello AX"
    let mutablePtr = strdup(str)
    return UnsafePointer(mutablePtr)
}
