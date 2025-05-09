#include <stdio.h>
// #include <stdlib.h>
#include <pthread.h>
// #include <unistd.h>

#define NUM_THREADS 3

pthread_barrier_t barrier;  // 线程栅栏

void *worker(void *arg) {
    long id = (long)arg;
    printf("Thread %ld is doing some work...\n", id+2);
    // sleep(id + 1); // 模拟不同线程的不同工作时间

    printf("Thread %ld is waiting at the barrier...\n", id+2);
    int ret = pthread_barrier_wait(&barrier);

    if (ret == -1) {
        printf("Thread %ld is the last one to arrive!\n", id+2);
    }

    printf("Thread %ld has passed the barrier!\n", id+2);
    return NULL;
}

int main() {
    pthread_t threads[NUM_THREADS];

    // 初始化 barrier，3 个线程需要同步
    pthread_barrier_init(&barrier, NULL, NUM_THREADS);

    // 创建线程
    for (long i = 0; i < NUM_THREADS; i++) {
        pthread_create(&threads[i], NULL, worker, (void *)i);
    }

    // 等待所有线程完成
    for (int i = 0; i < NUM_THREADS; i++) {
        pthread_join(threads[i], NULL);
    }

    // 销毁 barrier
	printf("Destroying barrier now...\n");
    pthread_barrier_destroy(&barrier);
    return 0;
}
