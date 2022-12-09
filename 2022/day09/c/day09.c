#include <stdint.h>
#include <stddef.h>
#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <assert.h>
#include <string.h>
#include <helpers.h>

typedef enum dir_t
{
    DIR_LEFT = 'L',
    DIR_RIGHT = 'R',
    DIR_UP = 'U',
    DIR_DOWN = 'D',
} dir_t;

typedef struct movement_t
{
    dir_t dir;
    int count;
} movement_t;

typedef struct pos_t {
    int x;
    int y;
} pos_t;

bool
GetMovementFromLines(
    size_t lineCount,
    const char** lines,
    size_t* outMovementCount,
    movement_t** outMovements
    )
{
    movement_t* movements = malloc(sizeof(movement_t) * lineCount); // one movement per line
    if (movements == NULL)
    {
        *outMovementCount = 0;
        *outMovements = NULL;
        return false;
    }

    for (size_t i = 0; i < lineCount; ++i)
    {
        const char* line = lines[i];
        assert(strlen(line) >= 3);
        assert(line[1] == ' ');

        int moveCount = atoi(&line[2]);
        assert(moveCount >= 1);

        dir_t moveDir;
        switch (line[0])
        {
            case 'L': moveDir = DIR_LEFT; break;
            case 'R': moveDir = DIR_RIGHT; break;
            case 'U': moveDir = DIR_UP; break;
            case 'D': moveDir = DIR_DOWN; break;
            default:  assert(false); moveDir = 0; break;
        }

        movements[i].dir = moveDir;
        movements[i].count = moveCount;
    }

    *outMovementCount = lineCount;
    *outMovements = movements;
    return true;
}

/*
inline
static
void
SimulateMovementPt1(
    pos_t* headPos,
    pos_t* tailPos,
    dir_t headMoveDir
    )
{
    switch (headMoveDir)
    {
        case  DIR_LEFT: headPos->x -= 1; break;
        case DIR_RIGHT: headPos->x += 1; break;
        case  DIR_DOWN: headPos->y -= 1; break;
        case    DIR_UP: headPos->y += 1; break;
        default: assert(0); break;
    }

    size_t yDiff = abs(headPos->y - tailPos->y);
    size_t xDiff = abs(headPos->x - tailPos->x);
    if (yDiff < 2 && xDiff < 2)
    {
        return;
    }

    if (headPos->x == tailPos->x)
    {
        tailPos->y += headPos->y > tailPos->y ? 1 : -1;
    }
    else if (headPos->y == tailPos->y)
    {
        tailPos->x += headPos->x > tailPos->x ? 1 : -1;
    }
    else
    {
        tailPos->y += headPos->y > tailPos->y ? 1 : -1;
        tailPos->x += headPos->x > tailPos->x ? 1 : -1;
    }
}
*/

inline
static
void
SimulateMovementPt2(
    size_t knotCount,
    pos_t* knots,
    dir_t headMoveDir
    )
{
    assert (knotCount > 0);

    // First move the head knot
    switch (headMoveDir)
    {
        case  DIR_LEFT: knots[0].x -= 1; break;
        case DIR_RIGHT: knots[0].x += 1; break;
        case  DIR_DOWN: knots[0].y -= 1; break;
        case    DIR_UP: knots[0].y += 1; break;
        default: assert(0); break;
    }

    // now simulate each following knot very similar to how we simulated in pt1.
    for (size_t i = 0; i < knotCount - 1; ++i)
    {
        pos_t* leadKnot = &knots[i];
        pos_t* followKnot = &knots[i+1];

        size_t yDiff = abs(leadKnot->y - followKnot->y);
        size_t xDiff = abs(leadKnot->x - followKnot->x);
        if (yDiff < 2 && xDiff < 2)
        {
            // this knot doesn't need to move since it's still within
            // 1 cell in both directions of the knot leading it.
            //
            // if this follow knot doesn't need to move, the knot
            // following it won't either.
            return;
        }

        if (leadKnot->x == followKnot->x)
        {
            followKnot->y += leadKnot->y > followKnot->y ? 1 : -1;
        }
        else if (leadKnot->y == followKnot->y)
        {
            followKnot->x += leadKnot->x > followKnot->x ? 1 : -1;
        }
        else
        {
            followKnot->y += leadKnot->y > followKnot->y ? 1 : -1;
            followKnot->x += leadKnot->x > followKnot->x ? 1 : -1;
        }
    }
}

size_t
CountUniqueTailCellsVisited(
    size_t movementCount,
    const movement_t* movements
    )
{
    size_t totalSteps = 0;
    for (size_t i = 0; i < movementCount; ++i)
    {
        totalSteps += movements[i].count;
    }

    // allocate enough positions to store the total number of steps the tail might move
    size_t tailCellPositionRecordCount = 0;
    pos_t* tailCellPositionRecords = malloc(sizeof(pos_t) * totalSteps);
    assert(tailCellPositionRecords != NULL);

// N.B. for pt. 1, define 2
//      for pt. 2, define 10
//#define KNOT_COUNT 2
#define KNOT_COUNT 10

    // Simulate all of the steps that each knot will take
    pos_t knots[KNOT_COUNT];
    memset(knots, 0, sizeof(knots));

    const pos_t* tailPos = &knots[ARRAYSIZE(knots) - 1];
    for (size_t i = 0; i < movementCount; ++i)
    {
        const movement_t currMove = movements[i];
        for (size_t j = 0; j < currMove.count; ++j)
        {
            SimulateMovementPt2(ARRAYSIZE(knots), knots, currMove.dir);
            tailCellPositionRecords[tailCellPositionRecordCount] = *tailPos;
            tailCellPositionRecordCount += 1;
        }
    }

    int minX = 0;
    int maxX = 0;
    int minY = 0;
    int maxY = 0;
    for (size_t i = 0; i < tailCellPositionRecordCount; ++i)
    {
        pos_t nextPos = tailCellPositionRecords[i];
        minX = nextPos.x < minX ? nextPos.x : minX;
        maxX = nextPos.x > maxX ? nextPos.x : maxX;
        minY = nextPos.y < minY ? nextPos.y : minY;
        maxY = nextPos.y > maxY ? nextPos.y : maxY;
    }

    size_t width = (maxX - minX) + 1;
    size_t height = (maxY - minY) + 1;
    uint16_t* grid = (uint16_t*)calloc(width * height, sizeof(uint16_t));

#define GRID_CELL(r, c) grid[ ((r-minY) * width) + (c-minX)  ] // offset version of row*width + col
    for (size_t i = 0; i < tailCellPositionRecordCount; ++i)
    {
        pos_t nextPos = tailCellPositionRecords[i];
        GRID_CELL(nextPos.y, nextPos.x) += 1;
    }

    size_t uniqueTailCellsVisitedCount = 0;
    for (int r = 0; r < height; ++r)
    {
        for (int c = 0; c < width; ++c)
        {
            if (grid[r * width + c] > 0)
            {
                uniqueTailCellsVisitedCount += 1;
            }
        }
    }
#undef GRID_CELL

    free(tailCellPositionRecords);
    return uniqueTailCellsVisitedCount;
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
        size_t        movementCount;
        movement_t*   movements; // TODO: fix

        success = GetMovementFromLines(inputLineCount, (const char**)inputLines, &movementCount, &movements);
        if (success)
        {
            Log("Processing...");
            size_t count = CountUniqueTailCellsVisited(movementCount, movements);
            Log("visited %zu unique tail cells\n", count);
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
