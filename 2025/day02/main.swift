import util

struct Range {
    let lower: UInt
    let upper: UInt

    static func sorter(r1: Range, r2: Range) -> Bool {
        if r1.lower < r2.lower {
            return true
        } else if r1.lower == r2.lower {
            return r1.upper <= r2.upper
        } else {
            return false
        }
    }

    func contains(_ n: UInt) -> Bool {
        return n >= self.lower && n <= self.upper
    }
}

func anyRangeContains(_ ranges: [Range], _ n: UInt) -> Bool {
    for r in ranges {
        if r.contains(n) {
            return true
        }
    }
    return false
}

enum InputError: Error {
    case IncorrectInputFormat(_ desc: String)
}

func processInput(_ lines: [String]) throws -> [Range] {
    if lines.count != 1 {
        throw InputError.IncorrectInputFormat("Expected only one line of input")
    }
    let line = lines[0]
    let RANGE_RGX = /(\d+)-(\d+)(,?)/
    let matches = line.matches(of: RANGE_RGX)
    if matches.count == 0{
        throw InputError.IncorrectInputFormat("Input was not a comma-separated range list: \(line)")
    }

    var ranges: [Range] = []
    for match in matches {
        let rangeLower = UInt(match.1)!
        let rangeUpper = UInt(match.2)!
        ranges.append(Range(lower: rangeLower, upper: rangeUpper))
    }
    return ranges
}

func calcShiftMultiplier(_ n: UInt) -> UInt {
    var multiplier: UInt = 10
    while n >= multiplier {
        multiplier *= 10
    }
    return multiplier
}

func repeatNumber(_ n: UInt) -> UInt {
    let shiftMultiplier = calcShiftMultiplier(n)
    return (shiftMultiplier * n) + n
}

func findInvalidIdsPt1(_ idRanges: [Range]) -> [UInt] {
    if idRanges.count == 0 {
        return []
    }

    let idRangeMinMin = idRanges.map{ $0.lower }.min()!
    let idRangeMaxMax = idRanges.map{ $0.upper }.max()!
    assert(idRangeMinMin <= idRangeMaxMax)

    var invalidIds: [UInt] = []
    for n in 1...UInt.max {
        let invalidIdCandidate = repeatNumber(n)
        if invalidIdCandidate < idRangeMinMin {
            continue
        } else if invalidIdCandidate > idRangeMaxMax {
            break
        }

        if (anyRangeContains(idRanges, invalidIdCandidate)) {
            invalidIds.append(invalidIdCandidate)
        }
    }

    return invalidIds
}

func findInvalidIdsPt2(_ idRanges: [Range]) -> [UInt] {
    if idRanges.count == 0 {
        return []
    }

    let idRangeMinMin = idRanges.map{ $0.lower }.min()!
    let idRangeMaxMax = idRanges.map{ $0.upper }.max()!
    assert(idRangeMinMin <= idRangeMaxMax)

    var invalidIds: Set<UInt> = []
    for n in 1...UInt.max {
        let shiftMultiplier = calcShiftMultiplier(n)
        var invalidIdCandidate = (n * shiftMultiplier) + n
        if invalidIdCandidate > idRangeMaxMax {
            break
        }

        while invalidIdCandidate < idRangeMaxMax {
            if invalidIdCandidate >= idRangeMinMin {
                if (anyRangeContains(idRanges, invalidIdCandidate)) {
                    invalidIds.insert(invalidIdCandidate)
                }
            }

            // add the next iteration of the pattern to get our next id candidate
            // e.g. pattern=12, invalidIdCandidate=1212, new invalidIdCandidate=121212
            invalidIdCandidate = (invalidIdCandidate * shiftMultiplier) + n
        }
    }

    return Array(invalidIds)
}

func run(inputFilePath: String) throws {
    let inputLines = try util.readTextFileAsLines(atPath: inputFilePath)
    let idRanges = try processInput(inputLines)
    print("idRanges: \(idRanges)")
    let sortedIdRanges = idRanges.sorted(by: Range.sorter)
    print("sortedIdRanges: \(sortedIdRanges)")

    do {
        let invalidIds = findInvalidIdsPt1(sortedIdRanges)
        // FIXME: replace with debug logger calls
        print("invalid ids pt1: \(invalidIds)")
        let invalidIdSum = invalidIds.reduce(0, +)
        print("invalid ids pt1 sum: \(invalidIdSum)")
    }

    do {
        let invalidIds = findInvalidIdsPt2(sortedIdRanges)
        // FIXME: replace with debug logger calls
        print("invalid ids pt2: \(invalidIds.sorted())")
        let invalidIdSum = invalidIds.reduce(0, +)
        print("invalid ids pt2 sum: \(invalidIdSum)")
    }
}

func main() {
    let args = CommandLine.arguments
    if args.count < 2 {
        fatalError("need input file argument")
    }

    do {
        try run(inputFilePath: args[1])
    } catch {
        fatalError(error.localizedDescription)
    }
}

main()