import Foundation

public func printHelloWorld(_ extra: String?) {
    let trailer = extra ?? "(util)"
    print("Hello, World! \(trailer)") 
}

public func readTextFile(atPath filePath: String) throws -> String {
    let fileURL = URL(fileURLWithPath: filePath)
    return try String(contentsOf: fileURL, encoding: .utf8)
}

public func readTextFileAsLines(atPath filePath: String) throws -> [String] {
    let fileText = try readTextFile(atPath: filePath)
    let linesSequence = fileText.split(whereSeparator: \.isNewline)
    var lines: [String] = []
    lines.reserveCapacity(linesSequence.count)
    for line in linesSequence {
        lines.append(String(line))
    }
    return lines
}

public func readBinaryFile(atPath filePath: String) throws -> Data {
    let fileURL = URL(fileURLWithPath: filePath)
    return try Data(contentsOf: fileURL)
}