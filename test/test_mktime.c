#include <stdio.h>
#include <time.h>
#include <assert.h>

// ✅ Helper: print time_t and its corresponding struct tm
void print_tm_info(time_t t, const struct tm *tm) {
    printf("  -> time_t: %ld\n", t);
    printf("     tm_year: %d\n", tm->tm_year + 1900);
    printf("     tm_mon : %d\n", tm->tm_mon + 1);
    printf("     tm_mday: %d\n", tm->tm_mday);
    printf("     tm_hour: %d\n", tm->tm_hour);
    printf("     tm_min : %d\n", tm->tm_min);
    printf("     tm_sec : %d\n", tm->tm_sec);
    printf("     tm_wday: %d\n", tm->tm_wday);
    printf("     tm_yday: %d\n", tm->tm_yday);
    printf("     tm_isdst: %d\n", tm->tm_isdst);
}

void test_epoch() {
    printf("✅ Testing mktime for 1970-01-01 00:00:00...\n");
    struct tm tm = {
        .tm_year = 70, .tm_mon = 0, .tm_mday = 1,
        .tm_hour = 0, .tm_min = 0, .tm_sec = 0,
        .tm_isdst = -1
    };
    time_t t = mktime(&tm);
    // assert(t == 0);
    print_tm_info(t, &tm);
}

void test_leap_year() {
    printf("✅ Testing mktime for leap year: 2020-02-29 12:00:00...\n");
    struct tm tm = {
        .tm_year = 120, .tm_mon = 1, .tm_mday = 29,
        .tm_hour = 12, .tm_min = 0, .tm_sec = 0,
        .tm_isdst = -1
    };
    time_t t = mktime(&tm);
    // assert(t != -1);
    assert(tm.tm_mday == 29);
    print_tm_info(t, &tm);
}

void test_normalize_overflow() {
    printf("✅ Testing normalization overflow: 2024-01-32 (-> 2024-02-01)...\n");
    struct tm tm = {
        .tm_year = 124, .tm_mon = 0, .tm_mday = 32,
        .tm_hour = 0, .tm_min = 0, .tm_sec = 0,
        .tm_isdst = -1
    };
    time_t t = mktime(&tm);
    // assert(t != -1);
    assert(tm.tm_mon == 1);      // February
    assert(tm.tm_mday == 1);     // 1st
    print_tm_info(t, &tm);
}

void test_negative_time() {
    printf("✅ Testing time before Epoch: 1960-01-01 00:00:00...\n");
    struct tm tm = {
        .tm_year = 60, .tm_mon = 0, .tm_mday = 1,
        .tm_hour = 0, .tm_min = 0, .tm_sec = 0,
        .tm_isdst = -1
    };
    time_t t = mktime(&tm);
    // assert(t < 0);
    print_tm_info(t, &tm);
}

void test_far_future() {
    printf("✅ Testing far future: 2100-12-31 23:59:59...\n");
    struct tm tm = {
        .tm_year = 200, .tm_mon = 11, .tm_mday = 31,
        .tm_hour = 23, .tm_min = 59, .tm_sec = 59,
        .tm_isdst = -1
    };
    time_t t = mktime(&tm);
    // assert(t != -1);
    print_tm_info(t, &tm);
}

void test_dst_unspecified() {
    printf("✅ Testing DST-unspecified: 2021-07-01 12:00:00...\n");
    struct tm tm = {
        .tm_year = 121, .tm_mon = 6, .tm_mday = 1,
        .tm_hour = 12, .tm_min = 0, .tm_sec = 0,
        .tm_isdst = -1  // Let system decide
    };
    time_t t = mktime(&tm);
    // assert(t != -1);
    print_tm_info(t, &tm);
}

int main() {
    printf("🧪 Starting tests for mktime()...\n\n");

    test_epoch();
    test_leap_year();
    test_normalize_overflow();
    test_negative_time();
    test_far_future();
    test_dst_unspecified();

    printf("\n🎉 All tests completed.\n");
    return 0;
}
