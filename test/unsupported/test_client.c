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
    int client_sock;
    struct sockaddr_in server_addr;
    char buffer[BUFFER_SIZE];

    // 创建 TCP socket
    client_sock = socket(AF_INET, SOCK_STREAM, 0);
    if (client_sock < 0) {
        perror("Client: socket() failed");
        exit(1);
    }

    // 设置服务器地址
    memset(&server_addr, 0, sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    server_addr.sin_port = htons(PORT);
    server_addr.sin_addr.s_addr = inet_addr("127.0.0.1");

    // 连接服务器
    if (connect(client_sock, (struct sockaddr*)&server_addr, sizeof(server_addr)) < 0) {
        perror("Client: connect() failed");
        close(client_sock);
        exit(1);
    }

    printf("Connected to server. Type messages (type 'exit' to quit):\n");

    // 读取用户输入，发送数据
    while (1) {
        memset(buffer, 0, BUFFER_SIZE);
        printf("> ");
        fgets(buffer, BUFFER_SIZE, stdin);

        send(client_sock, buffer, strlen(buffer), 0);

        if (strncmp(buffer, "exit", 4) == 0) {
            printf("Closing connection...\n");
            break;
        }

        memset(buffer, 0, BUFFER_SIZE);
        recv(client_sock, buffer, BUFFER_SIZE, 0);
        printf("Server echoed: %s", buffer);
    }

    close(client_sock);
    return 0;
}
