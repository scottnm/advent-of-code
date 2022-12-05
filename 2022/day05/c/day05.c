#include <stdint.h>
#include <stddef.h>
#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <assert.h>
#include <string.h>
#include <helpers.h>

typedef struct stack_t {
    uint32_t crateCount;
    char* crateStack;
} stack_t;

typedef struct instr_t {
    uint32_t moveCount;
    uint32_t srcStack;
    uint32_t destStack;
} instr_t;

void
SplitLinesIntoStacksAndInstructions(
    size_t lineCount,
    const char** lines,
    size_t* stackLineCount,
    const char*** stackLines,
    size_t* instrLineCount,
    const char*** instrLines
    )
{
    for (size_t i = 0; i < lineCount; ++i)
    {
        if (strcmp(lines[i], "") == 0)
        {
            *stackLineCount = i;
            *stackLines = &lines[0];
            *instrLineCount = lineCount - i - 1;
            *instrLines = &lines[i+1];
            return;
        }
    }

    Log("Failed to split lines into stacks and instructions!");
    assert(false);
}

size_t
CountStacks(
    size_t stackLineCount,
    const char** stackLines
    )
{
    const char* stackIdLine = stackLines[stackLineCount - 1]; // the last stack line is where the ids are stored

    // cheat a little
    // ids are stored like the following line
    // " 1   2   3   4   5   6   7   8   9 "
    // each stack's id takes up 4 characters.
    // So rather than parsing and interpetting the numbers on this line, we'll just cheat and divide by 4
    return (strlen(stackIdLine) / 4) + 1;
}

stack_t*
BuildStacks(
    size_t stackLineCount,
    const char** stackLines,
    size_t maxCrateCount,
    size_t stackCount
    )
{
    const size_t crateBufferSize = (maxCrateCount + 1) * sizeof(char); // maxCrateCount + 1 so each crate can have a null at the end
    const size_t eachStackSize = sizeof(stack_t) + crateBufferSize;
    const size_t stackBufferSize = stackCount * eachStackSize;
    stack_t* stacks = (stack_t*)malloc(stackBufferSize);
    if (stacks == NULL)
    {
        return NULL;
    }

    // zero out the allocated memory for safety
    // This also has the benefit of zeroing out all of the crateCount fields
    memset(stacks, 0, stackBufferSize);

    // initialize each crate buffer area
    char* crateBufferArea = (char*)&stacks[stackCount];
    for (int i = 0; i < stackCount; ++i)
    {
        // stacks[i].crateCount initialized later
        stacks[i].crateStack = &crateBufferArea[i * crateBufferSize];
    }

    for (int i = 0; i < (stackLineCount - 1); ++i)
    {
        const char* stackLine = stackLines[i];

        // crates are in reverse order. crates higher in the stack will be in earlier lines
        //|    [D]       <--- (crateIndex == 2) && (i == 0)
        //|[N] [C]
        //|[Z] [M] [P]
        const size_t crateIndex = ((stackLineCount - 1) - 1) - i;

        for (size_t stackColumnIndex = 0; stackColumnIndex < stackCount; ++stackColumnIndex)
        {
            // Each crate column takes up 4 characters. The crate value is always the 2nd character in those 4 characters.
            size_t stackLineCrateCharacterIndex = (stackColumnIndex * 4) + 1;
            char crateCharacter = stackLine[stackLineCrateCharacterIndex];
            if (crateCharacter != ' ')
            {
                if (!(crateCharacter >= 'A' && crateCharacter <= 'Z'))
                {
                    Log("Unexpected char! %c (stackLine=%s)", crateCharacter, stackLine);
                    assert(0 && "unexpected char processing crates");
                }
                // Log("set stacks[%zu].crateStack[%zu] = %c", stackColumnIndex, crateIndex, crateCharacter);
                stacks[stackColumnIndex].crateStack[crateIndex] = crateCharacter;
            }
        }
    }

    for (int i = 0; i < stackCount; ++i)
    {
        // Convenient hack! because each crate stack is filled contiguously, filled with A-Z chars, and is initially zero'd
        // we can just use strlen to count the number of characters in the stack
        stacks[i].crateCount = strlen(stacks[i].crateStack);
    }

    return stacks;
}

