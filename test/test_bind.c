#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <unistd.h>
#include <string.h>

#define PASS   "✅ PASS"
#define SKIP   "⚠️ SKIP"
#define SFAIL  "❌ SOFT FAIL"
#define STRFAIL "❌❌ STRONG FAIL"

// 测试结果输出
#define TEST_RESULT(condition, msg) \
    do { \
        if (condition) { printf("[PSSS] %s: %s\n", PASS, msg); } \
        else { printf("[PSSS] %s: %s \n", STRFAIL, msg); } \
    } while (0)

#define TEST_EXPECT_FAIL(condition, msg) \
    do { \
        if (condition) { printf("[PSSS] %s: %s\n", SFAIL, msg); } \
        else { printf("[PSSS] %s: %s\n", PASS, msg); } \
    } while (0)

int main() {
    printf("\n=== Running bind() tests ===\n");

    // ✅ 创建 IPv4 TCP 套接字
    int sock = socket(AF_INET, SOCK_STREAM, 0);
    TEST_RESULT(sock >= 0, "Created IPv4 TCP socket");

    // ✅ 绑定到 127.0.0.1:8080
    struct sockaddr_in addr = {
        .sin_family = AF_INET,
        .sin_port = htons(8080),
        .sin_addr.s_addr = inet_addr("127.0.0.1")
    };

    int bind_res = bind(sock, (struct sockaddr*)&addr, sizeof(addr));
    TEST_RESULT(bind_res == 0, "Bound socket to 127.0.0.1:8080");

    // ❌ 绑定到非法端口（负数）
    addr.sin_port = htons(-1);
    int bind_invalid = bind(sock, (struct sockaddr*)&addr, sizeof(addr));
    TEST_EXPECT_FAIL(bind_invalid == 0, "bind() with invalid port (-1)");

    // ❌ 绑定到 0.0.0.0:99999（非法端口）
    addr.sin_port = htons(99999);
    int bind_invalid_port = bind(sock, (struct sockaddr*)&addr, sizeof(addr));
    TEST_EXPECT_FAIL(bind_invalid_port == 0, "bind() with out-of-range port (99999)");

    // ✅ 关闭套接字
    // if (sock >= 0) close(sock);

    printf("\n=== All bind() tests completed ===\n");
    return 0;
}
