#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>

void test_listen_basic() {
    int sockfd = socket(AF_INET, SOCK_STREAM, 0);
    struct sockaddr_in addr;
    addr.sin_family = AF_INET;
    addr.sin_port = htons(12348);
    addr.sin_addr.s_addr = INADDR_ANY;

    bind(sockfd, (struct sockaddr*)&addr, sizeof(addr));
    if (listen(sockfd, 5) == 0) {
        printf("✅ Basic listen succeeded (backlog: 5)\n");
    } else {
        printf("❌ Basic listen failed\n");
    }
    close(sockfd);
}

void test_listen_invalid_fd() {
    if (listen(-1, 5) < 0) {
        printf("✅ Invalid fd listen test passed (expected failure)\n");
    } else {
        printf("❌ Invalid fd listen test failed\n");
    }
}

void test_listen_unbound_socket() {
    int sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (listen(sockfd, 5) < 0) {
        printf("✅ Unbound socket listen test passed (expected failure)\n");
    } else {
        printf("❌ Unbound socket listen test failed\n");
    }
    close(sockfd);
}

void test_listen_large_backlog() {
    int sockfd = socket(AF_INET, SOCK_STREAM, 0);
    struct sockaddr_in addr;
    addr.sin_family = AF_INET;
    addr.sin_port = htons(12349);
    addr.sin_addr.s_addr = INADDR_ANY;

    bind(sockfd, (struct sockaddr*)&addr, sizeof(addr));
    if (listen(sockfd, 1000) == 0) {
        printf("✅ Large backlog listen succeeded (backlog: 1000)\n");
    } else {
        printf("❌ Large backlog listen failed\n");
    }
    close(sockfd);
}

int main() {
    printf("Running listen tests...\n");
    test_listen_basic();
    test_listen_invalid_fd();
    test_listen_unbound_socket();
    test_listen_large_backlog();
    printf("Listen tests completed.\n");
    return 0;
}