void
PrintStacks(
    size_t stackCount,
    const stack_t* stacks
    )
{
    for (size_t i = 0; i < stackCount; ++i)
    {
        printf("stack[%zu] = ", i);
        for (uint32_t j = 0; j < stacks[i].crateCount; ++j)
        {
            if (stacks[i].crateStack[j] != 0)
            {
                printf("[%c]", stacks[i].crateStack[j]);
            }
            else
            {
                printf("[_]");
            }
        }
        printf(" (len=%u)\n", stacks[i].crateCount);
    }
}

bool
GetStacksFomLines(
    size_t lineCount,
    const char** lines,
    size_t* outStackCount,
    stack_t** outStacks,
    size_t* outInstructionCount,
    instr_t** outInstructions
    )
{
    // default initialize the out params to zero in case of failure
    *outStackCount = 0;
    *outStacks = NULL;
    *outInstructionCount = 0;
    *outInstructions = NULL;

    size_t stackLineCount;
    const char** stackLines;
    size_t instructionLineCount;
    const char** instructionLines;
    SplitLinesIntoStacksAndInstructions(lineCount, lines, &stackLineCount, &stackLines, &instructionLineCount, &instructionLines);

    // Log("stacklinecount = %zu; first stackline = \"%s\"; instructionLine = %zu; first instructionline = \"%s\"",
    //     stackLineCount,
    //     stackLines[0],
    //     instructionLineCount,
    //     instructionLines[0]); // sanity check

    const size_t stackCount = CountStacks(stackLineCount, stackLines);

    // there's one crate on each stack line except the last (that's where the ids are stored)
    // we'll be greedy with memory and reserve room for each stack to hold as many crates as possible across all stacks
    const size_t maxCrateCount = stackCount * (stackLineCount - 1);

    stack_t* stacks = BuildStacks(stackLineCount, stackLines, maxCrateCount, stackCount);
    if (stacks == NULL)
    {
        return false;
    }

    *outStackCount = stackCount;
    *outStacks = stacks;

    //
    // Generate instructions from text input
    //
    instr_t* instructions = (instr_t*)malloc(instructionLineCount * sizeof(instr_t));
    if (instructions == NULL)
    {
        return false;
    }

    for (size_t i = 0; i < instructionLineCount; ++i)
    {
        const char* instructionLine = instructionLines[i];
        sscanf(instructionLine, "move %u from %u to %u",
            &instructions[i].moveCount,
            &instructions[i].srcStack,
            &instructions[i].destStack);
    }

    *outInstructionCount = instructionLineCount;
    *outInstructions = instructions;
    return true;
}

void
ExecuteInstructions_CrateMover9000(
    size_t instructionCount,
    const instr_t* instructions,
    size_t stackCount,
    stack_t* stacks
    )
{
    for (size_t instructionIndex = 0; instructionIndex < instructionCount; ++instructionIndex)
    {

        instr_t instruction = instructions[instructionIndex];
        for (uint32_t move = 0; move < instruction.moveCount; ++move)
        {
            // the instructions in the source file are 1-based rather than 0-based
            stack_t* srcStack = &stacks[instruction.srcStack - 1];
            stack_t* destStack = &stacks[instruction.destStack - 1];

            // move the crate from src->dest
            destStack->crateStack[destStack->crateCount] = srcStack->crateStack[srcStack->crateCount - 1];
            destStack->crateCount += 1;

            // erase the crate from src
            srcStack->crateStack[srcStack->crateCount - 1] = '\0';
            srcStack->crateCount -= 1;
        }

    }
}

