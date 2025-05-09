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

#define PORT 12345
#define BUFFER_SIZE 1024

void test_recvmsg(int client_socket) {
    struct msghdr msg;
    struct iovec iov;
    char buffer[BUFFER_SIZE] = {0};
    char control[CMSG_SPACE(sizeof(int))];

    // 设置 iovec
    iov.iov_base = buffer;
    iov.iov_len = sizeof(buffer);

    // 初始化 msghdr
    memset(&msg, 0, sizeof(msg));
    msg.msg_iov = &iov;
    msg.msg_iovlen = 1;
    msg.msg_control = control;
    msg.msg_controllen = sizeof(control);

    // 调用 recvmsg()
    ssize_t bytes_received = recvmsg(client_socket, &msg, 0);
    assert(bytes_received > 0);
    printf("[TEST] recvmsg(): Received %zd bytes\n", bytes_received);

    // 解析控制消息
    struct cmsghdr *cmsg = CMSG_FIRSTHDR(&msg);
    if (cmsg && cmsg->cmsg_level == SOL_SOCKET && cmsg->cmsg_type == SCM_RIGHTS) {
        int received_fd;
        memcpy(&received_fd, CMSG_DATA(cmsg), sizeof(received_fd));
        printf("[TEST] Received file descriptor: %d\n", received_fd);
    } else {
        printf("[TEST] No control message received.\n");
    }

    // 确保数据内容正确
    assert(strcmp(buffer, "Hello, Server (via sendmsg)") == 0);
    printf("[TEST] recvmsg(): PASSED\n");
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

    // 测试 recvmsg()
    test_recvmsg(client_socket);

    // 关闭 socket
    close(client_socket);
    close(server_fd);
    return 0;
}
