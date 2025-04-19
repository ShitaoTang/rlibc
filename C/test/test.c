#include <stdio.h>
#include <stdlib.h>
#include <assert.h>

int main() {
    printf("Hello, World!\n");
    printf("This is a test string: %s\n", "Hello, rlibc!");
    printf("Integer test: %d\n", 42);
    printf("Print emoji: ✅ ❌\n");

    assert(1 == 1);
    assert(1 == 0);
    ___exit(17);
}