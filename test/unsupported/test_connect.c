#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <errno.h>
#include <unistd.h>
#include <string.h>
#include <pthread.h>

#define PASS   "✅ PASS"
#define SFAIL  "❌ SOFT FAIL"
#define STRFAIL "❌❌ STRONG FAIL"
#define SKIP   "⚠️ SKIP"

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

#define SERVER_PORT 8082
#define TEST_MESSAGE "Hello from client"
#define BUFFER_SIZE 128

// 服务器线程
void* server_thread(void* arg) {
    int server_sock, client_sock;
    struct sockaddr_in server_addr, client_addr;
    socklen_t client_len = sizeof(client_addr);
    char buffer[BUFFER_SIZE];

    server_sock = socket(AF_INET, SOCK_STREAM, 0);
    TEST_RESULT(server_sock >= 0, "Created server socket");

    memset(&server_addr, 0, sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    server_addr.sin_port = htons(SERVER_PORT);
    server_addr.sin_addr.s_addr = INADDR_ANY;

    int bind_res = bind(server_sock, (struct sockaddr*)&server_addr, sizeof(server_addr));
    TEST_RESULT(bind_res == 0, "Bound server to port 8082");

    int listen_res = listen(server_sock, 5);
    TEST_RESULT(listen_res == 0, "Listening on port 8082");

    client_sock = accept(server_sock, (struct sockaddr*)&client_addr, &client_len);
    TEST_RESULT(client_sock >= 0, "Accepted client connection");

    memset(buffer, 0, BUFFER_SIZE);
    ssize_t recv_res = recv(client_sock, buffer, BUFFER_SIZE, 0);
    TEST_RESULT(recv_res > 0, "Received data from client");

    if (recv_res > 0) {
        TEST_RESULT(strcmp(buffer, TEST_MESSAGE) == 0, "Received correct message from client");
    }

    close(client_sock);
    close(server_sock);
    return NULL;
}

int main() {
    pthread_t server_tid;
    printf("\n=== Running bind(), listen(), connect() tests ===\n");

    pthread_create(&server_tid, NULL, server_thread, NULL);
    sleep(1);

    // ✅ 创建客户端 socket
    int client_sock = socket(AF_INET, SOCK_STREAM, 0);
    TEST_RESULT(client_sock >= 0, "Created client socket");

    struct sockaddr_in server_addr;
    memset(&server_addr, 0, sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    server_addr.sin_port = htons(SERVER_PORT);
    server_addr.sin_addr.s_addr = inet_addr("127.0.0.1");

    int connect_res = connect(client_sock, (struct sockaddr*)&server_addr, sizeof(server_addr));
    TEST_RESULT(connect_res == 0, "Connected to server on 127.0.0.1:8082");

    ssize_t send_res = send(client_sock, TEST_MESSAGE, strlen(TEST_MESSAGE), 0);
    TEST_RESULT(send_res > 0, "Sent data to server");

    close(client_sock);

    // ❌ 连接未监听的端口
    int invalid_sock = socket(AF_INET, SOCK_STREAM, 0);
    struct sockaddr_in invalid_addr = server_addr;
    invalid_addr.sin_port = htons(9999);
    int invalid_connect = connect(invalid_sock, (struct sockaddr*)&invalid_addr, sizeof(invalid_addr));
    TEST_EXPECT_FAIL(invalid_connect == 0, "connect() to unbound port 9999");
    close(invalid_sock);

    // ❌ UDP socket 调用 connect()
    int udp_sock = socket(AF_INET, SOCK_DGRAM, 0);
    int udp_connect = connect(udp_sock, (struct sockaddr*)&server_addr, sizeof(server_addr));
    TEST_EXPECT_FAIL(udp_connect == 0, "connect() using UDP socket");
    close(udp_sock);

    // ❌ 连接无效 IP
    struct sockaddr_in bad_addr = server_addr;
    in_addr_t invalid_ip = inet_addr("256.256.256.256");

    if (invalid_ip == INADDR_NONE) {
        printf("[PSSS] ✅ PASS: inet_addr() correctly detected invalid IP\n");
    } else {
        bad_addr.sin_addr.s_addr = invalid_ip;
        int bad_connect = connect(client_sock, (struct sockaddr*)&bad_addr, sizeof(bad_addr));
        TEST_EXPECT_FAIL(bad_connect == 0, "connect() to invalid IP address");
    }

    pthread_join(server_tid, NULL);
    printf("\n=== All tests completed ===\n");
    return 0;
}
