import Foundation

// A C-compatible callback definition for Rust
typealias McWalkCallback = @convention(c) (UnsafePointer<CChar>?) -> Void

@_cdecl("mcwalk_enumerate")
func mcwalk_enumerate(cPath: UnsafePointer<CChar>, callback: McWalkCallback) -> Int32 {
    let path = String(cString: cPath)
    let url = URL(fileURLWithPath: path)
    
    let keys: [URLResourceKey] = []
    let options: FileManager.DirectoryEnumerationOptions = [.skipsHiddenFiles, .skipsPackageDescendants]
    
    guard let enumerator = FileManager.default.enumerator(
        at: url,
        includingPropertiesForKeys: keys,
        options: options,
        errorHandler: { _, _ in true }
    ) else {
        return -1 // Failure to create enumerator
    }
    
    for case let fileURL as URL in enumerator {
        // Pass the absolute path back to Rust
        fileURL.path.withCString { cString in
            callback(cString)
        }
    }
    
    return 0 // Success
}
