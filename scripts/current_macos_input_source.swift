import Carbon
import Foundation

let source = TISCopyCurrentKeyboardInputSource().takeRetainedValue()
guard let pointer = TISGetInputSourceProperty(
    source,
    kTISPropertyInputSourceID
) else {
    FileHandle.standardError.write(Data("missing input source id\n".utf8))
    exit(1)
}
let identifier = Unmanaged<CFString>
    .fromOpaque(pointer).takeUnretainedValue() as String
print(identifier)
