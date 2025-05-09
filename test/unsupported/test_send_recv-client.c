#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>
#include <assert.h>

#define SERVER_IP "127.0.0.1"
#define PORT 12345
#define BUFFER_SIZE 1024

void test_send(int sock) {
    const char *message = "Hello, Server";
    ssize_t bytes_sent = send(sock, message, strlen(message) + 1, 0);
    
    // 断言：确保 send() 成功
    assert(bytes_sent == (ssize_t)(strlen(message) + 1));
    printf("[TEST] send(): PASSED\n");
}

int main() {
    int sock;
    struct sockaddr_in server_addr;

    // 创建 socket
    sock = socket(AF_INET, SOCK_STREAM, 0);
    assert(sock >= 0);

    // 设置服务器地址
    server_addr.sin_family = AF_INET;
    server_addr.sin_port = htons(PORT);
    assert(inet_pton(AF_INET, SERVER_IP, &server_addr.sin_addr) > 0);

    // 连接服务器
    assert(connect(sock, (struct sockaddr *)&server_addr, sizeof(server_addr)) == 0);
    printf("[INFO] Connected to server.\n");

    // 测试 send()
    test_send(sock);

    // 关闭 socket
    close(sock);
    return 0;
}
