import util

class BatteryBank {
    let batteryJoltages: [UInt]
    init(_ joltages: [UInt]) {
        self.batteryJoltages = joltages
    }

enum InputError: Error {
    case IncorrectInputFormat(_ desc: String)
}

func processInput(_ lines: [String]) throws -> [BatteryBank] {
    var banks: [BatteryBank] = []
    for line in lines {
        var batteryJoltages: [UInt] = []
        for c in line {
            guard let joltageVal: Int = c.wholeNumberValue else {
                throw InputError.IncorrectInputFormat("Input line had invalid value '\(c)': \(line)")
            }

            batteryJoltages.append(UInt(joltageVal))
        }
        banks.append(BatteryBank(batteryJoltages))
    }
    return banks
}

func findLargestJoltageFromBankPt1(_ bank: BatteryBank) -> UInt {
    precondition(bank.batteryJoltages.count >= 2)

    var maxFromLeft = [UInt](repeating: 0, count: bank.batteryJoltages.count)
    maxFromLeft[0] = bank.batteryJoltages[0]
    var maxFromRight = [UInt](repeating: 0, count: bank.batteryJoltages.count)
    maxFromRight[maxFromRight.count - 1] = bank.batteryJoltages[bank.batteryJoltages.count - 1]

    for i in 1..<bank.batteryJoltages.count {
        maxFromLeft[i] = max(maxFromLeft[i-1], bank.batteryJoltages[i])
    }

    for i in (0...bank.batteryJoltages.count-2).reversed() {
        maxFromRight[i] = max(maxFromRight[i+1], bank.batteryJoltages[i])
    }

    var largestDigits: (UInt, UInt) = (0, 0)
    for i in 0...(bank.batteryJoltages.count-2) {
        let leftDigit = maxFromLeft[i]
        let rightDigit = maxFromRight[i+1]
        if (leftDigit > largestDigits.0) ||
           (leftDigit == largestDigits.0 && rightDigit > largestDigits.1) {
            largestDigits = (leftDigit, rightDigit)
        }
    }

    return largestDigits.0 * 10 + largestDigits.1
}

func prependDigit(digit: UInt, rightHandDigits: UInt) -> UInt {
    assert(1 <= digit && digit <= 9)
    assert(rightHandDigits >= 0)

    var digitShiftMultiplier: UInt = 1
    var rightHandDigitCounter = rightHandDigits
    while rightHandDigitCounter > 0 {
        rightHandDigitCounter /= 10
        digitShiftMultiplier *= 10
    }

    return digit * digitShiftMultiplier + rightHandDigits
}


func findLargestJoltageFromBankPt2(_ bank: BatteryBank) -> UInt {
    var memo: [BatteryBankJoltageMemoKey: UInt] = Dictionary()
    return findLargestJoltageFromBankPt2Helper(
        bank,
        withMemo: &memo,
        fromIdx: 0,
        withReqSize: 12)
}

struct BatteryBankJoltageMemoKey: Hashable {
    let listStartIdx: Int
    let sequenceLength: Int

    static func == (lhs: BatteryBankJoltageMemoKey, rhs: BatteryBankJoltageMemoKey) -> Bool {
        return lhs.listStartIdx == rhs.listStartIdx && lhs.sequenceLength == rhs.sequenceLength
    }


    func hash(into hasher: inout Hasher) {
        hasher.combine(listStartIdx)
        hasher.combine(sequenceLength)
    }
}

func findLargestJoltageFromBankPt2Helper(
    _ bank: BatteryBank,
    withMemo memo: inout [BatteryBankJoltageMemoKey: UInt],
    fromIdx bankStartIdx: Int,
    withReqSize requiredSequenceSize: Int
    ) -> UInt {
    precondition(bankStartIdx < bank.batteryJoltages.count)
    precondition(requiredSequenceSize <= (bank.batteryJoltages.count - bankStartIdx))

    let memoKey = BatteryBankJoltageMemoKey(
        listStartIdx: bankStartIdx,
        sequenceLength: requiredSequenceSize)

    if let memodValue = memo[memoKey] {
        return memodValue
    }

    var maxValue: UInt = 0
    for i in bankStartIdx ..< (bank.batteryJoltages.count - (requiredSequenceSize - 1)) {
        let newCandidateDigit = bank.batteryJoltages[i]
        var maxValueCandidate: UInt
        if requiredSequenceSize > 1 {
            let newCandidateLeastSignificantDigits = findLargestJoltageFromBankPt2Helper(
                bank,
                withMemo: &memo,
                fromIdx: i + 1,
                withReqSize: requiredSequenceSize - 1)
            maxValueCandidate = prependDigit(
                digit: newCandidateDigit,
                rightHandDigits: newCandidateLeastSignificantDigits)
        } else {
            maxValueCandidate = newCandidateDigit
        }
        maxValue = max(maxValue, maxValueCandidate)
    }

    memo[memoKey] = maxValue
    return maxValue
}

func run(inputFilePath: String) throws {
    let inputLines = try util.readTextFileAsLines(atPath: inputFilePath)
    let banks = try processInput(inputLines)
    print("banks: \(banks)")

    util.timeSection("pt1") {
        let largestJoltages = banks.map(){ findLargestJoltageFromBankPt1($0) }
        let sumLargestJoltages = largestJoltages.reduce(0, +)
        print("pt1. total output joltage \(sumLargestJoltages)")
    }

    util.timeSection("pt1") {
        let largestJoltages = banks.map(){ findLargestJoltageFromBankPt2($0) }
        let sumLargestJoltages = largestJoltages.reduce(0, +)
        print("pt2. total output joltage \(sumLargestJoltages)")
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