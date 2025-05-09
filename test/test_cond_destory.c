#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

pthread_mutex_t mutex = PTHREAD_MUTEX_INITIALIZER;
pthread_cond_t cond = PTHREAD_COND_INITIALIZER;
int ready = 0;

void* thread_func(void* arg) {
    pthread_mutex_lock(&mutex);
    while (!ready) {
        printf("Thread waiting on condition variable...\n");
        pthread_cond_wait(&cond, &mutex);
    }
    printf("Thread received signal!\n");
    pthread_mutex_unlock(&mutex);
    return NULL;
}

int main() {
    pthread_t thread;

    // 创建线程，线程会在条件变量上等待
    if (pthread_create(&thread, NULL, thread_func, NULL) != 0) {
        perror("pthread_create");
        exit(EXIT_FAILURE);
    }

    sleep(1); // 确保线程先进入等待状态

    // 发送信号，唤醒等待的线程
    pthread_mutex_lock(&mutex);
    ready = 1;
    pthread_cond_signal(&cond);
    pthread_mutex_unlock(&mutex);

    // 等待线程结束
    pthread_join(thread, NULL);

    // 尝试销毁条件变量
    if (pthread_cond_destroy(&cond) == 0) {
        printf("Condition variable destroyed successfully.\n");
    } else {
        perror("pthread_cond_destroy failed");
    }

    return 0;
}
