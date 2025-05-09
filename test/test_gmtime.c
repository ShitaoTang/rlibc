#include <stdio.h>
#include <time.h>
#include <assert.h>

void test_gmtime() {
    time_t timestamps[] = {
        0,                     // Epoch time: 1970-01-01 00:00:00 UTC
        946684800,             // 2000-01-01 00:00:00 UTC
        1672531199,            // 2022-12-31 23:59:59 UTC
        2147483647,            // Max 32-bit signed int: 2038-01-19 03:14:07 UTC
        -1                     // 1969-12-31 23:59:59 UTC (negative timestamp)
    };

    struct tm *tm_result;
    for (int i = 0; i < sizeof(timestamps) / sizeof(time_t); i++) {
        tm_result = gmtime(&timestamps[i]);
        
        assert(tm_result != NULL);
        printf("gmtime() passed for timestamp: %ld -> UTC %d-%d-%d %d:%d:%d\n",
               timestamps[i],
               tm_result->tm_year + 1900, tm_result->tm_mon + 1, tm_result->tm_mday,
               tm_result->tm_hour, tm_result->tm_min, tm_result->tm_sec);
    }
}

int main() {
    test_gmtime();
    printf("All tests passed successfully.\n");
    return 0;
}

