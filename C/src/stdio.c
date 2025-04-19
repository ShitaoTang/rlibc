#include "stdarg.h"
#include "stddef.h"
#include "string.h"
#include "unistd.h"
#include <sys/types.h>

int printf_intern(int fd, const char *fmt, va_list args)
{
    va_list args_copy;
    va_copy(args_copy, args);

    const char* p;
    char buffer[1024];
    int len = 0;

    for (p = fmt; *p; p++) {
        if (*p != '%') {
            buffer[len++] = *p;
            continue;
        }

        p++;
        if (*p == 'l') {
            p++;  
            if (*p == 'd') {
                long int num = va_arg(args_copy, long int);
                len += ltoa(num, &buffer[len]);
                continue;
            } else {
                buffer[len++] = '%';
                buffer[len++] = 'l';
                buffer[len++] = *p;
                continue;
            }
        } else if (*p == 'z') {
            p++;
            if (*p == 'u') {
                size_t num = va_arg(args_copy, size_t);
                if (sizeof(size_t) == sizeof(unsigned int)) {
                    len += itoa((unsigned int)num, &buffer[len]); // 32-bit
                } else {
                    len += ltoa((unsigned long)num, &buffer[len]); // 64-bit
                }
                continue;
            }
        }
        switch (*p) {
        case 's': {
            char* s = va_arg(args_copy, char*);
            if (s==NULL) s = "(null)";
            size_t l = rstrlen(s);
            for (size_t i = 0; i < l;) buffer[len++] = s[i++];
            break;
        }
        case 'd': {
            int num = va_arg(args_copy, int);
            len += itoa(num, &buffer[len]);
            break;
        }
        case '%': {
            buffer[len++] = '%';
            break;
        }
        default:
            buffer[len++] = '%';
            buffer[len++] = *p;
            break;
        }
    }

    va_end(args_copy);

    write(fd, buffer, len);
    return len;
}

int printf(const char *fmt, ...)
{
    va_list args;
    va_start(args, fmt);

    int ret = printf_intern(1, fmt, args);

    va_end(args);
    return ret;
}

int fdprintf(int fd, const char *fmt, ...)
{
    va_list args;
    va_start(args, fmt);

    int ret = printf_intern(fd, fmt, args);

    va_end(args);
    return ret;
}

int puts(const char *s)
{
    size_t len = rstrlen(s);
    ssize_t written = write(1, s, len);
    if (written < 0) return -1;

    written = write(1, "\n", 1);
    if (written < 0) return -1;

    return 0;
}

int putchar(int c)
{
    return write(1, &c, 1);
}

void perror(const char *s)
{
    fdprintf(2, "%s\n", s);
}