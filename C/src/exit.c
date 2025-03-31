#include "../include/rlibc.h"

void ___exit(int status)
{
#ifdef __x86_64__
    __asm__ volatile (
        "mov %0, %%rdi\n"
        "mov $60, %%rax\n"
        "syscall\n"
        :
        : "r" ((long)status)
        : "%rdi", "%rax"
    );
#elif defined(__aarch64__)
    __asm__ volatile (
        "mov x0, %0\n"
        "mov x8, #93\n"
        "svc #0\n"
        :
        : "r" ((long)status)
        : "%x0", "%x8"
    );
#else
    #error "Unsupported architecture"
#endif
    __builtin_unreachable();
}