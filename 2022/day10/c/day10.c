#include <stdint.h>
#include <stddef.h>
#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <assert.h>
#include <string.h>
#include <helpers.h>

typedef enum instr_type_t
{
    INSTR_NOOP,
    INSTR_ADD_X,
} instr_type_t;

typedef struct instr_t
{
    instr_type_t type;
    int arg;
} instr_t;

typedef struct cpu_t
{
    int cycle_count;
    int reg_x;
} cpu_t;

bool
GetInstructionsFromLines(
    size_t lineCount,
    const char** lines,
    size_t* outInstructionCount,
    instr_t** outInstructions
    )
{
    instr_t* instructions = T_MALLOC(instr_t, lineCount);
    if (instructions == NULL)
    {
        *outInstructionCount = 0;
        *outInstructions = NULL;
        return false;
    }

    for (size_t i = 0; i < lineCount; ++i)
    {
        const char* line = lines[i];
        if (strcmp(line, "noop") == 0)
        {
            instructions[i] = (instr_t) {
                .type = INSTR_NOOP,
                .arg = 0,
            };
        }
        else if (strncmp(line, "addx ", 5) == 0)
        {
            int arg = atoi(&line[5]);
            instructions[i] = (instr_t) {
                .type = INSTR_ADD_X,
                .arg = arg,
            };
        }
        else
        {
            Log("Invalid instruction line! \"%s\"", line);
            assert(0 && "invalid instr line");
        }
    }

    *outInstructionCount = lineCount;
    *outInstructions = instructions;
    return true;
}

static
inline
bool
ShouldCalcSignalStrength(
    int cycleIndex
    )
{
    switch (cycleIndex)
    {
        case  20:
        case  60:
        case 100:
        case 140:
        case 180:
        case 220:
            return true;
        default:
            // it's unclear if the program is trying to formalize a constraint on this or not.
            // Neither input seems to go to 220 lines. Call this a sanity check.
            assert (cycleIndex < 260);
            return false;
    }
}

static
inline
size_t
CalcSignalStrength(
    int reg_x,
    int cycleIndex
    )
{
    if (ShouldCalcSignalStrength(cycleIndex))
    {
        return cycleIndex * reg_x;
    }
    else
    {
        return 0;
    }
}

size_t
CalcSignalStrengthSumFromProgram(
    size_t instructionCount,
    const instr_t* instructions
    )
{
    cpu_t cpu = { .cycle_count = 0, .reg_x = 1 };
    size_t signalStrengthSum = 0;

    // rough'd out basic simulation based on prompt
    for (size_t i = 0; i < instructionCount; ++i)
    {
        const instr_t instruction = instructions[i];
        switch (instruction.type)
        {
            case INSTR_NOOP:
            {
                // N.B. even though noops don't change the register value, we ahve to check the signal strength
                // in case it changed *AFTER* the last add op
                signalStrengthSum += CalcSignalStrength(cpu.reg_x, cpu.cycle_count + 1);

                cpu.cycle_count += 1;
                break;
            }
            case INSTR_ADD_X:
            {
                // N.B. this problem statement has a weird definition that we calculate signalStrength *DURING*
                // a cycle, but changes to the register happen *AFTER* the cycle.
                // So an add operation which takes 2 cycles would have the same reg value *DURING* its initial state
                // and during the next cycle. We don't even bother checking
                //
                // also cycles are 1-based so the state before we've applied any changes is cycle_count+1
                signalStrengthSum += CalcSignalStrength(cpu.reg_x, cpu.cycle_count + 1);
                signalStrengthSum += CalcSignalStrength(cpu.reg_x, cpu.cycle_count + 2);

                // the reg doesn't change until after 2 cycles
                cpu.cycle_count += 2;
                cpu.reg_x += instruction.arg;
                break;
            }
            default:
            {
                Log("Bad instruction type! %i", instruction.type);
                assert(0 && "bad instruction");
                break;
            }
        }
    }

    // FIXME: maybe I can rewrite this for ease by just putting this at the end of the loop.
    // fence post problem, do one last check after the loop in case we're *DURING* one of the important cycle markers
    // *AFTER* executing all instructions.
    signalStrengthSum += CalcSignalStrength(cpu.reg_x, cpu.cycle_count + 1);
    return signalStrengthSum;
}

static
inline
void
RecordRegisterValue(
    cpu_t cpu,
    int* regValues,
    size_t regValuesBufferSize
    )
{
    if ((size_t)cpu.cycle_count < regValuesBufferSize)
    {
        regValues[cpu.cycle_count] = cpu.reg_x;
    }
}

