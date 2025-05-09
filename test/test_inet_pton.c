#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <arpa/inet.h>
#include <assert.h>

void test_ipv4_valid() {
    struct in_addr addr;
    int ret = inet_pton(AF_INET, "192.168.1.1", &addr);
    assert(ret == 1);
    assert(addr.s_addr == htonl(0xC0A80101));
    printf("✅ IPv4 valid test passed!\n");
}

void test_ipv4_invalid() {
    struct in_addr addr;
    int ret = inet_pton(AF_INET, "999.999.999.999", &addr);
    assert(ret == 0);
    printf("✅ IPv4 invalid test passed!\n");
}

void test_ipv4_edge_cases() {
    struct in_addr addr;
    int ret = inet_pton(AF_INET, "0.0.0.0", &addr);
    assert(ret == 1);
    assert(addr.s_addr == htonl(0x00000000));
    printf("✅ IPv4 edge case test (0.0.0.0) passed!\n");

    ret = inet_pton(AF_INET, "255.255.255.255", &addr);
    assert(ret == 1);
    assert(addr.s_addr == htonl(0xFFFFFFFF));
    printf("✅ IPv4 edge case test (255.255.255.255) passed!\n");
}

void test_ipv6_invalid() {
    struct in6_addr addr;
    int ret = inet_pton(AF_INET6, "2001::zzz", &addr);
    assert(ret == 0);
    printf("✅ IPv6 invalid test passed!\n");
}

void test_ipv6_random() {
    struct in6_addr addr6 = {0};
    uint8_t expected_addr[] = {0x20, 0x01, 0x0d, 0xb8, 0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01};

    int ret = inet_pton(AF_INET6, "2001:db8::1", &addr6);
    printf("inet_pton returned: %d\n", ret);
    if (ret != 1) {
        perror("inet_pton failed");
        exit(1);
    }

    printf("addr6: ");
    for (int i = 0; i < 16; i++) {
        printf("%02x ", addr6.s6_addr[i]);
    }
    printf("\nexpected: ");
    for (int i = 0; i < 16; i++) {
        printf("%02x ", expected_addr[i]);
    }
    printf("\n");

    assert(memcmp(addr6.s6_addr, expected_addr, 16) == 0);
    printf("✅ IPv6 valid test passed!\n");

    ret = inet_pton(AF_INET6, "::1", &addr6);
    assert(ret == 1);
    assert(addr6.s6_addr[15] == 1);
    printf("✅ IPv6 loopback test passed!\n");
}

void test_ipv6_edge_cases() {
    struct in6_addr addr;
    int ret = inet_pton(AF_INET6, "::", &addr);
    assert(ret == 1);
    assert(memcmp(&addr, &in6addr_any, sizeof(addr)) == 0);
    printf("✅ IPv6 edge case test (::) passed!\n");

    ret = inet_pton(AF_INET6, "::1", &addr);
    assert(ret == 1);
    assert(memcmp(&addr, &in6addr_loopback, sizeof(addr)) == 0);
    printf("✅ IPv6 edge case test (::1) passed!\n");
}

void test_invalid_family() {
    struct in_addr addr;
    int ret = inet_pton(999, "192.168.1.1", &addr);
    assert(ret == -1);
    printf("✅ Invalid address family test passed!\n");
}

int main() {
    test_ipv4_valid();
    test_ipv4_invalid();
    test_ipv4_edge_cases();
    test_ipv6_invalid();
	test_ipv6_random();
    test_ipv6_edge_cases();
    test_invalid_family();
    printf("🎉 All tests passed successfully!\n");
    return 0;
}

