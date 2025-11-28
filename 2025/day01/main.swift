import util
import Foundation

func getFileName(fromPath filePath: String) -> String {
    if let lastPathSep = filePath.lastIndex(of: "/") {
        let fileNameStartIndex = filePath.index(lastPathSep, offsetBy: 1);
        return String(filePath.suffix(from: fileNameStartIndex))
    }

    return filePath
}

let selfFilePath: String = #file;
let selfFileName: String = getFileName(fromPath: selfFilePath)

util.printHelloWorld(selfFilePath);
let selfFileText: String;
let selfFileTextLines: [String];
let selfFileBinary: Data;
do {
    selfFileText = try util.readTextFile(atPath: selfFilePath);
    selfFileTextLines = try util.readTextFileAsLines(atPath: selfFilePath);
    selfFileBinary = try util.readBinaryFile(atPath: selfFilePath);
} catch {
    fatalError("Failed to read \(selfFilePath): \(error.localizedDescription)")
}

for line in selfFileTextLines {
    print("\(selfFileName): \(line)")
}

print("\(selfFilePath): \(selfFileBinary.count)")