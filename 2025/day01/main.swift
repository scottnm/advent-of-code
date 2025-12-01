import util
import Foundation

let allArgs = CommandLine.arguments
let userArgs = allArgs.dropFirst()
print("all args: \(allArgs)")
print("user args: \(userArgs)")
if userArgs.count < 1 {
    fatalError("need input file argument")
}

let inputFilePath = userArgs[1] // N.B. interesting.. swift slices are views that share the original indices
print("inputFilePath: \(inputFilePath)")
let inputLines: [String];
do {
    inputLines = try util.readTextFileAsLines(atPath: inputFilePath);
} catch {
    fatalError("Failed to read \(inputFilePath): \(error.localizedDescription)")
}
print("input lines: \(inputLines)")

enum Direction {
    case Left
    case Right
}

struct InputPair {
    let dir: Direction
    let offset: UInt
}

struct DialSimulationStep {
    let resultPos: Int
    let zeroCrosses: UInt
}

func processInput(_ lines: [String]) -> [InputPair] {
    let LINE_RGX = /([LR])(\d+)/
    var pairs: [InputPair] = []
    for line in lines {
        let d: Direction
        let offset: UInt
        if let match = line.firstMatch(of: LINE_RGX) {
            d = match.1 == "L" ? Direction.Left : Direction.Right
            if let parsedOffset = UInt(match.2) {
                offset = parsedOffset
            } else {
                fatalError("Unexpected invalid line \(line)")
            }
        } else {
            fatalError("Unexpected invalid line \(line)")
        }

        pairs.append(InputPair(dir: d, offset: offset))
    }
    return pairs
}

func simulateDialPositions(
    dialStart: Int, 
    dialMin: Int, 
    dialMax: Int, 
    steps: [InputPair]) -> [DialSimulationStep] {
    precondition(dialMax > dialMin, "invalid dial settings (min less-than max)")
    precondition(dialStart >= dialMin, "invalid dialStart (smaller than min)")
    precondition(dialStart <= dialMax, "invalid dialStart (larger than max)")

    let dialRange = dialMax - dialMin + 1
    let dialStartNormalized = dialStart - dialMin

    var dialPos = dialStartNormalized
    var results: [DialSimulationStep] = [ DialSimulationStep(resultPos: dialPos, zeroCrosses: 0) ]
    for step in steps {
        let offset = Int(step.offset)
        var zeroCrosses = UInt(offset / dialRange)
        let simpleOffset = offset % dialRange
        switch step.dir {
            case Direction.Left:
                zeroCrosses += (dialPos != 0 && simpleOffset >= dialPos) ? 1 : 0
                dialPos += (dialRange - simpleOffset)
                dialPos %= dialRange
            case Direction.Right:
                zeroCrosses += (dialPos != 0 && simpleOffset + dialPos >= dialRange) ? 1 : 0
                dialPos += offset
                dialPos %= dialRange
        } 
        results.append(DialSimulationStep(resultPos: dialPos, zeroCrosses: zeroCrosses))
    }

    return results
}

let inputs = processInput(inputLines)
print("inputs: \(inputs)")

let dialPositions = simulateDialPositions(dialStart: 50, dialMin: 0, dialMax: 99, steps: inputs)
print("dialPositions: \(dialPositions)")

let zeroCount = dialPositions.filter{ $0.resultPos == 0 }.count
print("zeros #: \(zeroCount)")

let zeroCrosses = dialPositions.reduce(0) { $0 + $1.zeroCrosses }
print("zero-crosses #: \(zeroCrosses)")