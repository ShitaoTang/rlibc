#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <unistd.h>

// 宏用于打印测试结果
#define CHECK_RESULT(cond, msg) \
    do { \
        if (cond) { \
            printf("✅ %s\n", msg); \
        } else { \
            printf("❌ %s\n", msg); \
        } \
    } while (0)

// 测试 getsockname()
void test_getsockname() {
    int sockfd = socket(AF_INET, SOCK_STREAM, 0);
    assert(sockfd >= 0);

    struct sockaddr_in addr;
    socklen_t addrlen = sizeof(addr);

    memset(&addr, 0, sizeof(addr));
    addr.sin_family = AF_INET;
    addr.sin_addr.s_addr = htonl(INADDR_ANY);
    addr.sin_port = 0; // 让系统分配端口

    assert(bind(sockfd, (struct sockaddr *)&addr, sizeof(addr)) == 0);

    struct sockaddr_in actual_addr;
    socklen_t actual_len = sizeof(actual_addr);
    assert(getsockname(sockfd, (struct sockaddr *)&actual_addr, &actual_len) == 0);

    CHECK_RESULT(actual_addr.sin_family == AF_INET, "getsockname() 获取地址成功");
    close(sockfd);
}

// 测试 getpeername()
void test_getpeername() {
    int server_fd = socket(AF_INET, SOCK_STREAM, 0);
    assert(server_fd >= 0);

    struct sockaddr_in server_addr;
    memset(&server_addr, 0, sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    server_addr.sin_port = htons(8081); // 改用 8081 端口，避免和系统已有服务冲突

    assert(bind(server_fd, (struct sockaddr *)&server_addr, sizeof(server_addr)) == 0);
    assert(listen(server_fd, 1) == 0);

    // 创建客户端 socket 并连接到服务器
    int client_fd = socket(AF_INET, SOCK_STREAM, 0);
    assert(client_fd >= 0);
    assert(connect(client_fd, (struct sockaddr *)&server_addr, sizeof(server_addr)) == 0);

    // 服务器端接受连接
    struct sockaddr_in client_addr;
    socklen_t client_len = sizeof(client_addr);
    int conn_fd = accept(server_fd, (struct sockaddr *)&client_addr, &client_len);
    assert(conn_fd >= 0);

    // 测试 getpeername()
    struct sockaddr_in peer_addr;
    socklen_t peer_len = sizeof(peer_addr);
    assert(getpeername(client_fd, (struct sockaddr *)&peer_addr, &peer_len) == 0);

    CHECK_RESULT(peer_addr.sin_family == AF_INET, "getpeername() 获取对端地址成功");

    close(client_fd);
    close(conn_fd);
    close(server_fd);
}

// 测试 getsockopt()
void test_getsockopt() {
    int sockfd = socket(AF_INET, SOCK_STREAM, 0);
    assert(sockfd >= 0);

    int optval;
    socklen_t optlen = sizeof(optval);

    assert(getsockopt(sockfd, SOL_SOCKET, SO_REUSEADDR, &optval, &optlen) == 0);
    CHECK_RESULT(optlen == sizeof(optval), "getsockopt() 获取 SO_REUSEADDR 选项成功");

    close(sockfd);
}

// 测试 setsockopt()
void test_setsockopt() {
    int sockfd = socket(AF_INET, SOCK_STREAM, 0);
    assert(sockfd >= 0);

    int optval = 1;
    assert(setsockopt(sockfd, SOL_SOCKET, SO_REUSEADDR, &optval, sizeof(optval)) == 0);

    int actual_val;
    socklen_t optlen = sizeof(actual_val);
    assert(getsockopt(sockfd, SOL_SOCKET, SO_REUSEADDR, &actual_val, &optlen) == 0);

    CHECK_RESULT(actual_val == 1, "setsockopt() 成功设置 SO_REUSEADDR 选项");

    close(sockfd);
}

// 测试 shutdown()
void test_shutdown() {
    int sockfd = socket(AF_INET, SOCK_STREAM, 0);
    assert(sockfd >= 0);

    struct sockaddr_in server_addr;
    memset(&server_addr, 0, sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    server_addr.sin_port = htons(8081); // 使用一个不常用的端口

    assert(bind(sockfd, (struct sockaddr *)&server_addr, sizeof(server_addr)) == 0);
    assert(listen(sockfd, 1) == 0);

    int client_fd = socket(AF_INET, SOCK_STREAM, 0);
    assert(client_fd >= 0);
    assert(connect(client_fd, (struct sockaddr *)&server_addr, sizeof(server_addr)) == 0);

    // 关闭写端
    assert(shutdown(client_fd, SHUT_WR) == 0);
    CHECK_RESULT(1, "shutdown() 关闭写端成功");

    close(client_fd);
    close(sockfd);
}

int main() {
    printf("===== 开始测试 socket 控制函数 =====\n");

    test_getsockname();
    test_getpeername();
    test_getsockopt();
    test_setsockopt();
    test_shutdown();

    printf("===== 测试完成 =====\n");

    return 0;
}
