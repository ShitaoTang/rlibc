#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>

void test_socket_basic() {
    int sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (sockfd >= 0) {
        printf("✅ Basic TCP socket creation succeeded (fd: %d)\n", sockfd);
        close(sockfd);
    } else {
        printf("❌ Basic TCP socket creation failed\n");
    }
}

void test_socket_invalid_domain() {
    int sockfd = socket(-1, SOCK_STREAM, 0); // 无效的 domain
    if (sockfd < 0) {
        printf("✅ Invalid domain test passed (expected failure)\n");
    } else {
        printf("❌ Invalid domain test failed (fd: %d)\n", sockfd);
        close(sockfd);
    }
}

void test_socket_invalid_type() {
    int sockfd = socket(AF_INET, -1, 0); // 无效的 type
    if (sockfd < 0) {
        printf("✅ Invalid type test passed (expected failure)\n");
    } else {
        printf("❌ Invalid type test failed (fd: %d)\n", sockfd);
        close(sockfd);
    }
}

void test_socket_multiple() {
    int sockfd1 = socket(AF_INET, SOCK_STREAM, 0);
    int sockfd2 = socket(AF_INET, SOCK_DGRAM, 0);
    if (sockfd1 >= 0 && sockfd2 >= 0) {
        printf("✅ Multiple socket creation succeeded (TCP fd: %d, UDP fd: %d)\n", sockfd1, sockfd2);
        close(sockfd1);
        close(sockfd2);
    } else {
        printf("❌ Multiple socket creation failed (TCP fd: %d, UDP fd: %d)\n", sockfd1, sockfd2);
    }
}

int main() {
    printf("Running socket tests...\n");
    test_socket_basic();
    test_socket_invalid_domain();
    test_socket_invalid_type();
    test_socket_multiple();
    printf("Socket tests completed.\n");
    return 0;
}
