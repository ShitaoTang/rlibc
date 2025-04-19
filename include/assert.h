#include <features.h>

#undef assert

#ifdef NDEBUG
#define	assert(x) (void)0
#else
#define assert(x) \
    do { \
        if (!(x)) { \
            __assert("Assertion failed: " #x ", file " __FILE__ ", line " TOSTR(__LINE__) "\n"); \
        } \
    } while (0)

#define TOSTR(x) STR(x)
#define STR(x) #x
#endif

void __assert(const char *msg);

#if __STDC_VERSION__ >= 201112L && !defined(__cplusplus)
#define static_assert _Static_assert
#endif

#ifdef __cplusplus
extern "C" {
#endif

_Noreturn void __assert_fail (const char *, const char *, int, const char *);

#ifdef __cplusplus
}
#endif
