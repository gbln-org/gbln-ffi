#include "../include/gbln.h"
#include <stdio.h>

int main() {
    struct GblnValue* value = NULL;
    enum GblnErrorCode err;
    
    // Test 1: Single value without type hint - should work per spec
    printf("Test 1: name(Alice)\n");
    err = gbln_parse("name(Alice)", &value);
    printf("  Result: %d\n", err);
    if (err != Ok) {
        char* msg = gbln_last_error_message();
        if (msg) { printf("  Error: %s\n", msg); gbln_string_free(msg); }
    } else {
        gbln_value_free(value);
    }
    
    // Test 2: Object without type hints
    printf("\nTest 2: {name(Alice)age(25)}\n");
    err = gbln_parse("{name(Alice)age(25)}", &value);
    printf("  Result: %d\n", err);
    if (err != Ok) {
        char* msg = gbln_last_error_message();
        if (msg) { printf("  Error: %s\n", msg); gbln_string_free(msg); }
    } else {
        gbln_value_free(value);
    }
    
    // Test 3: Array without type hints
    printf("\nTest 3: [1 2 3]\n");
    err = gbln_parse("[1 2 3]", &value);
    printf("  Result: %d\n", err);
    if (err != Ok) {
        char* msg = gbln_last_error_message();
        if (msg) { printf("  Error: %s\n", msg); gbln_string_free(msg); }
    } else {
        gbln_value_free(value);
    }
    
    return 0;
}
