#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>

#define PORT 8082
#define BUFFER_SIZE 1024

int main() {
    int server_sock, client_sock;
    struct sockaddr_in server_addr, client_addr;
    socklen_t client_len = sizeof(client_addr);
    char buffer[BUFFER_SIZE];

    // 创建 TCP socket
    server_sock = socket(AF_INET, SOCK_STREAM, 0);
    if (server_sock < 0) {
        perror("Server: socket() failed");
        exit(1);
    }

    // 绑定 IP 和端口
    memset(&server_addr, 0, sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    server_addr.sin_port = htons(PORT);
    server_addr.sin_addr.s_addr = INADDR_ANY;

    if (bind(server_sock, (struct sockaddr*)&server_addr, sizeof(server_addr)) < 0) {
        perror("Server: bind() failed");
        close(server_sock);
        exit(1);
    }

    // 监听连接请求
    if (listen(server_sock, 5) < 0) {
        perror("Server: listen() failed");
        close(server_sock);
        exit(1);
    }

    printf("Server listening on port %d...\n", PORT);

    // 等待客户端连接
    client_sock = accept(server_sock, (struct sockaddr*)&client_addr, &client_len);
    if (client_sock < 0) {
        perror("Server: accept() failed");
        close(server_sock);
        exit(1);
    }

    printf("Client connected: %s\n", inet_ntoa(client_addr.sin_addr));

    // 读取客户端数据
    while (1) {
        memset(buffer, 0, BUFFER_SIZE);
        ssize_t bytes_received = recv(client_sock, buffer, BUFFER_SIZE, 0);
        if (bytes_received <= 0) {
            printf("Client disconnected.\n");
            break;
        }

        printf("Received: %s", buffer);

        // 如果客户端输入 "exit"，则关闭连接
        if (strncmp(buffer, "exit", 4) == 0) {
            printf("Closing connection...\n");
            break;
        }

        send(client_sock, buffer, bytes_received, 0);
    }

    close(client_sock);
    close(server_sock);
    return 0;
}
