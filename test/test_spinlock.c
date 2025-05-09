#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <time.h>

#define NUM_ITERATIONS 1000000

pthread_spinlock_t spinlock;
extern int pthread_spin_init(pthread_spinlock_t *s, int shared);
extern int pthread_spin_lock(pthread_spinlock_t *s);
extern int pthread_spin_unlock(pthread_spinlock_t *s);
extern int pthread_spin_destory(pthread_spinlock_t *s);
volatile int counter = 0;

void *increment_counter(void *arg) {
    for (int i = 0; i < NUM_ITERATIONS; i++) {
        pthread_spin_lock(&spinlock);
        counter++;
        pthread_spin_unlock(&spinlock);
    }
    return NULL;
}

int main() {
    pthread_t thread1, thread2;
    struct timespec start, end;
    
    // 初始化自旋锁
    if (pthread_spin_init(&spinlock, PTHREAD_PROCESS_PRIVATE) != 0) {
        perror("pthread_spin_init");
        return EXIT_FAILURE;
    }
    
    // 记录开始时间
    clock_gettime(CLOCK_MONOTONIC, &start);
    
    // 创建两个线程
    pthread_create(&thread1, NULL, increment_counter, NULL);
    pthread_create(&thread2, NULL, increment_counter, NULL);
    
    // 等待线程执行完毕
    pthread_join(thread1, NULL);
    pthread_join(thread2, NULL);
    
    // 记录结束时间
    clock_gettime(CLOCK_MONOTONIC, &end);
    
    // 计算执行时间（秒和纳秒）
    double elapsed_time = (end.tv_sec - start.tv_sec) + (end.tv_nsec - start.tv_nsec) / 1e9;
    
    // 结果验证
    printf("Expected counter value: %d\n", 2 * NUM_ITERATIONS);
    printf("Actual counter value:   %d\n", counter);
    printf("Test %s\n", (counter == 2 * NUM_ITERATIONS) ? "PASSED" : "FAILED");
    printf("Execution time: %.6f seconds\n", elapsed_time);
    
    // 销毁自旋锁
    pthread_spin_destroy(&spinlock);
    
    return (counter == 2 * NUM_ITERATIONS) ? EXIT_SUCCESS : EXIT_FAILURE;
}

