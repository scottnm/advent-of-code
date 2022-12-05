#include <stdint.h>
#include <stddef.h>
#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <assert.h>
#include <string.h>
#include <helpers.h>

// I use signed characters to index into this array so disable the char-subscripts warning.
// I am well aware of the signedness of chars :)
#pragma clang diagnostic ignored "-Wchar-subscripts"

typedef struct compartment_t {
    // Each item is represented by an ascii character.
    // The key into this map is the ascii character itself.
    // The value is the count of times that item/character has appeared.
    int itemCounts[127];
} compartment_t;

typedef struct rucksack_t {
    compartment_t compartments[2];
} rucksack_t;

bool
GetRucksackFromLines(
    size_t lineCount,
    const char** lines,
    size_t* outRucksackCount,
    rucksack_t** outRucksacks
    )
{
    rucksack_t* rucksacks = (rucksack_t*)malloc(lineCount * sizeof(rucksack_t));
    if (rucksacks == NULL)
    {
        *outRucksackCount = 0;
        *outRucksacks = NULL;
        return false;
    }

    for (size_t line_i = 0; line_i < lineCount; ++line_i)
    {
        const char* line = lines[line_i];

        size_t lineLength = strlen(line);
        assert(lineLength % 2 == 0); // each line should be of even length since both compartments are always even sized.

        rucksack_t* rucksack = &rucksacks[line_i];
        memset(rucksack, 0, sizeof(*rucksack));

        // The first half of the line is all of the items in compartment 1
        for (size_t i = 0; i < lineLength / 2; ++i)
        {
            char item = line[i];
            rucksack->compartments[0].itemCounts[item] += 1;
        }

        // The second half of the line is all of the items in compartment 2
        for (size_t i = lineLength / 2; i < lineLength; ++i)
        {
            char item = line[i];
            rucksack->compartments[1].itemCounts[item] += 1;
        }
    }

    *outRucksackCount = lineCount;
    *outRucksacks = rucksacks;
    return true;
}

bool
ItemOutOfPlaceInRucksack(
    const rucksack_t* rucksack,
    char c
    )
{
    return (rucksack->compartments[0].itemCounts[c] != 0) &&
           (rucksack->compartments[1].itemCounts[c] != 0);
}

uint32_t
CalculateItemPriority(char c)
{
    return c >= 'a' ?
        // items 'a'->'z' have priority 1->26
        (c - 'a' + 1) :
        // items 'A'->'Z' have priority 27->52
        (c - 'A' + 27);
}

uint32_t
CalculateRucksackMisalignmentPriority(
    const rucksack_t* rucksack
    )
{
    // Check 'A'-'Z' and 'a'-'z'.
    for (char c = 'A'; c <= 'Z'; ++c)
    {
        if (ItemOutOfPlaceInRucksack(rucksack, c))
        {
            return CalculateItemPriority(c);
        }
    }
    for (char c = 'a'; c <= 'z'; ++c)
    {
        if (ItemOutOfPlaceInRucksack(rucksack, c))
        {
            return CalculateItemPriority(c);
        }
    }

    // All rucksacks should have some misalignment
    assert(false);
    return 0;
}

uint32_t
SumRucksackMisalignmentPriorities(
    size_t rucksackCount,
    const rucksack_t* rucksacks
    )
{
    uint32_t prioritySum = 0;
    for (size_t i = 0; i < rucksackCount; ++i)
    {
        prioritySum += CalculateRucksackMisalignmentPriority(&rucksacks[i]);
    }
    return prioritySum;
}

bool
DoesItemAppearInAllElfRucksacks(
    char itemId,
    size_t elvesInGroup,
    const rucksack_t* rucksacks
    )
{
    for (size_t i = 0; i < elvesInGroup; ++i)
    {
        if (rucksacks[i].compartments[0].itemCounts[itemId] == 0 &&
            rucksacks[i].compartments[1].itemCounts[itemId] == 0)
        {
            return false;
        }
    }
    return true;
}

char
FindBadgeItemIdFromElfGroup(
    size_t elvesInGroup,
    const rucksack_t* rucksacks
    )
{
    // Check 'A'-'Z' and 'a'-'z' for an item which appears
    // in all elf rucksacks. This is our badge item id.
    for (char c = 'A'; c <= 'Z'; ++c)
    {
        if (DoesItemAppearInAllElfRucksacks(c, elvesInGroup, rucksacks))
        {
            return c;
        }
    }
    for (char c = 'a'; c <= 'z'; ++c)
    {
        if (DoesItemAppearInAllElfRucksacks(c, elvesInGroup, rucksacks))
        {
            return c;
        }
    }

    assert(false); // we didn't find a badge item
    return 0;
}

uint32_t
SumElfGroupBadgePriorities(
    size_t rucksackCount,
    const rucksack_t* rucksacks
    )
{
    uint32_t prioritySum = 0;

    static const size_t elvesPerGroup = 3;
    assert(rucksackCount % elvesPerGroup == 0);

    for (size_t i = 0; i < rucksackCount; i += elvesPerGroup)
    {
        char badgeItemId = FindBadgeItemIdFromElfGroup(elvesPerGroup, &rucksacks[i]);
        prioritySum += CalculateItemPriority(badgeItemId);
    }

    return prioritySum;
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
        size_t        rucksackCount;
        rucksack_t*   rucksacks;

        success = GetRucksackFromLines(inputLineCount, (const char**)inputLines, &rucksackCount, &rucksacks);
        if (success)
        {
            Log("Processing pt 1...");
            uint32_t resultPt1 = SumRucksackMisalignmentPriorities(rucksackCount, rucksacks);
            Log("priority sum... %u\n", resultPt1);

            Log("Processing pt 2...");
            uint32_t resultPt2 = SumElfGroupBadgePriorities(rucksackCount, rucksacks);
            Log("badge priority sum... %u\n", resultPt2);
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
