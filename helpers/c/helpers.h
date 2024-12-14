#pragma once

#if   defined(_WIN32)
#define Log(fmt, ...) printf(fmt, __VA_ARGS__)
#elif defined(__linux__)
#define Log(fmt, ...) printf(fmt "\n", ##__VA_ARGS__)
#elif defined(__APPLE__)
#define Log(fmt, ...) printf(fmt "\n", ##__VA_ARGS__)
#else
#error  Log definition needed
#endif // _WIN32 || __linux__

#if !defined(ARRAYSIZE)
#define ARRAYSIZE(ARR) (sizeof(ARR)/sizeof(ARR[0]))
#endif //!ARRAYSIZE

#ifndef T_MALLOC
#define T_MALLOC(type, count) (type*)(malloc(count * sizeof(type)))
#endif // T_MALLOC

#ifndef T_CALLOC
#define T_CALLOC(type, count) (type*)(calloc(count, sizeof(type)))
#endif // T_CALLOC

bool ReadInputLines(const char* inputFileName, size_t* lineCount, char*** lines);
