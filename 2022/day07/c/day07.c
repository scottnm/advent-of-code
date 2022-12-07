#include <stdint.h>
#include <stddef.h>
#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <assert.h>
#include <string.h>
#include <helpers.h>

#define TOTAL_DISK_SPACE 70000000
#define UPDATE_REQUIRED_DISK_SPACE 30000000
#define MAX_FSN_NAME 64


typedef enum fsn_type_t
{
    FSN_TYPE_FILE,
    FSN_TYPE_DIR,
} fsn_type_t;

typedef struct fsn_t
{
    char name[MAX_FSN_NAME];
    struct fsn_t* parent;
    fsn_type_t type;
    size_t size;
    size_t childNodeCount;
    struct fsn_t* childNodes;
} fsn_t;

void
InitFileSystemNode(
    fsn_t* node,
    const char* name,
    fsn_type_t type,
    size_t size,
    fsn_t* parent
    )
{
    strcpy(node->name, name);
    node->parent = parent;
    node->type = type;
    node->size = size;
    node->childNodeCount = 0;
    node->childNodes = NULL;
}

void
AllocateListedNodes(
    fsn_t* currentNode,
    size_t lineCount,
    const char** lines,
    fsn_t* freeNodes,
    size_t* freeNodesUsed
    )
{
    size_t listedNodeCount = 0;

    for (size_t i = 0; i < lineCount; ++i)
    {
        const char* line = lines[i];

        if (line[0] == '$')
        {
            // if the current line is a 'command' we have reached the end of the ls output
            break;
        }

        if (strncmp(line, "dir ", 4) == 0)
        {
            // Processing a directory..
            const char* dirName = &line[4];
            InitFileSystemNode(&freeNodes[listedNodeCount], dirName, FSN_TYPE_DIR, 0, currentNode);
        }
        else
        {
            // Processing a file..
            size_t fileSize;
            char fileNameBuffer[MAX_FSN_NAME];
            sscanf(line, "%zu %s", &fileSize, fileNameBuffer);
            InitFileSystemNode(&freeNodes[listedNodeCount], fileNameBuffer, FSN_TYPE_FILE, fileSize, currentNode);
        }

        listedNodeCount += 1;
    }

    *freeNodesUsed = listedNodeCount;
}

void
CalculateDirectorySizes(
    fsn_t* parent
    )
{
    assert(parent != NULL);
    assert(parent->type == FSN_TYPE_DIR);
    assert(parent->size == 0);

    // First recursively calculate the directory sizes of all child directories
    for (size_t i = 0; i < parent->childNodeCount; ++i)
    {
        fsn_t* child = &parent->childNodes[i];
        if (child->type == FSN_TYPE_DIR)
        {
            assert(child->size == 0);
            CalculateDirectorySizes(child);
        }
    }

    // Then sum the sizes
    size_t directorySize = 0;
    for (size_t i = 0; i < parent->childNodeCount; ++i)
    {
        const fsn_t* child = &parent->childNodes[i];
        directorySize += child->size;
    }

    parent->size = directorySize;
}

bool
GetFileSystemFromLines(
    size_t lineCount,
    const char** lines,
    fsn_t** outFileSystem
    )
{
    // HACK! we certainly can't have more nodes than are number of lines
    // in the input so just allocate enough nodes for that.
    const size_t fileSystemNodeBufferSize = sizeof(fsn_t) * lineCount;
    fsn_t* fileSystemNodes = malloc(fileSystemNodeBufferSize);
    if (fileSystemNodes == NULL)
    {
        *outFileSystem = NULL;
        return false;
    }

    memset(fileSystemNodes, 0, fileSystemNodeBufferSize);

    fsn_t* root = &fileSystemNodes[0];
    InitFileSystemNode(root, "/", FSN_TYPE_DIR, 0, NULL);

    fsn_t* freeNodes = &fileSystemNodes[1];
    fsn_t* currentNode = root;

    size_t currLineIndex = 0;
    while (currLineIndex < lineCount)
    {
        const char* line = lines[currLineIndex];

        if (strcmp(line, "$ cd /") == 0)
        {
            currentNode = root;
        }
        else if (strcmp(line, "$ cd ..") == 0)
        {
            assert (currentNode->parent != NULL);
            currentNode = currentNode->parent;
        }
        else if (strcmp(line, "$ ls") == 0)
        {
            // HACK! New nodes are basically unknown and unusable until we `ls` for them.
            // This is pretty unlike how a real filesystem works but I'll take whatever
            // shortcuts I can get. This would break if we wanted to support input that
            // `cd` into a file before knowing about that file
            if (currentNode->type != FSN_TYPE_DIR ||
                currentNode->childNodeCount != 0 ||
                currentNode->childNodes != NULL)
            {
                Log("Unexpected dir state! dir=%s, type=%i, childCount=%zu, childNodes=%p\nLine #%zu: \"%s\"",
                    currentNode->name,
                    currentNode->type,
                    currentNode->childNodeCount,
                    currentNode->childNodes,
                    currLineIndex,
                    line);
                assert(0 && "Unexpected dir state!");
            }

            size_t freeNodesUsed;
            AllocateListedNodes(
                currentNode,
                lineCount - (currLineIndex + 1),
                &lines[currLineIndex+1],
                freeNodes,
                &freeNodesUsed);

            currentNode->childNodeCount = freeNodesUsed;
            currentNode->childNodes = freeNodes;

            freeNodes += freeNodesUsed;

            // consume each of the ls result lines
            currLineIndex += freeNodesUsed;
        }
        else if (strncmp(line, "$ cd ", 5) == 0)
        {
            assert (currentNode->type == FSN_TYPE_DIR);

            const char* dirName = &line[5];
            fsn_t* dirNode = NULL;

            for (size_t i = 0; i < currentNode->childNodeCount; ++i)
            {
                if (strcmp(dirName, currentNode->childNodes[i].name) == 0)
                {
                    dirNode = &currentNode->childNodes[i];
                    break;
                }
            }

            if (dirNode == NULL)
            {
                Log("Failed to find node \"%s\" in node \"%s\"!\nLine #%zu: \"%s\"", dirName, currentNode->name, currLineIndex, line);
                assert (0 && "dirNode != NULL");
            }

            currentNode = dirNode;
        }
        else
        {
            Log("Unexpected line in input!\nline #%zu: \"%s\"", currLineIndex, line);
            assert(false && "Unexpected line in input!");
        }

        // consume the current line
        currLineIndex += 1;
    }

    // Now that all nodes have been discovered, calculate the sizes of all directories.
    CalculateDirectorySizes(root);
    *outFileSystem = root;
    return true;
}

