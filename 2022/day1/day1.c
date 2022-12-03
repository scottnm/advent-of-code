#include <stdint.h>
#include <stddef.h>
#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <helpers.h>

typedef struct ElfPack
{
    size_t itemCount;
    const int* items;
} ElfPack;

void
PrintUsage()
{
    Log("day1 [input_file]");
}

bool
LineHasNonWhitespaceChars(
    const char* line
    )
{
    if (line == NULL)
    {
        return false;
    }

    while (true)
    {
        switch (*line)
        {
            case '\t':
            case '\n':
            case '\r':
            case ' ':
                line++;
                continue;
            case '\0':
                return false;
            default:
                return true;
        }
    }
}

bool
GetElfPacksFromLines(
    size_t lineCount,
    const char** lines,
    size_t* outElfCount,
    ElfPack** outElfPacks
    )
{
    // For simplicity, allocate more than enough space to store all elves and items
    void* elfPackBuffer = malloc(lineCount * (sizeof(ElfPack) + sizeof(int)));
    if (elfPackBuffer == NULL)
    {
        *outElfCount = 0;
        *outElfPacks = NULL;
        return false;
    }

    ElfPack* elfPacks = elfPackBuffer;
    int* items = (int*)(((char*)elfPackBuffer) + (sizeof(ElfPack) * lineCount));

    size_t elfCount = 0;
    ElfPack* nextElf = elfPacks;
    int* nextItem = items;

    bool processingItems = false;
    bool processingEmptyLines = true;
    for (size_t i = 0; i < lineCount; ++i)
    {
        const char* line = lines[i];
        if (LineHasNonWhitespaceChars(line))
        {
            if (!processingItems)
            {
                processingItems = true;
                nextElf->itemCount = 0;
                nextElf->items = nextItem;
                elfCount += 1;
            }

            sscanf(line, "%i", nextItem);
            nextItem++;
            nextElf->itemCount += 1;
        }
        else
        {
            if (processingItems)
            {
                processingItems = false;
                nextElf++;
            }
        }
    }

    *outElfCount = elfCount;
    *outElfPacks = elfPacks;
    return true;
}

int
SumElfCalorieLoad(
    ElfPack elfPack
    )
{
    int elfCalorieLoad = 0;
    for (size_t item_i = 0; item_i < elfPack.itemCount; ++item_i)
    {
        elfCalorieLoad += elfPack.items[item_i];
    }
    return elfCalorieLoad;
}

int
FindTotalCaloriesOfMostCaloricElf(
    size_t elfCount,
    const ElfPack* elfPacks
    )
{
    int maxCalorieLoad = 0;
    for (size_t elf_i = 0; elf_i < elfCount; ++elf_i)
    {
        const int elfCalorieLoad = SumElfCalorieLoad(elfPacks[elf_i]);
        if (maxCalorieLoad < elfCalorieLoad)
        {
            maxCalorieLoad = elfCalorieLoad;
        }
    }

    return maxCalorieLoad;
}

int
FindTotalCaloriesOfTopNElves(
    size_t elfCount,
    const ElfPack* elfPacks,
    size_t topN,
    int* topNCalorieLoads
    )
{
    Log("zeroing loads");
    for (int i = 0; i < topN; ++i)
    {
        topNCalorieLoads[i] = 0;
    }

    for (size_t elf_i = 0; elf_i < elfCount; ++elf_i)
    {
        const int elfCalorieLoad = SumElfCalorieLoad(elfPacks[elf_i]);
        Log("checking elf %zu with load %i", elf_i, elfCalorieLoad);

        int* leastExceededCalorieLoadSlot = NULL;
        for (size_t i = 0; i < topN; ++i)
        {
            if (elfCalorieLoad > topNCalorieLoads[i])
            {
                if (leastExceededCalorieLoadSlot == NULL ||
                    topNCalorieLoads[i] < *leastExceededCalorieLoadSlot)
                {
                    leastExceededCalorieLoadSlot = &topNCalorieLoads[i];
                }
            }
        }

        if (leastExceededCalorieLoadSlot != NULL)
        {
            Log("replacing old load %i with %i", *leastExceededCalorieLoadSlot, elfCalorieLoad);
            *leastExceededCalorieLoadSlot = elfCalorieLoad;
        }
    }

    Log("summing loads");
    int topNCalorieLoadSum = 0;
    for (int i = 0; i < topN; ++i)
    {
        topNCalorieLoadSum += topNCalorieLoads[i];
    }

    return topNCalorieLoadSum;
}

int
main(int argc, const char** argv)
{
    if (argc < 2)
    {
        PrintUsage();
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
        Log("Converting lines to elf packs");
        size_t elfCount;
        ElfPack* elfPacks;
        success = GetElfPacksFromLines(inputLineCount, (const char**)inputLines, &elfCount, &elfPacks);
        if (success)
        {
            Log("Processing elf packs");
            int totalCalories = FindTotalCaloriesOfMostCaloricElf(elfCount, elfPacks);
            Log("Total calories carried by elf with most calories in pack... %i\n", totalCalories);


            int topElfTotalCalorieSums[3];
            int sumTotalCaloriesOfTopElves = FindTotalCaloriesOfTopNElves(
                elfCount,
                elfPacks,
                ARRAYSIZE(topElfTotalCalorieSums),
                topElfTotalCalorieSums);

            Log("Total calories carried by top %lu elves... %i",
                ARRAYSIZE(topElfTotalCalorieSums),
                sumTotalCaloriesOfTopElves);

            for (size_t i = 0; i < ARRAYSIZE(topElfTotalCalorieSums); ++i)
            {
                Log("%zu - %i", i+1, topElfTotalCalorieSums[i]);
            }
        }
        else
        {
            Log("Failed to GetElfPacksFromLines!");
        }

        free(inputLines);
    }
    else
    {
        Log("Failed to ReadInputLines!");
    }
}
