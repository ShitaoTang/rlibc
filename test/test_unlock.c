#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <errno.h>

#define THREAD_COUNT 4

// 声明标准的 pthread_mutex_unlock 作为外部函数
extern int pthread_mutex_unlock(pthread_mutex_t *m);
extern int pthread_mutex_lock(pthread_mutex_t *m);

// 全局互斥锁和共享变量
pthread_mutex_t mutex = PTHREAD_MUTEX_INITIALIZER;
int shared_counter = 0;

// 测试 1：基本加锁和解锁
void* test_basic_lock_unlock(void* arg) {
    int thread_id = *(int*)arg;

    printf("Thread %d: Attempting to lock mutex...\n", thread_id);
    int ret = pthread_mutex_lock(&mutex);
    if (ret != 0) {
        printf("Thread %d: Failed to lock mutex, error: %d\n", thread_id, ret);
        return NULL;
    }
    printf("Thread %d: Mutex locked\n", thread_id);

    int old_value = shared_counter;
    usleep(100000); // 模拟工作
    shared_counter = old_value + 1;
    printf("Thread %d: Counter incremented to %d\n", thread_id, shared_counter);

    ret = pthread_mutex_unlock(&mutex);
    if (ret != 0) {
        printf("Thread %d: Failed to unlock mutex, error: %d\n", thread_id, ret);
        return NULL;
    }
    printf("Thread %d: Mutex unlocked\n", thread_id);

    return NULL;
}

// 测试 2：解锁未锁定的互斥锁
void* test_unlock_unlocked(void* arg) {
    int thread_id = *(int*)arg;

    printf("Thread %d: Attempting to unlock an unlocked mutex...\n", thread_id);
    int ret = pthread_mutex_unlock(&mutex);
    if (ret != 0) {
        printf("Thread %d: Unlock failed unexpectedly, error: %d (EPERM=%d)\n", thread_id, ret, EPERM);
    } else {
        printf("Thread %d: Unlock succeeded as expected!\n", thread_id);
    }

    return NULL;
}

// 测试 3：重复解锁
void* test_double_unlock(void* arg) {
    int thread_id = *(int*)arg;

    printf("Thread %d: Locking mutex...\n", thread_id);
    int ret = pthread_mutex_lock(&mutex);
    if (ret != 0) {
        printf("Thread %d: Failed to lock mutex, error: %d\n", thread_id, ret);
        return NULL;
    }

    printf("Thread %d: First unlock...\n", thread_id);
    ret = pthread_mutex_unlock(&mutex);
    if (ret != 0) {
        printf("Thread %d: First unlock failed, error: %d\n", thread_id, ret);
        return NULL;
    }

    printf("Thread %d: Attempting second unlock...\n", thread_id);
    ret = pthread_mutex_unlock(&mutex);
    if (ret != 0) {
        printf("Thread %d: Second unlock failed unexpectedly, error: %d (EPERM=%d)\n", thread_id, ret, EPERM);
    } else {
        printf("Thread %d: Second unlock succeeded as expected!\n", thread_id);
    }

    return NULL;
}

// 测试 4：死锁检测（故意不解锁）
void* test_deadlock(void* arg) {
    int thread_id = *(int*)arg;

    printf("Thread %d: Locking mutex (will not unlock)...\n", thread_id);
    int ret = pthread_mutex_lock(&mutex);
    if (ret != 0) {
        printf("Thread %d: Failed to lock mutex, error: %d\n", thread_id, ret);
        return NULL;
    }

    printf("Thread %d: Holding lock indefinitely...\n", thread_id);
    sleep(5); // 故意持有锁 5 秒，模拟死锁
    // 不调用 unlock，观察其他线程是否卡住

    return NULL;
}

int main() {
    pthread_t threads[THREAD_COUNT];
    int thread_ids[THREAD_COUNT] = {0, 1, 2, 3};

    printf("=== Test 1: Basic Lock and Unlock ===\n");
    for (int i = 0; i < THREAD_COUNT; i++) {
        pthread_create(&threads[i], NULL, test_basic_lock_unlock, &thread_ids[i]);
    }
    for (int i = 0; i < THREAD_COUNT; i++) {
        pthread_join(threads[i], NULL);
    }
    printf("Final counter: %d (expected %d)\n\n", shared_counter, THREAD_COUNT);

    printf("=== Test 2: Unlock Unlocked Mutex ===\n");
    pthread_create(&threads[0], NULL, test_unlock_unlocked, &thread_ids[0]);
    pthread_join(threads[0], NULL);
    printf("\n");

    printf("=== Test 3: Double Unlock ===\n");
    pthread_create(&threads[0], NULL, test_double_unlock, &thread_ids[0]);
    pthread_join(threads[0], NULL);
    printf("\n");

    // printf("=== Test 4: Deadlock Detection ===\n");
    // pthread_create(&threads[0], NULL, test_deadlock, &thread_ids[0]);
    // for (int i = 1; i < 3; i++) {
    //     pthread_create(&threads[i], NULL, test_basic_lock_unlock, &thread_ids[i]);
    // }
    // for (int i = 0; i < 3; i++) {
    //     pthread_join(threads[i], NULL);
    // }
    // pthread_mutex_unlock(&mutex); // 主线程解锁，避免程序卡死
    // printf("Main: Deadlock test completed\n");

    pthread_mutex_destroy(&mutex);
    return 0;
}
