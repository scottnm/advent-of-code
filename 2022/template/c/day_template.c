#include <stdint.h>
#include <stddef.h>
#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <assert.h>
#include <string.h>
#include <helpers.h>

bool
GetFooFromLines(
    size_t lineCount,
    const char** lines,
    size_t* outFooCount,
    void** outFoos
    )
{
    #pragma message("TODO: impl GetFooFromLines")

    void* foos = NULL;// TODO: = malloc(...)
    if (foos == NULL)
    {
        *outFooCount = 0;
        *outFoos = NULL;
        return false;
    }

    for (size_t i = 0; i < lineCount; ++i)
    {
        const char* line = lines[i];
        (void)line;// TODO: handle
    }

    *outFooCount = lineCount;
    *outFoos = foos;
    return true;
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
        size_t        fooCount;
        void*         foos; // TODO: fix

        success = GetFooFromLines(inputLineCount, (const char**)inputLines, &fooCount, &foos);
        if (success)
        {
            Log("Processing...");
            // TODO: do processing here
            // int result = ...
            #pragma message("TODO: do processing")
            // TODO: report result here
            // Log("result... %i\n", totalScore);
            #pragma message("TODO: report result")
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
