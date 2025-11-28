// Copyright (c) 2025 Vivian Burkhard Voss
// SPDX-License-Identifier: Apache-2.0

#include "../include/gbln.h"
#include <stdio.h>

int main() {
    // According to spec examples
    const char* input = "tags<s16>[rust python golang]";
    struct GblnValue* value = NULL;
    
    enum GblnErrorCode err = gbln_parse(input, &value);
    printf("Parse '%s'\n", input);
    printf("  Result: %d\n", err);
    
    if (err != Ok) {
        char* msg = gbln_last_error_message();
        if (msg) {
            printf("  Error: %s\n", msg);
            gbln_string_free(msg);
        }
    } else {
        printf("  Success!\n");
        size_t len = gbln_array_len(value);
        printf("  Array length: %zu\n", len);
        gbln_value_free(value);
    }
    
    return 0;
}
