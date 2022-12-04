#include <stdint.h>
#include <stddef.h>
#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <assert.h>
#include <string.h>
#include <helpers.h>

typedef struct section_assignment_t {
    uint32_t start;
    uint32_t end;
} section_assignment_t;

bool
GetSectionAssignmentFromLines(
    size_t lineCount,
    const char** lines,
    size_t* outSectionAssignmentCount,
    section_assignment_t** outSectionAssignments
    )
{
    section_assignment_t* sectionAssignments =
        (section_assignment_t*)malloc(lineCount * 2 * sizeof(section_assignment_t));

    if (sectionAssignments == NULL)
    {
        *outSectionAssignmentCount = 0;
        *outSectionAssignments = NULL;
        return false;
    }

    for (size_t i = 0; i < lineCount; ++i)
    {
        const char* line = lines[i];
        section_assignment_t* elf1Assignment = &sectionAssignments[(i * 2)];
        section_assignment_t* elf2Assignment = &sectionAssignments[(i * 2) + 1];

        sscanf(
            line,
            "%u-%u,%u-%u",
            &elf1Assignment->start, &elf1Assignment->end,
            &elf2Assignment->start, &elf2Assignment->end);
    }

    *outSectionAssignmentCount = lineCount * 2;
    *outSectionAssignments = sectionAssignments;
    return true;
}

bool
DoSectionAssignmentsOverlapCompletely(
    section_assignment_t assignment1,
    section_assignment_t assignment2
    )
{
    bool oneCompletelyOverlapsTwo = assignment1.start <= assignment2.start && assignment1.end >= assignment2.end;
    bool twoCompletelyOverlapsOne = assignment2.start <= assignment1.start && assignment2.end >= assignment1.end;
    return oneCompletelyOverlapsTwo || twoCompletelyOverlapsOne;
}

uint32_t
CountAssignmentPairsWithFullOverlap(
    size_t sectionAssignmentCount,
    const section_assignment_t* sectionAssignments
    )
{
    uint32_t fullOverlapCount = 0;

    for (size_t i = 0; i < sectionAssignmentCount; i += 2)
    {
        bool assignmentPairFullOverlaps = DoSectionAssignmentsOverlapCompletely(
            sectionAssignments[i],
            sectionAssignments[i+1]);

        if (assignmentPairFullOverlaps)
        {
            fullOverlapCount += 1;
        }
    }

    return fullOverlapCount;
}

bool
DoSectionAssignmentsOverlapPartially(
    section_assignment_t assignment1,
    section_assignment_t assignment2
    )
{
    bool onePartiallyOverlapsTwo = assignment1.start <= assignment2.start && assignment1.end >= assignment2.start;
    bool twoPartiallyOverlapsOne = assignment2.start <= assignment1.start && assignment2.end >= assignment1.start;
    return onePartiallyOverlapsTwo || twoPartiallyOverlapsOne;
}

uint32_t
CountAssignmentPairsWithPartialOverlap(
    size_t sectionAssignmentCount,
    const section_assignment_t* sectionAssignments
    )
{
    uint32_t partialOverlapCount = 0;

    for (size_t i = 0; i < sectionAssignmentCount; i += 2)
    {
        bool assignmentPairPartialOverlaps = DoSectionAssignmentsOverlapPartially(
            sectionAssignments[i],
            sectionAssignments[i+1]);

        if (assignmentPairPartialOverlaps)
        {
            partialOverlapCount += 1;
        }
    }

    return partialOverlapCount;
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
        size_t                sectionAssignmentCount;
        section_assignment_t* sectionAssignments;

        success = GetSectionAssignmentFromLines(inputLineCount, (const char**)inputLines, &sectionAssignmentCount, &sectionAssignments);
        if (success)
        {
            Log("Processing pt1...");
            uint32_t fullOverlapCount = CountAssignmentPairsWithFullOverlap(sectionAssignmentCount, sectionAssignments);
            Log("full overlap pair counts... %u\n", fullOverlapCount);

            Log("Processing pt2...");
            uint32_t partialOverlapCount = CountAssignmentPairsWithPartialOverlap(sectionAssignmentCount, sectionAssignments);
            Log("partial overlap pair counts... %u\n", partialOverlapCount);
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
