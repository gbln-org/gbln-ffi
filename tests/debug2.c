#include "../include/gbln.h"
#include <stdio.h>

int main() {
    // Test 1: Just key-value
    const char* input1 = "id<u32>(12345)";
    struct GblnValue* value1 = NULL;
    
    enum GblnErrorCode err1 = gbln_parse(input1, &value1);
    printf("Test 1 - Single value: %d\n", err1);
    if (err1 == Ok) {
        bool ok;
        uint32_t val = gbln_value_as_u32(value1, &ok);
        printf("  Direct u32: %u, ok: %d\n", val, ok);
        gbln_value_free(value1);
    }
    
    // Test 2: Object with fields
    const char* input2 = "{id<u32>(12345)name<s32>(Alice)}";
    struct GblnValue* value2 = NULL;
    
    enum GblnErrorCode err2 = gbln_parse(input2, &value2);
    printf("\nTest 2 - Anonymous object: %d\n", err2);
    if (err2 == Ok) {
        const struct GblnValue* id = gbln_object_get(value2, "id");
        printf("  Object get 'id': %p\n", (void*)id);
        if (id) {
            bool ok;
            uint32_t val = gbln_value_as_u32(id, &ok);
            printf("  Value: %u, ok: %d\n", val, ok);
        }
        gbln_value_free(value2);
    }
    
    return 0;
}
