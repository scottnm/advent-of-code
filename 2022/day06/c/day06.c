#include <stdint.h>
#include <stddef.h>
#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <assert.h>
#include <string.h>
#include <helpers.h>

static
inline
size_t
FirstNCharactersAreDistinct(
    size_t n,
    const char* buffer
    )
{
    for (size_t i = 0; i < n-1; ++i)
    {
        for (size_t j = i+1; j < n; ++j)
        {
            if (buffer[i] == buffer[j])
            {
                return false;
            }
        }
    }

    return true;
}

static
inline
size_t
FindEndOfFirstNDistinctCharacters(
    size_t n,
    const char* buffer
    )
{
    const size_t bufferLength = strlen(buffer);
    assert(bufferLength >= n);

    for (size_t i = (n-1); i < bufferLength; ++i)
    {
        if (FirstNCharactersAreDistinct(n, &buffer[i-(n-1)]))
        {
            return i;
        }
    }

    assert(false);
    return 0;
}

size_t
FindStartOfPacketEndIndex(
    const char* buffer
    )
{
    return FindEndOfFirstNDistinctCharacters(4, buffer);
}

size_t
FindStartOfMessageEndIndex(
    const char* buffer
    )
{
    return FindEndOfFirstNDistinctCharacters(14, buffer);
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
        // Log("Converting lines to problem data");
        // size_t        fooCount;
        // void*         foos; // TODO: fix

        // success = GetFooFromLines(inputLineCount, (const char**)inputLines, &fooCount, &foos);
        if (success)
        {
            for (size_t i = 0; i < inputLineCount; ++i)
            {
                const char* datastreamBuffer = inputLines[i];
                Log("Processing stream #%zu... ", i);
                size_t startOfPacketEndIndex = FindStartOfPacketEndIndex(datastreamBuffer);
                Log("    stream marker end=%zu; %.*s", startOfPacketEndIndex+1, (int)startOfPacketEndIndex+1, datastreamBuffer);
                size_t startOfMessageEndIndex = FindStartOfMessageEndIndex(datastreamBuffer);
                Log("    stream marker end=%zu; %.*s", startOfMessageEndIndex+1, (int)startOfMessageEndIndex+1, datastreamBuffer);
            }
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

/*
 * ORIGINAL SOLUTION INCLUDED FOR LULZ
 *
 * it's a mess of unrolled code and gotos but I find it pretty funny that this is what I started with thinking
 * a more generalized solution would be too slow
 *
size_t
FindStartOfPacketEndIndex(
    const char* datastreamBuffer
    )
{
    const size_t datastreamLength = strlen(datastreamBuffer);
    assert(datastreamLength >= 4);

    for (size_t i = 3; i < datastreamLength; ++i)
    {
        const char c0 = datastreamBuffer[i-3];
        const char c1 = datastreamBuffer[i-2];
        const char c2 = datastreamBuffer[i-1];
        const char c3 = datastreamBuffer[i-0];

        bool foundEndMarker =
            c0 != c1 &&
            c0 != c2 &&
            c0 != c3 &&
            c1 != c2 &&
            c1 != c3 &&
            c2 != c3;

        if (foundEndMarker)
        {
            return i;
        }
    }

    assert(false);
    return 0;
}

size_t
FindStartOfMessageEndIndex(
    const char* datastreamBuffer
    )
{
    const size_t datastreamLength = strlen(datastreamBuffer);
    assert(datastreamLength >= 14);

    for (size_t i = 13; i < datastreamLength; ++i)
    {
        for (size_t j = i-13; j < i; ++j)
        {
            for (size_t k = j+1; k <= i; ++k)
            {
                if (datastreamBuffer[j] == datastreamBuffer[k])
                {
                    goto notequal;
                }
            }
        }
        return i;

        notequal: continue;
    }

    assert(false);
    return 0;
}
*/
