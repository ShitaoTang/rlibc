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
#define BUFFER_SIZE 256

void receive_fd(int sock) {
    struct msghdr msg = {0};
    struct iovec io;
    char buf[BUFFER_SIZE];
    memset(buf, 0, sizeof(buf));
    
    // 控制消息缓冲区
    char cmsg_buf[CMSG_SPACE(sizeof(int))];
    memset(cmsg_buf, 0, sizeof(cmsg_buf));

    io.iov_base = buf;
    io.iov_len = sizeof(buf);
    msg.msg_iov = &io;
    msg.msg_iovlen = 1;
    msg.msg_control = cmsg_buf;
    msg.msg_controllen = sizeof(cmsg_buf);

    // 接收消息
    ssize_t n = recvmsg(sock, &msg, 0);
    if (n < 0) {
        perror("[ERROR] recvmsg failed");
        exit(1);
    }
    printf("[TEST] recvmsg(): Received %zd bytes: \"%s\"\n", n, buf);

    // 解析控制消息
    struct cmsghdr *cmsg = CMSG_FIRSTHDR(&msg);
    if (cmsg && cmsg->cmsg_level == SOL_SOCKET && cmsg->cmsg_type == SCM_RIGHTS) {
        int received_fd;
        memcpy(&received_fd, CMSG_DATA(cmsg), sizeof(received_fd));
        printf("[TEST] Received file descriptor: %d\n", received_fd);

        // 读取文件内容
        char file_content[BUFFER_SIZE];
        ssize_t read_size = read(received_fd, file_content, BUFFER_SIZE - 1);
        if (read_size > 0) {
            file_content[read_size] = '\0';
            printf("[TEST] Read from received FD: \"%s\"\n", file_content);
        } else {
            perror("[ERROR] Failed to read from received FD");
        }
        close(received_fd);
    } else {
        printf("[TEST] No control message received.\n");
    }
    printf("[TEST] recvmsg(): PASSED\n");
}

int main() {
    int server_fd, client_fd;
    struct sockaddr_un addr;

    server_fd = socket(AF_UNIX, SOCK_STREAM, 0);
    assert(server_fd >= 0);

    memset(&addr, 0, sizeof(addr));
    addr.sun_family = AF_UNIX;
    strncpy(addr.sun_path, SOCKET_PATH, sizeof(addr.sun_path) - 1);
    unlink(SOCKET_PATH);

    assert(bind(server_fd, (struct sockaddr *)&addr, sizeof(addr)) == 0);
    assert(listen(server_fd, 5) == 0);

    printf("==== Server listening on %s ====\n", SOCKET_PATH);
    client_fd = accept(server_fd, NULL, NULL);
    assert(client_fd >= 0);
    printf("[INFO] Client connected.\n");

    receive_fd(client_fd);

    close(client_fd);
    close(server_fd);
    unlink(SOCKET_PATH);
    return 0;
}
