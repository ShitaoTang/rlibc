#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <unistd.h>

pthread_mutex_t mutex = PTHREAD_MUTEX_INITIALIZER;
pthread_cond_t cond = PTHREAD_COND_INITIALIZER;

extern int pthread_cond_wait(pthread_cond_t *c, pthread_mutex_t *m);

void *thread_func(void *arg) {
    pthread_mutex_lock(&mutex);
    printf("Thread waiting on condition variable...\n");
    pthread_cond_wait(&cond, &mutex);
    printf("Thread resumed after condition signal.\n");
    pthread_mutex_unlock(&mutex);
    return NULL;
}

int main() {
    pthread_t thread;
    pthread_create(&thread, NULL, thread_func, NULL);
    
    sleep(2); // Simulate some work before signaling
    
    pthread_mutex_lock(&mutex);
    printf("Main thread signaling condition variable...\n");
    pthread_cond_signal(&cond);
    pthread_mutex_unlock(&mutex);
    
    pthread_join(thread, NULL);
    return 0;
}

