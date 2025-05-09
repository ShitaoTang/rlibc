#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <unistd.h>

#define NUM_THREADS 5   // 线程数量
#define NUM_ROUNDS  3   // 重复屏障同步轮数

pthread_barrier_t barrier;  // 屏障变量

void print_result(const char *test_name, int passed) {
    printf("%s %s\n", test_name, passed ? "PASSED ✅" : "FAILED ❌");
}

// **基础屏障测试**
void* basic_barrier_test(void* arg) {
    int id = *(int*)arg;
    printf("Thread %d is doing some work...\n", id);
    // usleep(100000 * (id % 3));  // 模拟工作，部分线程快到，部分慢到

    printf("Thread %d is waiting at the barrier...\n", id);
    int res = pthread_barrier_wait(&barrier);

    if (res == PTHREAD_BARRIER_SERIAL_THREAD) {
        printf("Thread %d is the last one to arrive!\n", id);
    }
    printf("Thread %d has passed the barrier!\n", id);

    return NULL;
}

// **快速到达屏障测试**
void* fast_arrival_test(void* arg) {
    int id = *(int*)arg;
    printf("Thread %d is arriving quickly at the barrier...\n", id);
    int res = pthread_barrier_wait(&barrier);

    if (res == PTHREAD_BARRIER_SERIAL_THREAD) {
        printf("Thread %d is the last one in fast test!\n", id);
    }
    printf("Thread %d has passed the fast barrier!\n", id);
    return NULL;
}

// **多轮屏障测试**
void* multi_round_test(void* arg) {
    int id = *(int*)arg;
    for (int round = 1; round <= NUM_ROUNDS; round++) {
        printf("Thread %d (Round %d) is waiting at the barrier...\n", id, round);
        int res = pthread_barrier_wait(&barrier);

        if (res == PTHREAD_BARRIER_SERIAL_THREAD) {
            printf("Thread %d (Round %d) is the last one!\n", id, round);
        }
        printf("Thread %d (Round %d) has passed the barrier!\n", id, round);
        // usleep(50000 * (id % 2));  // 模拟工作
    }
    return NULL;
}

int run_test(void* (*test_func)(void*), const char* test_name) {
    pthread_t threads[NUM_THREADS];
    int thread_ids[NUM_THREADS];
    int success = 1;

    pthread_barrier_init(&barrier, NULL, NUM_THREADS);  // 初始化屏障
    printf("\n===== Running Test: %s =====\n", test_name);

    for (int i = 0; i < NUM_THREADS; i++) {
        thread_ids[i] = i + 1;
        if (pthread_create(&threads[i], NULL, test_func, &thread_ids[i]) != 0) {
            // perror("pthread_create failed");
            success = 0;
        }
    }

    for (int i = 0; i < NUM_THREADS; i++) {
        if (pthread_join(threads[i], NULL) != 0) {
            // perror("pthread_join failed");
            success = 0;
        }
    }

    pthread_barrier_destroy(&barrier);  // 销毁屏障
    print_result(test_name, success);
    return success;
}

int main() {
    int all_tests_passed = 1;

    // 运行多个测试用例
    all_tests_passed &= run_test(basic_barrier_test, "Basic Barrier Test");
    all_tests_passed &= run_test(fast_arrival_test, "Fast Arrival Barrier Test");
    all_tests_passed &= run_test(multi_round_test, "Multi-Round Barrier Test");

    printf("\n===== Test Summary =====\n");
    print_result("All Tests", all_tests_passed);

    return all_tests_passed ? 0 : 1;
}
