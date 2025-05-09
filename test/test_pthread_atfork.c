#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

static void prepare(void) {
    printf("✅ prepare called before fork (simulated)\n");
}

static void parent(void) {
    printf("✅ parent called after fork (simulated in parent)\n");
}

static void child(void) {
    printf("✅ child called after fork (simulated in new thread)\n");
}

void* child_thread(void* arg) {
    child();
    return NULL;
}

void test_pthread_atfork() {
    if (pthread_atfork(prepare, parent, child) != 0) {
        printf("❌ pthread_atfork registration failed\n");
        exit(1);
    }

    printf("🔹 Simulating fork using pthread...\n");
    prepare();

    pthread_t tid;
    if (pthread_create(&tid, NULL, child_thread, NULL) != 0) {
        printf("❌ pthread_create failed\n");
        exit(1);
    }

    parent();
    pthread_join(tid, NULL);
    printf("✅ Simulated child thread exited successfully\n");
}

int main() {
    test_pthread_atfork();
    return 0;
}