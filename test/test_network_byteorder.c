#include <stdio.h>
#include <stdint.h>
#include <arpa/inet.h>
#include <assert.h>

void to_hex_string(uint32_t val, char* hex_str, int is_short) {
    const char hex_digits[] = "0123456789abcdef";
    int i = 0;
    int size = is_short ? 4 : 8;  // 16位用4个字符，32位用8个字符

    // 根据数据类型大小转换为十六进制
    for (i = size - 1; i >= 0; --i) {
        hex_str[i] = hex_digits[val & 0xF];  // 获取最低4位的十六进制数字
        val >>= 4;  // 右移4位，处理下一个4位
    }
    hex_str[size] = '\0';  // 结束符
}

void print_hex(uint32_t val, const char* label, int is_short) {
    char hex_str[9];  // 存储转换后的十六进制字符串

    // 将整数转换为十六进制字符串
    to_hex_string(val, hex_str, is_short);

    // 打印结果
    printf("%s: 0x%s\n", label, hex_str);
}

void test_htons_ntohs() {
    uint16_t host_short = 0x1234;
    uint16_t net_short;

    // Convert host short to network short
    net_short = htons(host_short);
    // Assert that the network byte order is correct (reverse byte order)
    assert(ntohs(net_short) == host_short);
    printf("✅ test_htons_ntohs passed\n");

    // Print conversion details
    print_hex(host_short, "Host short", 1);
    print_hex(net_short, "Network short", 1);
    print_hex(ntohs(net_short), "After ntohs", 1);

    // Test edge case with 0
    host_short = 0x0000;
    net_short = htons(host_short);
    assert(ntohs(net_short) == host_short);
    printf("✅ test_htons_ntohs (edge case) passed\n");

    // Print conversion details
    print_hex(host_short, "Host short (edge case)", 1);
    print_hex(net_short, "Network short (edge case)", 1);
    print_hex(ntohs(net_short), "After ntohs (edge case)", 1);

    // Test with maximum 16-bit value
    host_short = 0xFFFF;
    net_short = htons(host_short);
    assert(ntohs(net_short) == host_short);
    printf("✅ test_htons_ntohs (max value) passed\n");

    // Print conversion details
    print_hex(host_short, "Host short (max value)", 1);
    print_hex(net_short, "Network short (max value)", 1);
    print_hex(ntohs(net_short), "After ntohs (max value)", 1);
}

void test_htonl_ntohl() {
    uint32_t host_long = 0x12345678;
    uint32_t net_long;

    // Convert host long to network long
    net_long = htonl(host_long);
    // Assert that the network byte order is correct (reverse byte order)
    assert(ntohl(net_long) == host_long);
    printf("✅ test_htonl_ntohl passed\n");

    // Print conversion details
    print_hex(host_long, "Host long", 0);
    print_hex(net_long, "Network long", 0);
    print_hex(ntohl(net_long), "After ntohl", 0);

    // Test edge case with 0
    host_long = 0x00000000;
    net_long = htonl(host_long);
    assert(ntohl(net_long) == host_long);
    printf("✅ test_htonl_ntohl (edge case) passed\n");

    // Print conversion details
    print_hex(host_long, "Host long (edge case)", 0);
    print_hex(net_long, "Network long (edge case)", 0);
    print_hex(ntohl(net_long), "After ntohl (edge case)", 0);

    // Test with maximum 32-bit value
    host_long = 0xFFFFFFFF;
    net_long = htonl(host_long);
    assert(ntohl(net_long) == host_long);
    printf("✅ test_htonl_ntohl (max value) passed\n");

    // Print conversion details
    print_hex(host_long, "Host long (max value)", 0);
    print_hex(net_long, "Network long (max value)", 0);
    print_hex(ntohl(net_long), "After ntohl (max value)", 0);
}

int main() {
    // Run tests for 16-bit short conversion functions
    test_htons_ntohs();

    // Run tests for 32-bit long conversion functions
    test_htonl_ntohl();

    return 0;
}