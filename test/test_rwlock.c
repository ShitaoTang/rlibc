#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <assert.h>
#include <unistd.h>  // usleep
#include <time.h>    // timespec, clock_gettime

extern int pthread_rwlock_init(pthread_rwlock_t *rwlock, const pthread_rwlockattr_t *attr);
//extern int pthread_rwlock_rdlock(pthread_rwlock_t *rwlock);
//extern int pthread_rwlock_wrlock(pthread_rwlock_t *rwlock);
// extern int pthread_rwlock_unlock(pthread_rwlock_t *rwlock);

// 共享资源
static int shared_resource = 0;
static pthread_rwlock_t rwlock;

#define NUM_READERS 3
#define NUM_WRITERS 3
#define NUM_ITERATIONS 5

// 获取当前时间的辅助函数（纳秒精度）
static void print_timestamp(const char *prefix, int id, int value) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);

    printf("[");
    printf("%ld", ts.tv_sec);
    printf(".");

    // 手动补零，确保纳秒部分始终为 9 位
    long nsec = ts.tv_nsec;
    long temp = nsec;
    int digits = 0;
    while (temp > 0) {
        temp /= 10;
        digits++;
    }
    for (int i = 0; i < 9 - digits; i++) {
        printf("0");
    }
    printf("%ld", nsec);

    printf("] ");
    printf("%s %d: shared_resource = %d\n", prefix, id, value);
}

static void print_timestamp_write(const char *prefix, int id, int old_value, int new_value) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);

    printf("[");
    printf("%ld", ts.tv_sec);
    printf(".");

    // 手动补零
    long nsec = ts.tv_nsec;
    long temp = nsec;
    int digits = 0;
    while (temp > 0) {
        temp /= 10;
        digits++;
    }
    for (int i = 0; i < 9 - digits; i++) {
        printf("0");
    }
    printf("%ld", nsec);

    printf("] ");
    printf("%s %d: shared_resource: ", prefix, id);
    printf("%d", old_value);
    printf(" -> ");
    printf("%d\n", new_value);
}

void *reader_thread(void *arg) {
    int id = *(int *)arg;
    int last_value = -1;

    for (int i = 0; i < NUM_ITERATIONS; i++) {
        pthread_rwlock_rdlock(&rwlock);
        int value = shared_resource;
        print_timestamp("Reader", id, value);  // 在临界区内打印时间
        pthread_rwlock_unlock(&rwlock);

        // 断言 shared_resource 不应该倒退
        if (last_value != -1) {
            assert(value >= last_value && "Error: Reader observed a rollback in shared_resource!");
        }
        last_value = value;

        usleep(10000); // 模拟延迟
    }
    return NULL;
}

void *writer_thread(void *arg) {
    int id = *(int *)arg;

    for (int i = 0; i < NUM_ITERATIONS; i++) {
        pthread_rwlock_wrlock(&rwlock);
        int old_value = shared_resource;
        shared_resource++;
        int new_value = shared_resource;
        print_timestamp_write("Writer", id, old_value, new_value);  // 在临界区内打印时间
        pthread_rwlock_unlock(&rwlock);

        // 断言写入的值是递增的
        assert(new_value == old_value + 1 && "Error: Writer did not properly increment shared_resource!");

        usleep(15000); // 模拟延迟
    }
    return NULL;
}

void test_multiple_readers() {
    printf("===== Test 1: Multiple Readers =====\n");
    pthread_t readers[NUM_READERS];
    int ids[NUM_READERS];

    for (int i = 0; i < NUM_READERS; i++) {
        ids[i] = i + 1;
        pthread_create(&readers[i], NULL, reader_thread, &ids[i]);
    }
    for (int i = 0; i < NUM_READERS; i++) {
        pthread_join(readers[i], NULL);
    }
    printf("Test 1 PASSED ✅\n\n");
}

void test_multiple_writers() {
    printf("===== Test 2: Multiple Writers =====\n");
    pthread_t writers[NUM_WRITERS];
    int ids[NUM_WRITERS];

    for (int i = 0; i < NUM_WRITERS; i++) {
        ids[i] = i + 1;
        pthread_create(&writers[i], NULL, writer_thread, &ids[i]);
    }
    for (int i = 0; i < NUM_WRITERS; i++) {
        pthread_join(writers[i], NULL);
    }

    // 断言最终值是否符合预期
    int expected_value = NUM_WRITERS * NUM_ITERATIONS;
    assert(shared_resource == expected_value && "Error: shared_resource does not match expected value after writers!");

    printf("Test 2 PASSED ✅\n\n");
}

void test_mixed_readers_writers() {
    printf("===== Test 3: Mixed Readers and Writers =====\n");
    pthread_t readers[NUM_READERS], writers[NUM_WRITERS];
    int ids[NUM_READERS + NUM_WRITERS];

    for (int i = 0; i < NUM_READERS; i++) {
        ids[i] = i + 1;
        pthread_create(&readers[i], NULL, reader_thread, &ids[i]);
    }
    for (int i = 0; i < NUM_WRITERS; i++) {
        ids[NUM_READERS + i] = i + 1;
        pthread_create(&writers[i], NULL, writer_thread, &ids[NUM_READERS + i]);
    }

    for (int i = 0; i < NUM_READERS; i++) {
        pthread_join(readers[i], NULL);
    }
    for (int i = 0; i < NUM_WRITERS; i++) {
        pthread_join(writers[i], NULL);
    }

    // 再次验证最终值
    int expected_value = NUM_WRITERS * NUM_ITERATIONS * 2;
    assert(shared_resource == expected_value && "Error: shared_resource does not match expected value after mixed operations!");

    printf("Test 3 PASSED ✅\n\n");
}

int main() {
    // 初始化读写锁
    pthread_rwlock_init(&rwlock, NULL);

    test_multiple_readers();
    test_multiple_writers();
    test_mixed_readers_writers();

    printf("All tests PASSED ✅🎉\n");
    return 0;
}
