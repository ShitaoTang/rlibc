#include "rlibc.h"

int rstrlen(const char *s)
{
    int len = 0;
    while (*s++) len++;
    return len;
}

int itoa(int n, char *buf)
{
    char tmp[20];
    int i = 0;
    int neg = n < 0;
    unsigned int num = neg ? -n : n;

    do {
        tmp[i++] = (num % 10) + '0';
        num /= 10;
    } while (num > 0);
    if (neg) {
        tmp[i++] = '-';
    }

    for (int j = 0; j < i; ++j) {
        buf[j] = tmp[i-j-1];
    }
    return i;
}

int ltoa(long int n, char *buf)
{
    char tmp[32]; 
    int i = 0;
    int neg = n < 0;
    unsigned int num = neg ? -n : n;

    do {
        tmp[i++] = (num % 10) + '0';
        num /= 10;
    } while (num > 0);
    if (neg) {
        tmp[i++] = '-';
    }

    for (int j = 0; j < i; ++j) {
        buf[j] = tmp[i-j-1];
    }
    return i;
}

ssize_t write(int fd, const void *buf, size_t count)
{
    ssize_t ret;
#ifdef __x86_64__
    /* Arch       Instruction      System Call    Ret Value
     * x86-64      syscall           rax            rax
     *
     * Arch         arg1             ...         arg6
     * x86-64       rdi   rsi   rdx   r10   r8    r9
     */
    __asm__ volatile (
        "mov %1, %%rdi\n"
        "mov %2, %%rsi\n"
        "mov %3, %%rdx\n"
        "mov $1, %%rax\n"
        "syscall\n"
        "mov %%rax, %0\n"
        : "=r" (ret)
        : "r" ((long)fd), "r" (buf), "r" (count)
        : "%rax", "%rdi", "%rsi", "%rdx"
    );
#elif defined(__aarch64__)
    /* Arch       Instruction      System Call    Ret Value
     * aarch64     svc #0             w8            x0
     *
     * Arch         arg1             ...         arg6
     * arm64        x0    x1    x2    x3    x4    x5
     */
    __asm__ volatile {
        "mov x0, %1\n"
        "mov x1, %2\n"
        "mov x2, %3\n"
        "mov x8, #64\n"
        "svc #0\n"
        "mov %0, x0\n"
        : "=r" (ret)
        : "r" ((long)fd), "r" (buf), "r" (count)
        : "%x0", "%x1", "%x2", "%x8"
    }
#else
    #error "Unsupported architecture"
#endif
    return ret;
}
