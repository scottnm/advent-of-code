#include <stdint.h>
#include <stddef.h>
#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <assert.h>
#include <string.h>
#include <helpers.h>

typedef struct tree_grid_t
{
    size_t width;
    size_t height;
    uint8_t cells[0];
} tree_grid_t;

static
inline
size_t
cell_index(size_t width, size_t row, size_t col)
{
    return (row * width) + col;
}

bool
GetGridFromLines(
    size_t lineCount,
    const char** lines,
    tree_grid_t** outGrid
    )
{
    size_t height = lineCount;
    size_t width = lineCount > 0 ? strlen(lines[0]) : 0;
    size_t treeCount = height * width;

    tree_grid_t* grid = malloc(sizeof(tree_grid_t) + (treeCount * sizeof(uint8_t)));
    if (grid == NULL)
    {
        *outGrid = NULL;
        return false;
    }

    grid->width = width;
    grid->height = height;
    for (size_t row = 0; row < lineCount; ++row)
    {
        for (size_t col = 0; col < width; ++col)
        {
            const char nextChar = lines[row][col];
            assert(nextChar >= '0' && nextChar <= '9');

            const size_t cellIndex = cell_index(width, row, col);
            grid->cells[cellIndex] = nextChar - '0';
        }
    }

    *outGrid = grid;
    return true;
}

#define MAX(a,b) ((a) > (b) ? (a):(b))

size_t
CountVisibleTrees(
    const tree_grid_t* grid
    )
{
    const size_t treeCount = (grid->width * grid->height);
    uint8_t* visibleTreesGrid = (uint8_t*)calloc(treeCount, sizeof(uint8_t));

    // Log("TMP: Allocated buffer");
    assert(visibleTreesGrid != NULL);

    // FIXME: improvement? I wonder if it makes better use of the cache to check the left and right side for the
    // same row one after the other to make better use of the cache

    // Log("TMP: checking left");
    // check for trees visible from the left. Skip the top and bottom row since border rows are all visible
    for (size_t row = 1; row < grid->height - 1; ++row)
    {
        uint8_t largestTree = grid->cells[cell_index(grid->width, row, 0)];
        for (size_t col = 1; col < grid->width - 1; ++col)
        {
            const size_t cellIndex = cell_index(grid->width, row, col);
            const uint8_t nextTreeHeight = grid->cells[cellIndex];
            if (nextTreeHeight > largestTree)
            {
                visibleTreesGrid[cellIndex] = 1;
                largestTree = nextTreeHeight;
            }
        }
    }

    // Log("TMP: checking right");
    // check for trees visible from the right. Skip the top and bottom row since border rows are all visible
    for (size_t row = 1; row < grid->height - 1; ++row)
    {
        uint8_t largestTree = grid->cells[cell_index(grid->width, row, grid->width - 1)];
        for (size_t col = grid->width - 2; col >= 1; --col)
        {
            const size_t cellIndex = cell_index(grid->width, row, col);
            const uint8_t nextTreeHeight = grid->cells[cellIndex];
            if (nextTreeHeight > largestTree)
            {
                visibleTreesGrid[cellIndex] = 1;
                largestTree = nextTreeHeight;
            }
        }
    }

    // Log("TMP: checking top");
    // check for trees visible from the top. Skip the left and right column since border columns are all visible
    for (size_t col = 1; col < grid->width - 1; ++col)
    {
        uint8_t largestTree = grid->cells[cell_index(grid->width, 0, col)];
        for (size_t row = 1; row < grid->height - 1; ++row)
        {
            const size_t cellIndex = cell_index(grid->width, row, col);
            const uint8_t nextTreeHeight = grid->cells[cellIndex];
            if (nextTreeHeight > largestTree)
            {
                visibleTreesGrid[cellIndex] = 1;
                largestTree = nextTreeHeight;
            }
        }
    }

    // Log("TMP: checking bottom");
    // check for trees visible from the bottom. Skip the left and right column since border columns are all visible
    for (size_t col = 1; col < grid->width - 1; ++col)
    {
        uint8_t largestTree = grid->cells[cell_index(grid->width, grid->height - 1, col)];
        for (size_t row = grid->height - 2; row >= 1; --row)
        {
            const size_t cellIndex = cell_index(grid->width, row, col);
            const uint8_t nextTreeHeight = grid->cells[cellIndex];
            if (nextTreeHeight > largestTree)
            {
                visibleTreesGrid[cellIndex] = 1;
                largestTree = nextTreeHeight;
            }
        }
    }

    // Log("TMP: checking summing");
    // having marked all the visible trees (except the borders)
    // let's count
    const size_t borderTreeCount =
        // the top and bottom row minus corners
        ( (grid->width-2) * 2) +
        // the left and right column minus corners
        ( (grid->height-2) * 2) +
        // the corners
        4;

    size_t visibleTreeCount = borderTreeCount;
    for (size_t row = 1; row < grid->height - 1; ++row)
    {
        for (size_t col = 1; col < grid->width - 1; ++col)
        {
            visibleTreeCount += visibleTreesGrid[cell_index(grid->width, row, col)];
        }
    }

    free(visibleTreesGrid);
    return visibleTreeCount;
}

