#include <stdint.h>
#include <stddef.h>
#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <assert.h>
#include <string.h>
#include <helpers.h>

typedef enum RPS_CHOICE {
    RPS_CHOICE_ROCK,
    RPS_CHOICE_PAPER,
    RPS_CHOICE_SCISSORS,
} RPS_CHOICE;

typedef struct RpsRoundData {
    RPS_CHOICE opponentChoice;
    RPS_CHOICE playerChoice;
} RpsRoundData;

uint8_t
CalculateRoundScore(
    RpsRoundData round
    )
{
    static const uint8_t ROUND_SCORE_LOSE = 0;
    static const uint8_t ROUND_SCORE_DRAW = 3;
    static const uint8_t ROUND_SCORE_WIN = 6;

    static const uint8_t CHOICE_SCORE_ROCK = 1;
    static const uint8_t CHOICE_SCORE_PAPER = 2;
    static const uint8_t CHOICE_SCORE_SCISSORS = 3;

    uint8_t roundResultScore;
    if (round.opponentChoice == round.playerChoice)
    {
        roundResultScore = ROUND_SCORE_DRAW;
    }
    else
    {
        bool won;
        switch (round.playerChoice)
        {
            case RPS_CHOICE_ROCK:
                won = (round.opponentChoice == RPS_CHOICE_SCISSORS);
                break;
            case RPS_CHOICE_PAPER:
                won = (round.opponentChoice == RPS_CHOICE_ROCK);
                break;
            case RPS_CHOICE_SCISSORS:
                won = (round.opponentChoice == RPS_CHOICE_PAPER);
                break;
            default:
                assert(false);
                won = false;
                break;
        }
        roundResultScore = won ? ROUND_SCORE_WIN : ROUND_SCORE_LOSE;
    }

    uint8_t choiceScore;
    switch (round.playerChoice)
    {
        case RPS_CHOICE_ROCK:
            choiceScore = CHOICE_SCORE_ROCK;
            break;
        case RPS_CHOICE_PAPER:
            choiceScore = CHOICE_SCORE_PAPER;
            break;
        case RPS_CHOICE_SCISSORS:
            choiceScore = CHOICE_SCORE_SCISSORS;
            break;
        default:
            assert(false);
            choiceScore = 0;
            break;
    }

    Log("choice=%i,resultScore=%u,choiceScore=%u", round.playerChoice, roundResultScore, choiceScore);
    return roundResultScore + choiceScore;
}

bool
GetRoundsFromLinesPt1(
    size_t lineCount,
    const char** lines,
    size_t* outRoundCount,
    RpsRoundData** outRounds
    )
{
    // For simplicity, allocate more than enough space to store all elves and items
    RpsRoundData* rounds = (RpsRoundData*)malloc(lineCount * (sizeof(RpsRoundData)));
    if (rounds == NULL)
    {
        *outRoundCount = 0;
        *outRounds = NULL;
        return false;
    }

    for (size_t i = 0; i < lineCount; ++i)
    {
        const char* line = lines[i];
        assert(strlen(line) == 3);
        char opponentChoiceChar = line[0];
        char playerChoiceChar = line[2];

        switch (opponentChoiceChar)
        {
            case 'A': rounds[i].opponentChoice = RPS_CHOICE_ROCK; break;
            case 'B': rounds[i].opponentChoice = RPS_CHOICE_PAPER; break;
            case 'C': rounds[i].opponentChoice = RPS_CHOICE_SCISSORS; break;
            default: assert(false); break;
        }

        switch (playerChoiceChar)
        {
            case 'X': rounds[i].playerChoice = RPS_CHOICE_ROCK; break;
            case 'Y': rounds[i].playerChoice = RPS_CHOICE_PAPER; break;
            case 'Z': rounds[i].playerChoice = RPS_CHOICE_SCISSORS; break;
            default: assert(false); break;
        }
    }

    *outRoundCount = lineCount;
    *outRounds = rounds;
    return true;
}

