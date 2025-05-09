#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>
#include <assert.h>

#define PORT 12345
#define BUFFER_SIZE 1024

void test_recv(int client_socket) {
    char buffer[BUFFER_SIZE] = {0};
    ssize_t bytes_received = recv(client_socket, buffer, BUFFER_SIZE, 0);
    
    // 断言：确保 recv() 成功
    assert(bytes_received > 0);
    assert(strcmp(buffer, "Hello, Server") == 0);
    
    printf("[TEST] recv(): PASSED\n");
}

int main() {
    int server_fd, client_socket;
    struct sockaddr_in server_addr, client_addr;
    socklen_t addr_len = sizeof(client_addr);

    // 创建 socket
    server_fd = socket(AF_INET, SOCK_STREAM, 0);
    assert(server_fd >= 0);

    // 绑定地址和端口
    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = INADDR_ANY;
    server_addr.sin_port = htons(PORT);
    assert(bind(server_fd, (struct sockaddr *)&server_addr, sizeof(server_addr)) == 0);
    
    // 监听
    assert(listen(server_fd, 1) == 0);
    printf("==== Server listening on port %d ====\n", PORT);

    // 等待客户端连接
    client_socket = accept(server_fd, (struct sockaddr *)&client_addr, &addr_len);
    assert(client_socket >= 0);
    printf("[INFO] Client connected.\n");

    // 测试 recv()
    test_recv(client_socket);

    // 关闭 socket
    close(client_socket);
    close(server_fd);
    return 0;
}