void
DrawImage(
    size_t instructionCount,
    const instr_t* instructions,
    size_t imgWidth,
    size_t imgHeight,
    char* imgBuffer
    )
{
    // N.B. there's probably a smarter way to do this but let's brute force it with memory lol
    // Basically, I'll execute the instructions and calculate the register value for every cycle
    //
    // This way I can just iterate over the cycles themselves rather than trying to keep track
    // the progress of any given register operation in relation to the sprite position.
    const size_t renderCycleCount = imgWidth * imgHeight;
    int* regValues = T_MALLOC(int, renderCycleCount);
    assert(regValues != NULL);

    {
        cpu_t cpu = { .cycle_count = 0, .reg_x = 1 };

        for (size_t i = 0; i < instructionCount; ++i)
        {
            const instr_t instruction = instructions[i];
            switch (instruction.type)
            {
                case INSTR_NOOP:
                {
                    RecordRegisterValue(cpu, regValues, renderCycleCount);
                    cpu.cycle_count += 1;
                    break;
                }
                case INSTR_ADD_X:
                {
                    // N.B. this problem statement has a weird definition that we calculate signalStrength *DURING*
                    // a cycle, but changes to the register happen *AFTER* the cycle.
                    // So an add operation which takes 2 cycles would have the same reg value *DURING* its initial state
                    // and during the next cycle. We don't even bother checking
                    //
                    // also cycles are 1-based so the state before we've applied any changes is cycle_count+1
                    RecordRegisterValue(cpu, regValues, renderCycleCount);
                    cpu.cycle_count += 1;
                    RecordRegisterValue(cpu, regValues, renderCycleCount);
                    cpu.cycle_count += 1;

                    // the reg doesn't change until after 2 cycles
                    cpu.reg_x += instruction.arg;
                    break;
                }
                default:
                {
                    Log("Bad instruction type! %i", instruction.type);
                    assert(0 && "bad instruction");
                    break;
                }
            }

            // early out even if we have instructions left to process but we're just processing instructions beyond
            // what the screen will even process.
            if (cpu.cycle_count >= renderCycleCount)
            {
                break;
            }
        }

        // FIXME: fence post problem. need to do one more after the loop in case after the last instruction is an interesting value)
        RecordRegisterValue(cpu, regValues, renderCycleCount);
    }

    // Render the register values to the screen
    for (size_t r = 0; r < imgHeight; ++r)
    {
        char* rowImgBuffer = &imgBuffer[r * imgWidth];
        const int* rowRegValues = &regValues[r * imgWidth];
        for (int c = 0; c < (int)imgWidth; ++c)
        {
            int spritePos = rowRegValues[c];
            if (c >= (spritePos - 1) && c <= (spritePos + 1))
            {
                rowImgBuffer[c] = '#';
            }
            else
            {
                rowImgBuffer[c] = '.';
            }
        }
    }

    free(regValues);
}

void
DisplayScreenBuffer(
    size_t width,
    size_t height,
    const char* screenBuffer
    )
{
    for (size_t r = 0; r < height; ++r)
    {
        const char* screenBufferOffset = &screenBuffer[r * width];
        printf("%.*s\n", (int)width, screenBufferOffset);
    }
}

void
PrintUsage(const char* prog)
{
    Log("%s [input_file]", prog);
}

int
main(int argc, const char** argv)
{
    if (argc < 2)
    {
        PrintUsage(argv[0]);
        return 1;
    }

    const char* inputFile = argv[1];

    bool success;
    size_t inputLineCount;
    char** inputLines;

    Log("Reading inputing lines...");
    success = ReadInputLines(inputFile, &inputLineCount, &inputLines);
    if (success)
    {
        Log("Converting lines to problem data");
        size_t        instructionCount;
        instr_t*      instructions;

        success = GetInstructionsFromLines(inputLineCount, (const char**)inputLines, &instructionCount, &instructions);
        if (success)
        {
            Log("Processing...");
            int signalStrengthSum = CalcSignalStrengthSumFromProgram(instructionCount, instructions);
            Log("signal strength sum... %i\n", signalStrengthSum);

            Log("Rendering...");
            #define SCREEN_WIDTH 40
            #define SCREEN_HEIGHT 6
            char screenBuffer[SCREEN_WIDTH * SCREEN_HEIGHT];
            DrawImage(instructionCount, instructions, SCREEN_WIDTH, SCREEN_HEIGHT, screenBuffer);
            DisplayScreenBuffer(SCREEN_WIDTH, SCREEN_HEIGHT, screenBuffer);
        }
        else
        {
            Log("Failed to convert lines!");
        }

        free(inputLines);
    }
    else
    {
        Log("Failed to ReadInputLines!");
    }
}
