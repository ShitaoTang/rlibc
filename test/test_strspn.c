#include <stdio.h>
#include <string.h>

extern size_t strspn(const char *s, const char *c);

void test_strspn() {
    struct {
        const char *s;
        const char *c;
        size_t expected;
    } tests[] = {
        {"abcdef", "abc", 3},
        {"abc123", "abc", 3},
        {"123456", "123456", 6},
        {"hello world", "aeiou", 0},
        {"xyz", "abc", 0},
        {"", "abc", 0},
        {"abc", "", 0},
        {"aaaaa", "a", 5},
        {"aabbaabb", "ab", 8},
        {"abcdef", "xyz", 0},
        {"000111222", "012", 9},
        {"test123", "tse", 4}
    };

    int passed = 1;
    for (size_t i = 0; i < sizeof(tests)/sizeof(tests[0]); i++) {
        size_t result = strspn(tests[i].s, tests[i].c);
        if (result == tests[i].expected) {
            printf("✅ Test %zu passed: strspn(\"%s\", \"%s\") = %zu\n", 
                   i + 1, tests[i].s, tests[i].c, result);
        } else {
            printf("❌ Test %zu failed: strspn(\"%s\", \"%s\") = %zu (expected %zu)\n", 
                   i + 1, tests[i].s, tests[i].c, result, tests[i].expected);
            passed = 0;
        }
    }

    if (passed) {
        printf("✅ All tests passed!\n");
    } else {
        printf("❌ Some tests failed.\n");
    }
}

int main() {
    test_strspn();
    return 0;
}