size_t
SumDirectoriesSmallerThan(
    const fsn_t* parent,
    size_t maxFileSize
    )
{
    assert(parent != NULL);
    assert(parent->type == FSN_TYPE_DIR);

    size_t sumDirectoriesSmallerThan = 0;

    for (size_t i = 0; i < parent->childNodeCount; ++i)
    {
        fsn_t* child = &parent->childNodes[i];
        if (child->type == FSN_TYPE_DIR)
        {
            sumDirectoriesSmallerThan += SumDirectoriesSmallerThan(child, maxFileSize);
        }
    }

    if (parent->size < maxFileSize)
    {
        sumDirectoriesSmallerThan += parent->size;
    }

    return sumDirectoriesSmallerThan;
}

const fsn_t*
FindSmallestDirectoryToDeleteOfAtleastNSize(
    const fsn_t* parent,
    size_t minSpaceRequired
    )
{
    assert(parent != NULL);
    assert(parent->type == FSN_TYPE_DIR);

    const fsn_t* deleteCandidate = NULL;

    for (size_t i = 0; i < parent->childNodeCount; ++i)
    {
        fsn_t* child = &parent->childNodes[i];
        if (child->type == FSN_TYPE_DIR)
        {
            const fsn_t* childDeleteCandidate = FindSmallestDirectoryToDeleteOfAtleastNSize(child, minSpaceRequired);
            if (deleteCandidate == NULL)
            {
                deleteCandidate = childDeleteCandidate;
            }
            else if (childDeleteCandidate != NULL &&
                     childDeleteCandidate->size < deleteCandidate->size)
            {
                deleteCandidate = childDeleteCandidate;
            }
        }
    }

    if (parent->size > minSpaceRequired)
    {
        if (deleteCandidate == NULL ||
            parent->size < deleteCandidate->size)
        {
            deleteCandidate = parent;
        }
    }

    return deleteCandidate;
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
        fsn_t* filesystem;

        success = GetFileSystemFromLines(inputLineCount, (const char**)inputLines, &filesystem);
        if (success)
        {
            Log("Processing...");

            size_t sumDirectoriesSmallerThan = SumDirectoriesSmallerThan(filesystem, 100000);
            Log("sum... %zu (of %zu)\n", sumDirectoriesSmallerThan, filesystem->size);

            const size_t filesystemUnusedCapacity = TOTAL_DISK_SPACE - filesystem->size;
            if (filesystemUnusedCapacity < UPDATE_REQUIRED_DISK_SPACE)
            {
                const size_t filesystemSpaceNeeded = UPDATE_REQUIRED_DISK_SPACE - filesystemUnusedCapacity;
                Log("Need to free update disk space! Update is %iB, only %zuB remaining, need %zuB!",
                    UPDATE_REQUIRED_DISK_SPACE,
                    filesystemUnusedCapacity,
                    filesystemSpaceNeeded);

                const fsn_t* deleteCandidate = FindSmallestDirectoryToDeleteOfAtleastNSize(filesystem, filesystemSpaceNeeded);
                assert (deleteCandidate->type == FSN_TYPE_DIR);
                Log("Recommend deleting dir \"%s\" of size %zu to make room for update.",
                    deleteCandidate->name,
                    deleteCandidate->size);
            }
            else
            {
                Log("Enough space is free for the %uB update!", UPDATE_REQUIRED_DISK_SPACE);
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
