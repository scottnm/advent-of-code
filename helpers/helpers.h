#pragma once

#if   defined(__WIN32)
#define Log(fmt, ...) printf(fmt, __VA_ARGS__)
#elif defined(__linux__)
#define Log(fmt, ...) printf(fmt "\n", ##__VA_ARGS__)
#else
#error  Log definition needed
#endif

#if !defined(ARRAYSIZE)
#define ARRAYSIZE(ARR) (sizeof(ARR)/sizeof(ARR[0]))
#endif

bool ReadInputLines(const char* inputFileName, size_t* lineCount, char*** lines);
