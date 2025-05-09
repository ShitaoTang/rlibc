#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <errno.h>
#include <unistd.h>
#include <string.h>

#define PASS   "✅ PASS"
#define SKIP   "⚠️ SKIP"
#define SFAIL  "❌ SOFT FAIL"
#define STRFAIL "❌❌ STRONG FAIL"

// 封装测试结果输出
#define TEST_RESULT(condition, msg) \
    do { \
        if (condition) { printf("[PSSS] %s: %s\n", PASS, msg); } \
        else { printf("[PSSS] %s: %s (errno: %d, %s)\n", STRFAIL, msg, errno, strerror(errno)); } \
    } while (0)

#define TEST_EXPECT_FAIL(condition, msg) \
    do { \
        if (condition) { printf("[PSSS] %s: %s\n", SFAIL, msg); } \
        else { printf("[PSSS] %s: %s\n", PASS, msg); } \
    } while (0)

int main() {
    printf("\n=== Running socket() tests ===\n");

    // ✅ 测试创建 IPv4 套接字
    int sock_v4 = socket(AF_INET, SOCK_STREAM, 0);
    TEST_RESULT(sock_v4 >= 0, "Created IPv4 TCP socket");

    int sock_v4_udp = socket(AF_INET, SOCK_DGRAM, 0);
    TEST_RESULT(sock_v4_udp >= 0, "Created IPv4 UDP socket");

    // ✅ 测试创建 IPv6 套接字
    int sock_v6 = socket(AF_INET6, SOCK_STREAM, 0);
    TEST_RESULT(sock_v6 >= 0, "Created IPv6 TCP socket");

    int sock_v6_udp = socket(AF_INET6, SOCK_DGRAM, 0);
    TEST_RESULT(sock_v6_udp >= 0, "Created IPv6 UDP socket");

    // ✅ 测试创建 Unix 域套接字
    int sock_unix = socket(AF_UNIX, SOCK_STREAM, 0);
    TEST_RESULT(sock_unix >= 0, "Created Unix domain socket");

    // ✅ 测试 SOCK_CLOEXEC 标志（如果支持）
#ifdef SOCK_CLOEXEC
    int sock_cloexec = socket(AF_INET, SOCK_STREAM | SOCK_CLOEXEC, 0);
    TEST_RESULT(sock_cloexec >= 0, "Created socket with SOCK_CLOEXEC");
#else
    printf("[PSSS] %s: System does not support SOCK_CLOEXEC\n", SKIP);
#endif

    // ❌ 测试非法 domain
    int sock_invalid_domain = socket(-1, SOCK_STREAM, 0);
    TEST_EXPECT_FAIL(sock_invalid_domain >= 0, "socket() with invalid domain (-1)");

    // ❌ 测试非法 type
    int sock_invalid_type = socket(AF_INET, -1, 0);
    TEST_EXPECT_FAIL(sock_invalid_type >= 0, "socket() with invalid type (-1)");

    // ❌ 测试非法 protocol
    int sock_invalid_proto = socket(AF_INET, SOCK_STREAM, -1);
    TEST_EXPECT_FAIL(sock_invalid_proto >= 0, "socket() with invalid protocol (-1)");

    // ✅ 关闭所有打开的 socket
    if (sock_v4 >= 0) close(sock_v4);
    if (sock_v4_udp >= 0) close(sock_v4_udp);
    if (sock_v6 >= 0) close(sock_v6);
    if (sock_v6_udp >= 0) close(sock_v6_udp);
    if (sock_unix >= 0) close(sock_unix);
#ifdef SOCK_CLOEXEC
    if (sock_cloexec >= 0) close(sock_cloexec);
#endif

    printf("\n=== All tests completed ===\n");
    return 0;
}
