#include <stdbool.h>
#include <stdint.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include <string.h>
#include "helpers.h"

static size_t ReadFileSize(FILE* f);
static bool ReadFileToBuffer(const char* filePath, size_t* outFileSize, char** outFileBuffer);

bool ReadInputLines(
    const char* inputFileName,
    size_t* outLineCount,
    char*** outLines
    )
{
    //
    // First read the file to a buffer
    //
    size_t fileByteCount;
    char* fileBytes;
    bool readSuccess = ReadFileToBuffer(inputFileName, &fileByteCount, &fileBytes);
    if (!readSuccess)
    {
        return false;
    }

    //
    // Handle empty files as an empty line buffer
    //
    if (fileByteCount == 0)
    {
        *outLineCount = 0;
        *outLines = NULL;
        return true;
    }

    //
    // Count the number of line separators so we can allocate a serialized buffer
    //
    int lineCount = 1;
    for (int i = 0; i < fileByteCount; ++i)
    {
        if (fileBytes[i] == '\n' && i != (fileByteCount - 1))
        {
            lineCount += 1;
        }
    }

    //
    // Allocate a serialized buffer and copy the file contents to the end of it.
    // The head will be used to store character pointers for easier iteration.
    //
    size_t linePtrsSize = (sizeof(const char*) * lineCount);
    char* serializedBuffer = malloc(fileByteCount + linePtrsSize + 1); // +1 for null
    if (serializedBuffer == NULL)
    {
        free(fileBytes);
        return false;
    }

    char** serializedBufferLinesSection = (char**)serializedBuffer;
    char* serializedBufferContentPtr = serializedBuffer + linePtrsSize;
    memcpy(serializedBufferContentPtr, fileBytes, fileByteCount);
    serializedBufferContentPtr[fileByteCount] = '\0';
    free(fileBytes);
    fileBytes = NULL;

    //
    // Lastly iterate over the contents, marking each line and replacing line separators with null terminators
    //
    int nextLineIndex = 0;
    bool nextCharStartsNewLine = true;
    for (int i = 0; i < fileByteCount; ++i)
    {
        if (nextCharStartsNewLine && serializedBufferContentPtr[i] != '\r')
        {
            serializedBufferLinesSection[nextLineIndex] = &serializedBufferContentPtr[i];
            nextLineIndex += 1;
            nextCharStartsNewLine = false;
        }

        if (serializedBufferContentPtr[i] == '\n')
        {
            nextCharStartsNewLine = true;
            serializedBufferContentPtr[i] = '\0';
        }
        else if (serializedBufferContentPtr[i] == '\r')
        {
            serializedBufferContentPtr[i] = '\0';
        }
    }

    assert(nextLineIndex == lineCount);
    *outLineCount = lineCount;
    *outLines = serializedBufferLinesSection;
    return true;
}

size_t
ReadFileSize(
    FILE* f
    )
{
    int res = fseek(f, 0, SEEK_END);
    assert(res == 0);
    size_t filesize = ftell(f);
    res = fseek(f, 0, SEEK_SET);
    assert(res == 0);
    return filesize;
}

bool
ReadFileToBuffer(
    const char* filePath,
    size_t* outFileSize,
    char** outFileBuffer
    )
{
    *outFileSize = 0;
    *outFileBuffer = NULL;

    bool success = true;
    char* buffer = NULL;
    FILE* f = fopen(filePath, "rb");
    if (f != NULL)
    {
        size_t filesize = ReadFileSize(f);
        if (filesize > 0)
        {
            buffer = (char*)malloc(filesize);
            if (buffer != NULL)
            {
                const size_t bytesRead = fread(buffer, 1, filesize, f);
                if (bytesRead == filesize)
                {
                    *outFileSize = filesize;
                    *outFileBuffer = buffer;
                }
                else
                {
                    Log("Failed to read %zu bytes from file", filesize);
                    free(buffer);
                    buffer = NULL;
                    success = false;
                }
            }
            else
            {
                Log("Failed to allocate %zu bytes for file buffer", filesize);
                success = false;
            }
        }
        fclose(f);
    }
    else
    {
        Log("Failed to open file '%s'", filePath);
        success = false;
    }

    return success;
}

