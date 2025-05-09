#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <arpa/inet.h>
#include <assert.h>

void test_inet_aton_valid() {
    struct in_addr addr;
    int ret = inet_aton("192.168.1.1", &addr);
    assert(ret == 1);
    assert(addr.s_addr == htonl(0xC0A80101));
    printf("✅ inet_aton valid test passed!\n");
}

void test_inet_aton_invalid() {
    struct in_addr addr;
    int ret = inet_aton("999.999.999.999", &addr);
    assert(ret == 0);
    printf("✅ inet_aton invalid test passed!\n");
}

void test_inet_aton_edge_cases() {
    struct in_addr addr;
    
    int ret = inet_aton("0.0.0.0", &addr);
    assert(ret == 1);
    assert(addr.s_addr == htonl(0x00000000));
    printf("✅ inet_aton edge case test (0.0.0.0) passed!\n");

    ret = inet_aton("255.255.255.255", &addr);
    assert(ret == 1);
    assert(addr.s_addr == htonl(0xFFFFFFFF));
    printf("✅ inet_aton edge case test (255.255.255.255) passed!\n");
}

void test_inet_ntoa_valid() {
    struct in_addr addr;
    addr.s_addr = htonl(0xC0A80101); // 192.168.1.1
    const char *ret = inet_ntoa(addr);
    assert(ret != NULL);
    assert(strcmp(ret, "192.168.1.1") == 0);
    printf("✅ inet_ntoa valid test passed!\n");
}

void test_inet_ntoa_edge_cases() {
    struct in_addr addr;
    
    addr.s_addr = htonl(0x00000000); // 0.0.0.0
    assert(strcmp(inet_ntoa(addr), "0.0.0.0") == 0);
    printf("✅ inet_ntoa edge case test (0.0.0.0) passed!\n");

    addr.s_addr = htonl(0xFFFFFFFF); // 255.255.255.255
    assert(strcmp(inet_ntoa(addr), "255.255.255.255") == 0);
    printf("✅ inet_ntoa edge case test (255.255.255.255) passed!\n");
}

int main() {
    test_inet_aton_valid();
    test_inet_aton_invalid();
    test_inet_aton_edge_cases();
    test_inet_ntoa_valid();
    test_inet_ntoa_edge_cases();
    printf("🎉 All tests passed successfully!\n");
    return 0;
}

