#include "../include/gbln.h"
#include <stdio.h>

int main() {
    const char* input = "tags<s8>[rust python golang]";
    struct GblnValue* value = NULL;
    
    enum GblnErrorCode err = gbln_parse(input, &value);
    printf("Parse result: %d\n", err);
    
    if (err == Ok) {
        size_t len = gbln_array_len(value);
        printf("Array length: %zu\n", len);
        
        // Try as single value first
        bool ok;
        char* str = gbln_value_as_string(value, &ok);
        printf("As string: ok=%d, val=%s\n", ok, str ? str : "NULL");
        if (str) gbln_string_free(str);
        
        gbln_value_free(value);
    }
    
    return 0;
}
