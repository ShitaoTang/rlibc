#include <stdio.h>
#include <pthread.h>

pthread_mutex_t lock;
int counter = 0;

extern int pthread_mutex_lock(pthread_mutex_t *m);

void test_mutex() {
    for (int i = 0; i < 1000000; i++) {
        pthread_mutex_lock(&lock);
        counter++;
        pthread_mutex_unlock(&lock);
    }
}

int main() {
    // 初始化互斥锁
    pthread_mutex_init(&lock, NULL);

    test_mutex();

    // 销毁互斥锁
    pthread_mutex_destroy(&lock);

    printf("Final counter value: %d\n", counter);
    return 0;
}
