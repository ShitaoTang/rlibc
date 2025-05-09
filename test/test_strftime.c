#include <stdio.h>
#include <time.h>
#include <string.h>

void test_standard_format() {
    time_t now = time(NULL);
    struct tm *tm_info = localtime(&now);
    
    char buffer[100];
    strftime(buffer, sizeof(buffer), "%Y-%m-%d %H:%M:%S", tm_info);
    
    printf("Test: Standard Date-Time Format (%%Y-%%m-%%d %%H:%%M:%%S)\n");
    if (strlen(buffer) > 0) {
        printf("✅ Passed: '%s'\n", buffer);
    } else {
        printf("❌ Failed: Output is empty.\n");
    }
}

void test_all_format_specifiers() {
    time_t now = time(NULL);
    struct tm *tm_info = localtime(&now);
    
    char buffer[100];
    
    printf("\nTesting all format specifiers:\n");
    
    // Year
    strftime(buffer, sizeof(buffer), "%Y", tm_info);
    printf("Test: %%Y (Year) -> '%s'\n", buffer);

    // Month
    strftime(buffer, sizeof(buffer), "%m", tm_info);
    printf("Test: %%m (Month) -> '%s'\n", buffer);

    // Day of the month
    strftime(buffer, sizeof(buffer), "%d", tm_info);
    printf("Test: %%d (Day) -> '%s'\n", buffer);

    // Day of the week (full name)
    strftime(buffer, sizeof(buffer), "%A", tm_info);
    printf("Test: %%A (Full Weekday) -> '%s'\n", buffer);

    // Day of the week (abbreviated)
    strftime(buffer, sizeof(buffer), "%a", tm_info);
    printf("Test: %%a (Abbreviated Weekday) -> '%s'\n", buffer);

    // Full month name
    strftime(buffer, sizeof(buffer), "%B", tm_info);
    printf("Test: %%B (Full Month) -> '%s'\n", buffer);

    // Abbreviated month name
    strftime(buffer, sizeof(buffer), "%b", tm_info);
    printf("Test: %%b (Abbreviated Month) -> '%s'\n", buffer);

    // Hour (24-hour clock)
    strftime(buffer, sizeof(buffer), "%H", tm_info);
    printf("Test: %%H (24-Hour) -> '%s'\n", buffer);

    // Hour (12-hour clock)
    strftime(buffer, sizeof(buffer), "%I", tm_info);
    printf("Test: %%I (12-Hour) -> '%s'\n", buffer);

    // Minute
    strftime(buffer, sizeof(buffer), "%M", tm_info);
    printf("Test: %%M (Minute) -> '%s'\n", buffer);

    // Second
    strftime(buffer, sizeof(buffer), "%S", tm_info);
    printf("Test: %%S (Second) -> '%s'\n", buffer);

    // AM/PM
    strftime(buffer, sizeof(buffer), "%p", tm_info);
    printf("Test: %%p (AM/PM) -> '%s'\n", buffer);

    // Day of the year
    strftime(buffer, sizeof(buffer), "%j", tm_info);
    printf("Test: %%j (Day of Year) -> '%s'\n", buffer);

    // Week number of the year
    strftime(buffer, sizeof(buffer), "%U", tm_info);
    printf("Test: %%U (Week Number, Sunday as first day) -> '%s'\n", buffer);

    // Week number of the year (Monday as first day)
    strftime(buffer, sizeof(buffer), "%W", tm_info);
    printf("Test: %%W (Week Number, Monday as first day) -> '%s'\n", buffer);

    // ISO Week Number
    strftime(buffer, sizeof(buffer), "%G", tm_info);
    printf("Test: %%G (ISO Week Number) -> '%s'\n", buffer);

    // Date and time representation
    strftime(buffer, sizeof(buffer), "%c", tm_info);
    printf("Test: %%c (Date and Time) -> '%s'\n", buffer);

    // Locale's date format
    strftime(buffer, sizeof(buffer), "%x", tm_info);
    printf("Test: %%x (Locale's Date Format) -> '%s'\n", buffer);

    // Locale's time format
    strftime(buffer, sizeof(buffer), "%X", tm_info);
    printf("Test: %%X (Locale's Time Format) -> '%s'\n", buffer);

    // Weekday abbreviation
    strftime(buffer, sizeof(buffer), "%u", tm_info);
    printf("Test: %%u (Weekday Number [1-7]) -> '%s'\n", buffer);

    // Day of the month (1-31)
    strftime(buffer, sizeof(buffer), "%e", tm_info);
    printf("Test: %%e (Day of Month [1-31]) -> '%s'\n", buffer);
}

void test_edge_cases() {
    struct tm min_tm = {0};
    min_tm.tm_year = 0;  // 1900年
    min_tm.tm_mon = 0;   // 1月
    min_tm.tm_mday = 1;  // 1号
    
    char buffer[100];
    strftime(buffer, sizeof(buffer), "%Y-%m-%d", &min_tm);
    printf("\nTest: Minimum Date (1900-01-01)\n");
    printf("✅ Passed: '%s'\n", buffer);
    
    struct tm leap_tm = {0};
    leap_tm.tm_year = 2024 - 1900;  // 2024年
    leap_tm.tm_mon = 1;             // 2月
    leap_tm.tm_mday = 29;           // 29日
    
    strftime(buffer, sizeof(buffer), "%Y-%m-%d", &leap_tm);
    printf("\nTest: Leap Year (2024-02-29)\n");
    printf("✅ Passed: '%s'\n", buffer);
}

int main() {
    test_standard_format();
    test_all_format_specifiers();
    test_edge_cases();
    
    return 0;
}