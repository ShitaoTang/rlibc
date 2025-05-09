#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <assert.h>

int n, depth = 0;
pthread_mutex_t lk = PTHREAD_MUTEX_INITIALIZER;
pthread_cond_t cv = PTHREAD_COND_INITIALIZER;

#define CAN_PRODUCE (depth < n)
#define CAN_CONSUME (depth > 0)

extern int pthread_cond_wait(pthread_cond_t *cond, pthread_mutex_t *mutex);

void *T_produce(void *arg) {
    while (1) {
        pthread_mutex_lock(&lk);

        while (!CAN_PRODUCE) {
            pthread_cond_wait(&cv, &lk);
        }

        assert(CAN_PRODUCE);

        printf("(");
        depth++;

        pthread_cond_broadcast(&cv);
        pthread_mutex_unlock(&lk);
    }
    return NULL;
}

void *T_consume(void *arg) {
    while (1) {
        pthread_mutex_lock(&lk);

        while (!CAN_CONSUME) {
            pthread_cond_wait(&cv, &lk);
        }

        printf(")");
        depth--;

        pthread_cond_broadcast(&cv);
        pthread_mutex_unlock(&lk);
    }
    return NULL;
}

int main(int argc, char *argv[]) {
    if (argc < 3) {
        fdprintf(2, "Usage: %s depth num-thread-pairs\n", argv[0]);
        return 1;
    }
    n = atoi(argv[1]);
    int t = atoi(argv[2]);

    pthread_t producers[t], consumers[t];

    for (int i = 0; i < t; i++) {
        pthread_create(&producers[i], NULL, T_produce, NULL);
        pthread_create(&consumers[i], NULL, T_consume, NULL);
    }

    for (int i = 0; i < t; i++) {
        pthread_join(producers[i], NULL);
        pthread_join(consumers[i], NULL);
    }

    return 0;
}

