#include <stdio.h>
#include <pthread.h>
#include <assert.h>

extern int pthread_mutex_init(pthread_mutex_t *mutex, const pthread_mutexattr_t *attr);
extern int pthread_mutex_lock(pthread_mutex_t *mutex);

/**
 * 测试普通的互斥锁初始化
 */
void test_normal_mutex() {
    pthread_mutex_t mutex;
    int ret = pthread_mutex_init(&mutex, NULL);
    assert(ret == 0);

    ret = pthread_mutex_lock(&mutex);
    assert(ret == 0);

    printf("test_normal_mutex passed.\n");
}

/**
 * 测试递归锁
 */
void test_recursive_lock() {
    pthread_mutex_t mutex;
    pthread_mutexattr_t attr;

    pthread_mutexattr_init(&attr);
    pthread_mutexattr_settype(&attr, PTHREAD_MUTEX_RECURSIVE);

    int ret = pthread_mutex_init(&mutex, &attr);
    assert(ret == 0);

    ret = pthread_mutex_lock(&mutex);
    assert(ret == 0);

    ret = pthread_mutex_lock(&mutex);  // 递归加锁
    assert(ret == 0);

    printf("test_recursive_lock passed.\n");
}

/**
 * 测试死锁情况
 */
pthread_mutex_t deadlock_mutex1, deadlock_mutex2;

void* deadlock_thread_func(void* arg) {
    pthread_mutex_lock(&deadlock_mutex2);
    printf("Thread 2 locked mutex2\n");

    pthread_mutex_lock(&deadlock_mutex1); // 这里可能会死锁
    printf("Thread 2 locked mutex1\n");

    return NULL;
}

void test_deadlock() {
    pthread_mutex_init(&deadlock_mutex1, NULL);
    pthread_mutex_init(&deadlock_mutex2, NULL);

    pthread_mutex_lock(&deadlock_mutex1);
    printf("Main thread locked mutex1\n");

    pthread_t t;
    pthread_create(&t, NULL, deadlock_thread_func, NULL);

    pthread_mutex_lock(&deadlock_mutex2);  // 这里会导致死锁
    printf("Main thread locked mutex2\n");

    pthread_join(t, NULL);
    printf("test_deadlock finished (should not reach here if deadlock occurs).\n");
}

/**
 * 测试未初始化的锁
 */
void test_uninitialized_mutex() {
    pthread_mutex_t mutex;  // 未初始化
    int ret = pthread_mutex_lock(&mutex);  // 直接加锁
    assert(ret != 0);  // 预期加锁失败

    printf("test_uninitialized_mutex passed.\n");
}

int main() {
    test_normal_mutex();
    test_recursive_lock();
    // test_deadlock();
    test_uninitialized_mutex();
    return 0;
}