size_t
FindHighestScenicScore(
    const tree_grid_t* grid
    )
{
    const size_t treeCount = grid->width * grid->height;

    // the grids that we use to keep track of score are only u8 sized since I don't have any input with a grid larger
    // than 256 rows or columns
    assert(grid->width <= UINT8_MAX);
    assert(grid->height <= UINT8_MAX);

    uint8_t* leftViewScores =  (uint8_t*)calloc(treeCount, sizeof(uint8_t));
    uint8_t* rightViewScores = (uint8_t*)calloc(treeCount, sizeof(uint8_t));
    uint8_t* upViewScores =    (uint8_t*)calloc(treeCount, sizeof(uint8_t));
    uint8_t* downViewScores =  (uint8_t*)calloc(treeCount, sizeof(uint8_t));

    // Log("TMP: Allocated buffers");
    assert(leftViewScores != NULL);
    assert(rightViewScores != NULL);
    assert(upViewScores != NULL);
    assert(downViewScores != NULL);

    // FIXME: improvement? I wonder if it makes better use of the cache to check the left and right side for the
    // same row one after the other to make better use of the cache

    // Log("TMP: checking rightViewScores");
    // Calculate the rightViewScores for each row (i.e. the views from the left.)
    for (size_t row = 0; row < grid->height; ++row)
    {
        for (size_t col = 0; col < grid->width - 1; ++col) // don't check the last col since it's right view score is always 0
        {
            uint8_t rightViewScore = 0;
            const uint8_t startingTreeHeight = grid->cells[cell_index(grid->width, row, col)];
            for (size_t nextCol = col + 1; nextCol < grid->width; ++nextCol)
            {
                rightViewScore += 1;

                const uint8_t cmpTreeHeight = grid->cells[cell_index(grid->width, row, nextCol)];
                if (startingTreeHeight <= cmpTreeHeight)
                {
                    break;
                }
            }
            rightViewScores[cell_index(grid->width, row, col)] = rightViewScore;
        }
    }

    // Log("TMP: checking leftViewScores");
    // Calculate the leftViewScores for each row (i.e. the views from the right.)
    for (size_t row = 0; row < grid->height; ++row)
    {
        for (size_t col = grid->width - 1; col > 0; --col) // don't check the first col since it's left viewscore is always 0
        {
            uint8_t leftViewScore = 0;
            const uint8_t startingTreeHeight = grid->cells[cell_index(grid->width, row, col)];
            for (int nextCol = (int)col - 1; nextCol >= 0; --nextCol)
            {
                leftViewScore += 1;

                const uint8_t cmpTreeHeight = grid->cells[cell_index(grid->width, row, (size_t)nextCol)];
                if (startingTreeHeight <= cmpTreeHeight)
                {
                    break;
                }
            }
            leftViewScores[cell_index(grid->width, row, col)] = leftViewScore;
        }
    }

    // Log("TMP: checking downViewScores");
    // Calculate the downViewScores for each col (i.e. the views from the top.)
    for (size_t col = 0; col < grid->width; ++col)
    {
        for (size_t row = 0; row < grid->height - 1; ++row) // don't check the last row since it's down view score is always 0
        {
            uint8_t downViewScore = 0;
            const uint8_t startingTreeHeight = grid->cells[cell_index(grid->width, row, col)];
            for (size_t nextRow = row + 1; nextRow < grid->height; ++nextRow)
            {
                downViewScore += 1;

                const uint8_t cmpTreeHeight = grid->cells[cell_index(grid->width, nextRow, col)];
                if (startingTreeHeight <= cmpTreeHeight)
                {
                    break;
                }
            }
            downViewScores[cell_index(grid->width, row, col)] = downViewScore;
        }
    }

    // Log("TMP: checking upViewScores");
    // Calculate the upViewScores for each col (i.e. the views from the bottom.)
    for (size_t col = 0; col < grid->width; ++col)
    {
        for (size_t row = grid->height - 1; row > 0; --row) // don't check the first row since it's up view score is always 0
        {
            uint8_t upViewScore = 0;
            const uint8_t startingTreeHeight = grid->cells[cell_index(grid->width, row, col)];
            for (int nextRow = (int)row - 1; nextRow >= 0; --nextRow)
            {
                upViewScore += 1;

                const uint8_t cmpTreeHeight = grid->cells[cell_index(grid->width, (size_t)nextRow, col)];
                if (startingTreeHeight <= cmpTreeHeight)
                {
                    break;
                }
            }
            upViewScores[cell_index(grid->width, row, col)] = upViewScore;
        }
    }

    // FIXME:
    // Here I skip the border trees since their scenic score will always be zero.
    // I probably should have skipped the border trees in the above loops given this fact.
    size_t maxScenicScore = 0;
    for (size_t row = 1; row < grid->height - 1; ++row)
    {
        for (size_t col = 1; col < grid->width - 1; ++col)
        {
            const size_t treeIndex = cell_index(grid->width, row, col);

            const uint8_t leftViewScore = leftViewScores[treeIndex];
            const uint8_t rightViewScore = rightViewScores[treeIndex];
            const uint8_t upViewScore = upViewScores[treeIndex];
            const uint8_t downViewScore = downViewScores[treeIndex];

            const size_t scenicScore = leftViewScore * rightViewScore * upViewScore * downViewScore;

            if (scenicScore > maxScenicScore)
            {
                maxScenicScore = scenicScore;
            }
        }
    }

#define print_scores(dir) \
    do {\
    const char* dirstr = #dir;\
    printf("%s:\n", dirstr);\
    for (size_t row = 1; row < grid->height - 1; ++row)\
    {\
        for (size_t col = 1; col < grid->width - 1; ++col)\
        {\
            const size_t treeIndex = cell_index(grid->width, row, col);\
\
            const uint8_t viewScore = dir##ViewScores[treeIndex];\
            printf("%u",viewScore);\
    \
        } \
        printf("\n"); \
    } \
    printf("\n");\
    } while (0)

    // print_scores(left);
    // print_scores(right);
    // print_scores(up);
    // print_scores(down);

    free(leftViewScores);
    free(rightViewScores);
    free(upViewScores);
    free(downViewScores);
    return maxScenicScore;
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

        tree_grid_t*  grid;
        success = GetGridFromLines(inputLineCount, (const char**)inputLines, &grid);
        if (success)
        {
            Log("Processing...");
            size_t visibleTreeCount = CountVisibleTrees(grid);
            Log("visible tree count... %zu\n", visibleTreeCount);
            size_t maxScenicScore = FindHighestScenicScore(grid);
            Log("max scenic score... %zu\n", maxScenicScore);
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
