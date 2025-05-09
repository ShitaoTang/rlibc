#include <stdio.h>
#include <time.h>
#include <assert.h>

// ✅ Helper function to print struct tm for debugging
void print_tm(const struct tm *tm) {
    printf("  -> tm_year: %d, tm_mon: %d, tm_mday: %d, tm_hour: %d, tm_min: %d, tm_sec: %d, tm_wday: %d, tm_yday: %d, tm_isdst: %d\n",
           tm->tm_year, tm->tm_mon, tm->tm_mday,
           tm->tm_hour, tm->tm_min, tm->tm_sec,
           tm->tm_wday, tm->tm_yday, tm->tm_isdst);
}

void test_epoch() {
    printf("✅ Testing localtime(0) — Epoch time...\n");
    time_t t = 0;
    struct tm *tm = localtime(&t);
    assert(tm != NULL);
    assert(tm->tm_year + 1900 == 1970);
    assert(tm->tm_mon == 0);
    assert(tm->tm_mday == 1);
    print_tm(tm);
}

void test_known_time() {
    printf("✅ Testing known time — 2000-01-01 00:00:00 UTC...\n");
    struct tm input = {
        .tm_year = 100,  // 2000
        .tm_mon = 0,     // Jan
        .tm_mday = 1,
        .tm_hour = 0,
        .tm_min = 0,
        .tm_sec = 0,
        .tm_isdst = -1,
    };
    time_t t = mktime(&input);
    struct tm *tm = localtime(&t);
    assert(tm != NULL);
    assert(tm->tm_year + 1900 == 2000);
    assert(tm->tm_mon == 0);
    assert(tm->tm_mday == 1);
    print_tm(tm);
}

void test_leap_year() {
    printf("✅ Testing leap year — 2020-02-29 12:00:00...\n");
    struct tm input = {
        .tm_year = 120,
        .tm_mon = 1,
        .tm_mday = 29,
        .tm_hour = 12,
        .tm_min = 0,
        .tm_sec = 0,
        .tm_isdst = -1,
    };
    time_t t = mktime(&input);
    struct tm *tm = localtime(&t);
    assert(tm != NULL);
    assert(tm->tm_year + 1900 == 2020);
    assert(tm->tm_mon == 1);
    assert(tm->tm_mday == 29);
    print_tm(tm);
}

void test_far_future() {
    printf("✅ Testing far future — 2100-12-31 23:59:59...\n");
    struct tm input = {
        .tm_year = 200,
        .tm_mon = 11,
        .tm_mday = 31,
        .tm_hour = 23,
        .tm_min = 59,
        .tm_sec = 59,
        .tm_isdst = -1,
    };
    time_t t = mktime(&input);
    struct tm *tm = localtime(&t);
    assert(tm != NULL);
    assert(tm->tm_year + 1900 == 2100);
    assert(tm->tm_mon == 11);
    assert(tm->tm_mday == 31);
    print_tm(tm);
}

void test_dst_transition() {
    printf("✅ Testing DST transition — 2021-03-14 02:30:00 (US DST start)...\n");
    struct tm input = {
        .tm_year = 121,
        .tm_mon = 2,
        .tm_mday = 14,
        .tm_hour = 2,
        .tm_min = 30,
        .tm_sec = 0,
        .tm_isdst = -1,
    };
    time_t t = mktime(&input);
    struct tm *tm = localtime(&t);
    assert(tm != NULL);
    print_tm(tm);  // DST behavior depends on local timezone
}

int main() {
    printf("🧪 Starting tests for localtime()...\n\n");

    test_epoch();
    test_known_time();
    test_leap_year();
    test_far_future();
    test_dst_transition();

    printf("\n🎉 All tests completed.\n");
    return 0;
}