bool
GetRoundsFromLinesPt2(
    size_t lineCount,
    const char** lines,
    size_t* outRoundCount,
    RpsRoundData** outRounds
    )
{
    // For simplicity, allocate more than enough space to store all elves and items
    RpsRoundData* rounds = (RpsRoundData*)malloc(lineCount * (sizeof(RpsRoundData)));
    if (rounds == NULL)
    {
        *outRoundCount = 0;
        *outRounds = NULL;
        return false;
    }

    for (size_t i = 0; i < lineCount; ++i)
    {
        const char* line = lines[i];
        assert(strlen(line) == 3);
        char opponentChoiceChar = line[0];
        char requiredResultChar = line[2];

        RPS_CHOICE opponentChoice;
        switch (opponentChoiceChar)
        {
            case 'A': opponentChoice = RPS_CHOICE_ROCK; break;
            case 'B': opponentChoice = RPS_CHOICE_PAPER; break;
            case 'C': opponentChoice = RPS_CHOICE_SCISSORS; break;
            default: assert(false); break;
        }

        static const char REQUESTED_RESULT_LOSE = 'X';
        static const char REQUESTED_RESULT_DRAW = 'Y';
        static const char REQUESTED_RESULT_WIN  = 'Z';
        RPS_CHOICE playerChoice;
        switch (requiredResultChar)
        {
            case REQUESTED_RESULT_DRAW:
                playerChoice = opponentChoice;
                break;
            case REQUESTED_RESULT_LOSE:
                switch (opponentChoice)
                {
                    case RPS_CHOICE_ROCK:     playerChoice = RPS_CHOICE_SCISSORS; break;
                    case RPS_CHOICE_PAPER:    playerChoice = RPS_CHOICE_ROCK; break;
                    case RPS_CHOICE_SCISSORS: playerChoice = RPS_CHOICE_PAPER; break;
                    default: assert(false); break;
                }
                break;
            case REQUESTED_RESULT_WIN:
                switch (opponentChoice)
                {
                    case RPS_CHOICE_ROCK:     playerChoice = RPS_CHOICE_PAPER; break;
                    case RPS_CHOICE_PAPER:    playerChoice = RPS_CHOICE_SCISSORS; break;
                    case RPS_CHOICE_SCISSORS: playerChoice = RPS_CHOICE_ROCK; break;
                    default: assert(false); break;
                }
                break;
            default:
                assert(false);
                break;
        }

        rounds[i].opponentChoice = opponentChoice;
        rounds[i].playerChoice = playerChoice;
    }

    *outRoundCount = lineCount;
    *outRounds = rounds;
    return true;
}

int
SumRoundScore(
    size_t roundCount,
    const RpsRoundData* rounds
    )
{
    int sum = 0;
    for (size_t i = 0; i < roundCount; ++i)
    {
        sum += CalculateRoundScore(rounds[i]);
    }
    return sum;
}

void
PrintUsage(const char* prog)
{
    Log("%s [input_file] [pt1|pt2]", prog);
}

int
main(int argc, const char** argv)
{
    if (argc < 3)
    {
        PrintUsage(argv[0]);
        return 1;
    }

    const char* inputFile = argv[1];

    typedef enum INPUT_FORMAT {
        INPUT_FORMAT_UNSET,
        INPUT_FORMAT_1,
        INPUT_FORMAT_2,
    } INPUT_FORMAT;

    INPUT_FORMAT inputFormat = INPUT_FORMAT_UNSET;
    if (strcmp(argv[2], "pt1") == 0)
    {
        inputFormat = INPUT_FORMAT_1;
    }
    else if (strcmp(argv[2], "pt2") == 0)
    {
        inputFormat = INPUT_FORMAT_2;
    }
    else
    {
        PrintUsage(argv[0]);
        return 1;
    }

    bool success;
    size_t inputLineCount;
    char** inputLines;

    Log("Reading inputing lines...");
    success = ReadInputLines(inputFile, &inputLineCount, &inputLines);
    if (success)
    {
        Log("Converting lines to round data (format=%i)", inputFormat);
        size_t roundCount;
        RpsRoundData* rounds;

        success = inputFormat == INPUT_FORMAT_1 ?
            GetRoundsFromLinesPt1(inputLineCount, (const char**)inputLines, &roundCount, &rounds) :
            GetRoundsFromLinesPt2(inputLineCount, (const char**)inputLines, &roundCount, &rounds);

        if (success)
        {
            Log("Processing rounds");
            int totalScore = SumRoundScore(roundCount, rounds);
            Log("Total score... %i\n", totalScore);
        }
        else
        {
            Log("Failed to GetRoundsFromLinesPt1!");
        }

        free(inputLines);
    }
    else
    {
        Log("Failed to ReadInputLines!");
    }
}
