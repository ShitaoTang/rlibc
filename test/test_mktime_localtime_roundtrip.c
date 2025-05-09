// test_mktime_localtime_roundtrip.c
#include <stdio.h>
#include <time.h>
#include <assert.h>

void print_tm(const struct tm *tm) {
    printf("  -> tm_year: %d, tm_mon: %d, tm_mday: %d, "
           "tm_hour: %d, tm_min: %d, tm_sec: %d, "
           "tm_wday: %d, tm_yday: %d, tm_isdst: %d\n",
           tm->tm_year, tm->tm_mon, tm->tm_mday,
           tm->tm_hour, tm->tm_min, tm->tm_sec,
           tm->tm_wday, tm->tm_yday, tm->tm_isdst);
}

void test_roundtrip(time_t original) {
    printf("✅ Testing round-trip for time_t = %ld...\n", original);
    struct tm *tm = localtime(&original);
    assert(tm != NULL);
    time_t round = mktime(tm);
    printf("  -> round-trip: original = %ld, round = %ld\n", original, round);
    assert(round == original);
}

void test_normalization() {
    printf("✅ Testing invalid input normalization — 2023-02-30...\n");
    struct tm tm = {
        .tm_year = 123,  // 2023
        .tm_mon  = 1,    // February
        .tm_mday = 30,   // Invalid
        .tm_hour = 12,
        .tm_min  = 0,
        .tm_sec  = 0,
        .tm_isdst = 0,
    };
    time_t t = mktime(&tm);
    struct tm *tm_result = localtime(&t);
    assert(tm_result != NULL);
    print_tm(tm_result);
    // 2023-03-02 is the normalized result
    assert(tm_result->tm_year + 1900 == 2023);
    assert(tm_result->tm_mon == 2);  // March
    assert(tm_result->tm_mday == 2);
}

void test_before_epoch_localtime() {
    printf("✅ Testing time before Epoch — 1969-12-31 23:59:59 UTC (using localtime)...\n");
    time_t t = -1;
    struct tm *tm = localtime(&t);
    assert(tm != NULL);
    print_tm(tm);

    // Assume system is in UTC+8, so this becomes 1970-01-01 07:59:59
    assert(tm->tm_year + 1900 == 1970);
    assert(tm->tm_mon == 0);
    assert(tm->tm_mday == 1);
    assert(tm->tm_hour == 7);
    assert(tm->tm_min == 59);
    assert(tm->tm_sec == 59);
}

void test_before_epoch_gmtime() {
    printf("✅ Testing time before Epoch — 1969-12-31 23:59:59 UTC (using gmtime)...\n");
    time_t t = -1;
    struct tm *tm = gmtime(&t);
    assert(tm != NULL);
    print_tm(tm);

    // This is pure UTC
    assert(tm->tm_year + 1900 == 1969);
    assert(tm->tm_mon == 11);
    assert(tm->tm_mday == 31);
    assert(tm->tm_hour == 23);
    assert(tm->tm_min == 59);
    assert(tm->tm_sec == 59);
}

int main() {
    printf("🧪 Starting mktime() <-> localtime() round-trip tests...\n");

    test_roundtrip(0);                 // Epoch
    test_roundtrip(946684800);        // 2000-01-01 00:00:00
    test_roundtrip(1582939200);       // 2020-02-29 12:00:00
    test_roundtrip(4102444799);       // 2100-12-31 23:59:59
    test_roundtrip(-1);               // Before Epoch

    test_normalization();             // Feb 30 → Mar 2
    test_before_epoch_localtime();    // Local time zone behavior
    test_before_epoch_gmtime();       // UTC behavior

    printf("\n🎉 All tests completed.\n");
    return 0;
}