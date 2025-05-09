#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <errno.h>

#define PORT 8080
#define MAX_MSG 5      // 客户端需要发送 5 条消息
#define BATCH_SIZE 3   // 每次 sendmmsg 最多发送 3 条
#define MSG_SIZE 256

int main() {
    int sockfd;
    struct sockaddr_in server_addr;
    struct mmsghdr msgvec[BATCH_SIZE];
    struct iovec iovecs[BATCH_SIZE];
    char *messages[MAX_MSG] = {
        "Hello", "Batch Send", "Last Packet", "Extra 1", "Extra 2"
    };

    // 创建 UDP socket
    if ((sockfd = socket(AF_INET, SOCK_DGRAM, 0)) == -1) {
        perror("socket failed");
        exit(EXIT_FAILURE);
    }

    // 服务器地址
    memset(&server_addr, 0, sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    server_addr.sin_port = htons(PORT);

    int total_sent = 0;
    while (total_sent < MAX_MSG) {
        int batch_size = (MAX_MSG - total_sent < BATCH_SIZE) ? (MAX_MSG - total_sent) : BATCH_SIZE;

        // 初始化 msgvec
        memset(msgvec, 0, sizeof(msgvec));
        for (int i = 0; i < batch_size; i++) {
            iovecs[i].iov_base = messages[total_sent + i];
            iovecs[i].iov_len = strlen(messages[total_sent + i]) + 1;
            msgvec[i].msg_hdr.msg_iov = &iovecs[i];
            msgvec[i].msg_hdr.msg_iovlen = 1;
            msgvec[i].msg_hdr.msg_name = &server_addr;
            msgvec[i].msg_hdr.msg_namelen = sizeof(server_addr);
        }

        int sent = sendmmsg(sockfd, msgvec, batch_size, 0);
        if (sent < 0) {
            perror("sendmmsg failed");
            break;
        }
        total_sent += sent;
        printf("Sent %d/%d messages\n", total_sent, MAX_MSG);
    }

    printf("All messages sent. Exiting...\n");
    close(sockfd);
    return 0;
}
