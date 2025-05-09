#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <arpa/inet.h>
#include <assert.h>

void test_ipv4_valid() {
    struct in_addr addr;
    addr.s_addr = htonl(0xC0A80101); // 192.168.1.1
    char buffer[INET_ADDRSTRLEN];
    const char *ret = inet_ntop(AF_INET, &addr, buffer, sizeof(buffer));
    assert(ret != NULL);
    assert(strcmp(buffer, "192.168.1.1") == 0);
    printf("✅ IPv4 valid test passed!\n");
}

void test_ipv4_edge_cases() {
    struct in_addr addr;
    char buffer[INET_ADDRSTRLEN];
    
    addr.s_addr = htonl(0x00000000); // 0.0.0.0
    assert(inet_ntop(AF_INET, &addr, buffer, sizeof(buffer)) != NULL);
    assert(strcmp(buffer, "0.0.0.0") == 0);
    printf("✅ IPv4 edge case test (0.0.0.0) passed!\n");

    addr.s_addr = htonl(0xFFFFFFFF); // 255.255.255.255
    assert(inet_ntop(AF_INET, &addr, buffer, sizeof(buffer)) != NULL);
    assert(strcmp(buffer, "255.255.255.255") == 0);
    printf("✅ IPv4 edge case test (255.255.255.255) passed!\n");
}

void test_ipv6_valid() {
    struct in6_addr addr;
    char buffer[INET6_ADDRSTRLEN] = {0};
    
    inet_pton(AF_INET6, "2001:db8::1", &addr);
    assert(inet_ntop(AF_INET6, &addr, buffer, sizeof(buffer)) != NULL);
	for (int i = 0; i <INET6_ADDRSTRLEN; ++i) {
		printf("%c", buffer[i]);
	}
	printf("\n");
    assert(strcmp(buffer, "2001:db8::1") == 0);
    printf("✅ IPv6 valid test passed!\n");
}

void test_ipv6_edge_cases() {
    struct in6_addr addr;
    char buffer[INET6_ADDRSTRLEN];
    
    inet_pton(AF_INET6, "::", &addr);
	const char *res = inet_ntop(AF_INET6, &addr, buffer, sizeof(buffer));
    assert(res != NULL);
    assert(strcmp(buffer, "::") == 0);
    printf("✅ IPv6 edge case test (::) passed!\n");

    inet_pton(AF_INET6, "::1", &addr);
    assert(inet_ntop(AF_INET6, &addr, buffer, sizeof(buffer)) != NULL);
    assert(strcmp(buffer, "::1") == 0);
    printf("✅ IPv6 edge case test (::1) passed!\n");
}

void test_invalid_family() {
    struct in_addr addr;
    char buffer[INET_ADDRSTRLEN];
    const char *ret = inet_ntop(999, &addr, buffer, sizeof(buffer));
    assert(ret == NULL);
    printf("✅ Invalid address family test passed!\n");
}

int main() {
    test_ipv4_valid();
    test_ipv4_edge_cases();
    test_ipv6_valid();
    test_ipv6_edge_cases();
    test_invalid_family();
    printf("🎉 All tests passed successfully!\n");
    return 0;
}