void
ExecuteInstructions_CrateMover9001(
    size_t instructionCount,
    const instr_t* instructions,
    size_t stackCount,
    stack_t* stacks
    )
{
    for (size_t instructionIndex = 0; instructionIndex < instructionCount; ++instructionIndex)
    {
        instr_t instruction = instructions[instructionIndex];
        stack_t* srcStack = &stacks[instruction.srcStack - 1];
        stack_t* destStack = &stacks[instruction.destStack - 1];

        // copy the crates over in order from the src stack to the dest stack
        memcpy(
            &destStack->crateStack[destStack->crateCount],
            &srcStack->crateStack[srcStack->crateCount - instruction.moveCount],
            instruction.moveCount * sizeof(srcStack->crateStack[0]));
        destStack->crateCount += instruction.moveCount;

        // clear out the old crates from the src stack
        memset(
            &srcStack->crateStack[srcStack->crateCount - instruction.moveCount],
            0,
            instruction.moveCount * sizeof(srcStack->crateStack[0]));
        srcStack->crateCount -= instruction.moveCount;
    }
}

void
ReadStackTops(
    size_t stackCount,
    stack_t* stacks,
    size_t bufSize,
    char* buf,
    size_t* bufSizeUsed
    )
{
    for (size_t i = 0; i < stackCount; ++i)
    {
        buf[i] = stacks[i].crateStack[stacks[i].crateCount - 1];
    }

    *bufSizeUsed = stackCount;
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
    if (!success)
    {
        Log("Failed to ReadInputLines!");
        inputLines = NULL;
    }

    if (success)
    {
        Log("Converting lines to problem data for CM9000");
        size_t    stackCount;
        stack_t*  stacks;
        size_t    instructionCount;
        instr_t*  instructions;

        success = GetStacksFomLines(
            inputLineCount,
            (const char**)inputLines,
            &stackCount,
            &stacks,
            &instructionCount,
            &instructions);
        if (success)
        {
            // Log("\n\nstacks before:");
            // PrintStacks(stackCount, stacks);
            // Log("\n\n");

            Log("Executing stack instructions...");
            ExecuteInstructions_CrateMover9000(instructionCount, instructions, stackCount, stacks);

            // Log("\n\nstacks after:");
            // PrintStacks(stackCount, stacks);
            // Log("\n\n");

            Log("Reading results...");
            size_t bufSizeUsed;
            char buf[1024];
            ReadStackTops(stackCount, stacks, ARRAYSIZE(buf), buf, &bufSizeUsed);

            Log("result... \"%.*s\"\n", (int)bufSizeUsed, buf);
        }
        else
        {
            Log("Failed to convert lines!");
        }
    }

    // this block of code is identical to the above block except it uses the CM9001 instructions.
    if (success)
    {
        Log("Converting lines to problem data for CM9001");
        size_t    stackCount;
        stack_t*  stacks;
        size_t    instructionCount;
        instr_t*  instructions;

        success = GetStacksFomLines(
            inputLineCount,
            (const char**)inputLines,
            &stackCount,
            &stacks,
            &instructionCount,
            &instructions);
        if (success)
        {
            // Log("\n\nstacks before:");
            // PrintStacks(stackCount, stacks);
            // Log("\n\n");

            Log("Executing stack instructions...");
            ExecuteInstructions_CrateMover9001(instructionCount, instructions, stackCount, stacks);

            // Log("\n\nstacks after:");
            // PrintStacks(stackCount, stacks);
            // Log("\n\n");

            Log("Reading results...");
            size_t bufSizeUsed;
            char buf[1024];
            ReadStackTops(stackCount, stacks, ARRAYSIZE(buf), buf, &bufSizeUsed);

            Log("result... \"%.*s\"\n", (int)bufSizeUsed, buf);
        }
        else
        {
            Log("Failed to convert lines!");
        }
    }

    if (inputLines != NULL)
    {
        free(inputLines);
    }

    return 0;
}
