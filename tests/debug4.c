#include "../include/gbln.h"
#include <stdio.h>

int main() {
    const char* tests[] = {
        "[rust python golang]",
        "<s8>[rust python golang]",
        "tags[rust python golang]",
        "{tags[rust python golang]}"
    };
    
    for (int i = 0; i < 4; i++) {
        printf("\nTest %d: %s\n", i+1, tests[i]);
        struct GblnValue* value = NULL;
        enum GblnErrorCode err = gbln_parse(tests[i], &value);
        printf("  Parse result: %d\n", err);
        
        if (err != Ok) {
            char* msg = gbln_last_error_message();
            if (msg) {
                printf("  Error: %s\n", msg);
                gbln_string_free(msg);
            }
        } else {
            size_t len = gbln_array_len(value);
            printf("  Array length: %zu\n", len);
            gbln_value_free(value);
        }
    }
    
    return 0;
}
