#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <errno.h>
#include <sys/time.h>

#define PORT 8080
#define MAX_MSG 5      // 服务器期望收到 5 条消息
#define BATCH_SIZE 3   // 每次 recvmmsg 最多接收 3 条
#define MSG_SIZE 256

int main() {
    int sockfd;
    struct sockaddr_in server_addr, client_addr;
    socklen_t client_len = sizeof(client_addr);

    struct mmsghdr msgvec[BATCH_SIZE];
    struct iovec iovecs[BATCH_SIZE];
    char buffers[BATCH_SIZE][MSG_SIZE];

    // 创建 UDP socket
    if ((sockfd = socket(AF_INET, SOCK_DGRAM, 0)) == -1) {
        perror("socket failed");
        exit(EXIT_FAILURE);
    }

    // 绑定端口
    memset(&server_addr, 0, sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = INADDR_ANY;
    server_addr.sin_port = htons(PORT);

    if (bind(sockfd, (struct sockaddr*)&server_addr, sizeof(server_addr)) < 0) {
        perror("bind failed");
        close(sockfd);
        exit(EXIT_FAILURE);
    }

    printf("Server listening on port %d...\n", PORT);

    int total_received = 0;
    while (total_received < MAX_MSG) {
        // 初始化 msgvec
        memset(msgvec, 0, sizeof(msgvec));
        for (int i = 0; i < BATCH_SIZE; i++) {
            iovecs[i].iov_base = buffers[i];
            iovecs[i].iov_len = MSG_SIZE;
            msgvec[i].msg_hdr.msg_iov = &iovecs[i];
            msgvec[i].msg_hdr.msg_iovlen = 1;
            msgvec[i].msg_hdr.msg_name = &client_addr;
            msgvec[i].msg_hdr.msg_namelen = client_len;
        }

        printf("Waiting for messages...\n");

        int received = recvmmsg(sockfd, msgvec, BATCH_SIZE, 0, NULL);
        if (received < 0) {
            perror("recvmmsg failed");
            continue;
        }

        total_received += received;
        printf("Received %d messages (Total: %d/%d):\n", received, total_received, MAX_MSG);
        for (int i = 0; i < received; i++) {
            printf("Message %d: %s\n", i, buffers[i]);
        }
    }

    printf("All messages received. Exiting...\n");
    close(sockfd);
    return 0;
}
