// Copyright (c) 2025 Vivian Burkhard Voss
// SPDX-License-Identifier: Apache-2.0

#include "../include/gbln.h"
#include <stdio.h>

int main() {
    const char* input = "{age<i8>(999)}";
    struct GblnValue* value = NULL;
    
    enum GblnErrorCode err = gbln_parse(input, &value);
    printf("Error code: %d (Expected: %d)\n", err, ErrorIntOutOfRange);
    
    if (err != Ok) {
        char* msg = gbln_last_error_message();
        if (msg) {
            printf("Message: %s\n", msg);
            gbln_string_free(msg);
        }
        
        char* sugg = gbln_last_error_suggestion();
        if (sugg) {
            printf("Suggestion: %s\n", sugg);
            gbln_string_free(sugg);
        }
    }
    
    return 0;
}
