#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>

#define NUM_THREADS 2
#define NUM_ITERATIONS 10000000

int counter = 0;
pthread_mutex_t lock;

extern int pthread_mutex_lock(pthread_mutex_t *m);
extern int pthread_mutex_init(pthread_mutex_t *m, const pthread_mutexattr_t *a);

void* increment(void* arg) {
    for (int i = 0; i < NUM_ITERATIONS; i++) {
    //    printf("Thread %ld trying to lock\n", pthread_self());
        pthread_mutex_lock(&lock);
    //    printf("Thread %ld acquired lock, counter = %d\n", pthread_self(), counter);
		int old = counter;
        counter++;
		if (counter != old + 1) {
            printf("Race detected: %d -> %d\n", old, counter);
		}
        pthread_mutex_unlock(&lock);
    }
    return NULL;
}

int main() {
    pthread_t threads[NUM_THREADS];

    // 初始化互斥锁
    if (pthread_mutex_init(&lock, NULL) != 0) {
       perror("pthread_mutex_init");
        return EXIT_FAILURE;
    }

    // 创建线程
    for (int i = 0; i < NUM_THREADS; i++) {
        if (pthread_create(&threads[i], NULL, increment, NULL) != 0) {
           perror("pthread_create");
            return EXIT_FAILURE;
        }
    }

    // 等待线程完成
    for (int i = 0; i < NUM_THREADS; i++) {
        pthread_join(threads[i], NULL);
    }

    // 销毁互斥锁
    pthread_mutex_destroy(&lock);

    // 打印最终计数值
    printf("Final counter value: %d\n", counter);
    return EXIT_SUCCESS;
}
