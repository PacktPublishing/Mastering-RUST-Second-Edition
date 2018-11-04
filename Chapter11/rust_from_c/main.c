#include <stdint.h>
#include <stdio.h>

int32_t compare_str(const char* value, const char* substr);

int main() {
    printf("%d\n", compare_str("amanda", "brian"));
    return 0;
}
