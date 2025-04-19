#include "assert.h"
#include "stdio.h"
#include "stdlib.h"

void __assert(const char *msg)
{
    fdprintf(2, "%s\n", msg);
    ___exit(42);
}