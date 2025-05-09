#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>
#include <assert.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <sys/uio.h>
#include <errno.h>

#define SERVER_IP "127.0.0.1"
#define PORT 12345
#define BUFFER_SIZE 1024

void test_sendmsg(int sock) {
    struct msghdr msg;
    struct iovec iov;
    char message[] = "Hello, Server (via sendmsg)";
    char control[CMSG_SPACE(sizeof(int))];

    // 设置 iovec
    iov.iov_base = message;
    iov.iov_len = strlen(message) + 1;

    // 初始化 msghdr
    memset(&msg, 0, sizeof(msg));
    msg.msg_iov = &iov;
    msg.msg_iovlen = 1;
    msg.msg_control = control;
    msg.msg_controllen = sizeof(control);

    // 添加控制消息 (示例：传递文件描述符)
    struct cmsghdr *cmsg = CMSG_FIRSTHDR(&msg);
    cmsg->cmsg_level = SOL_SOCKET;
    cmsg->cmsg_type = SCM_RIGHTS;
    cmsg->cmsg_len = CMSG_LEN(sizeof(int));
    int dummy_fd = 1;  // 传递一个文件描述符 (stdout)
    memcpy(CMSG_DATA(cmsg), &dummy_fd, sizeof(dummy_fd));

    // 发送消息
    ssize_t bytes_sent = sendmsg(sock, &msg, 0);
    assert(bytes_sent > 0);
    printf("[TEST] sendmsg(): Sent %zd bytes\n", bytes_sent);
    printf("[TEST] sendmsg(): PASSED\n");
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

    // 测试 sendmsg()
    test_sendmsg(sock);

    // 关闭 socket
    close(sock);
    return 0;
}
