#include <stdio.h>
#include <assert.h>
#include <unistd.h>
#include <time.h>

// Function to print test result with emoji
void print_result(const char *test_name, int passed) {
    if (passed) {
        printf("✅ %s: Passed\n", test_name);
    } else {
        printf("❌ %s: Failed\n", test_name);
    }
}

// Test 1: Basic functionality - clock() should return a non-negative value
void test_basic_functionality() {
    clock_t start = clock();
    int passed = (start >= 0);
    assert(start >= 0 && "clock() should return a non-negative value");
    print_result("Test Basic Functionality", passed);
}

// Test 2: Monotonicity - clock() should increase over time
void test_monotonicity() {
    clock_t start = clock();
    sleep(1); // Wait for 1 second
    clock_t end = clock();
    int passed = (end > start);
    printf("Start: %ld, End: %ld\n", (long)start, (long)end);
    assert(end > start && "clock() should increase after time passes");
    print_result("Test Monotonicity", passed);
}

// Test 3: Resolution - clock() should reflect small time differences
void test_resolution() {
    clock_t start = clock();
    for (volatile int i = 0; i < 1000000; i++); // Busy loop
    clock_t end = clock();
    int passed = (end > start);
    printf("Start: %ld, End: %ld, Diff: %ld\n", (long)start, (long)end, (long)(end - start));
    assert(end > start && "clock() should detect small time differences");
    print_result("Test Resolution", passed);
}

// // Test 4: Time measurement - clock() should approximate real time
// void test_time_measurement() {
//     clock_t start = clock();
//     sleep(2); // Sleep for 2 seconds
//     clock_t end = clock();
//     double elapsed = (double)(end - start);
//     int passed = (elapsed >= 2);
//     assert(passed && "Elapsed time should be approximately 2 seconds");
//     printf("Elapsed time: %ld seconds\n", elapsed);
//     print_result("Test Time Measurement", passed);
// }

// Test 5: Edge case - Very short duration
void test_short_duration() {
    clock_t start = clock();
    clock_t end = clock();
    int passed = (end >= start);
    assert(end >= start && "clock() should not decrease in short duration");
    printf("Start: %ld, End: %ld\n", (long)start, (long)end);
    print_result("Test Short Duration", passed);
}

// Main function to run all tests
int main() {
    printf("Running clock() function tests...\n");
    printf("CLOCKS_PER_SEC: %ld\n\n", (long)CLOCKS_PER_SEC);

    test_basic_functionality();
    test_monotonicity();
    test_resolution();
    // test_time_measurement();
    test_short_duration();

    printf("\nTest suite completed.\n");
    return 0;
}