#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <errno.h>
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
        else { printf("[PSSS] %s: %s (errno: %d, %s)\n", STRFAIL, msg, errno, strerror(errno)); } \
    } while (0)

#define TEST_EXPECT_FAIL(condition, msg) \
    do { \
        if (condition) { printf("[PSSS] %s: %s\n", SFAIL, msg); } \
        else { printf("[PSSS] %s: %s\n", PASS, msg); } \
    } while (0)

int main() {
    printf("\n=== Running listen() tests ===\n");

    // ✅ 创建 TCP 套接字
    int sock = socket(AF_INET, SOCK_STREAM, 0);
    TEST_RESULT(sock >= 0, "Created IPv4 TCP socket");

    // ✅ 绑定到 127.0.0.1:8081
    struct sockaddr_in addr = {
        .sin_family = AF_INET,
        .sin_port = htons(8081),
        .sin_addr.s_addr = inet_addr("127.0.0.1")
    };

    int bind_res = bind(sock, (struct sockaddr*)&addr, sizeof(addr));
    TEST_RESULT(bind_res == 0, "Bound socket to 127.0.0.1:8081");

    // ✅ 监听，最大连接队列 5
    int listen_res = listen(sock, 5);
    TEST_RESULT(listen_res == 0, "Listening on 127.0.0.1:8081 with backlog 5");

    // ❌ 在未绑定的 socket 上调用 listen()
    int sock_unbound = socket(AF_INET, SOCK_STREAM, 0);
    int listen_unbound = listen(sock_unbound, 5);
    TEST_EXPECT_FAIL(listen_unbound == 0, "listen() on unbound socket");

    // ❌ 在 UDP socket 上调用 listen()（无效）
    int sock_udp = socket(AF_INET, SOCK_DGRAM, 0);
    int listen_udp = listen(sock_udp, 5);
    TEST_EXPECT_FAIL(listen_udp == 0, "listen() on UDP socket");

    // ✅ 关闭所有打开的 socket
    if (sock >= 0) close(sock);
    if (sock_unbound >= 0) close(sock_unbound);
    if (sock_udp >= 0) close(sock_udp);

    printf("\n=== All listen() tests completed ===\n");
    return 0;
}
