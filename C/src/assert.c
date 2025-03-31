#include "../include/rlibc.h"

void __assert(const char *msg)
{
    fdprintf(2, "%s\n", msg);
    ___exit(42);
}