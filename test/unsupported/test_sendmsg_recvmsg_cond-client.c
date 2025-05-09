#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <sys/un.h>
#include <fcntl.h>
#include <errno.h>
#include <assert.h>

#define SOCKET_PATH "/tmp/test_socket"
#define TEST_FILE "/tmp/test_file.txt"

void send_fd(int sock, int fd_to_send) {
    struct msghdr msg = {0};
    struct iovec io;
    char buf[] = "Hello, server! Here's a file descriptor.";
    
    // 控制消息缓冲区
    char cmsg_buf[CMSG_SPACE(sizeof(int))];
    memset(cmsg_buf, 0, sizeof(cmsg_buf));

    io.iov_base = buf;
    io.iov_len = strlen(buf) + 1;
    msg.msg_iov = &io;
    msg.msg_iovlen = 1;
    msg.msg_control = cmsg_buf;
    msg.msg_controllen = sizeof(cmsg_buf);

    // 设置控制消息，发送文件描述符
    struct cmsghdr *cmsg = CMSG_FIRSTHDR(&msg);
    cmsg->cmsg_level = SOL_SOCKET;
    cmsg->cmsg_type = SCM_RIGHTS;
    cmsg->cmsg_len = CMSG_LEN(sizeof(fd_to_send));
    memcpy(CMSG_DATA(cmsg), &fd_to_send, sizeof(fd_to_send));

    // 发送消息
    ssize_t n = sendmsg(sock, &msg, 0);
    if (n < 0) {
        perror("[ERROR] sendmsg failed");
        exit(1);
    }
    printf("[TEST] sendmsg(): Sent %zd bytes\n", n);
}

int main() {
    int client_fd;
    struct sockaddr_un addr;

    client_fd = socket(AF_UNIX, SOCK_STREAM, 0);
    assert(client_fd >= 0);

    memset(&addr, 0, sizeof(addr));
    addr.sun_family = AF_UNIX;
    strncpy(addr.sun_path, SOCKET_PATH, sizeof(addr.sun_path) - 1);

    assert(connect(client_fd, (struct sockaddr *)&addr, sizeof(addr)) == 0);
    printf("[INFO] Connected to server.\n");

    // 创建测试文件
    int test_fd = open(TEST_FILE, O_CREAT | O_RDWR, 0666);
    assert(test_fd >= 0);
    write(test_fd, "File content example.", 22);
    lseek(test_fd, 0, SEEK_SET); // 复位文件指针

    send_fd(client_fd, test_fd);

    close(test_fd);
    close(client_fd);
    return 0;
}
