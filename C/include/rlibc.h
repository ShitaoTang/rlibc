#ifndef RLIBC_H
#define RLIBC_H

#include <stddef.h>
#include <stdint.h>
#include <stdarg.h>
#include <sys/types.h>

#define assert(x) \
    do { \
        if (!(x)) { \
            __assert("Assertion failed: " #x ", file " __FILE__ ", line " TOSTR(__LINE__) "\n"); \
        } \
    } while (0)

#define TOSTR(x) STR(x)
#define STR(x) #x

int rstrlen(const char *s);
int itoa(int n, char *buf);
int printf(const char *fmt, ...);
int fdprintf(int fd, const char *fmt, ...);
ssize_t write(int fd, const void *buf, size_t count);
void __assert(const char *msg);
void ___exit(int status);

#endif