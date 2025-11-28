// Copyright (c) 2025 Vivian Burkhard Voss
// SPDX-License-Identifier: Apache-2.0

/*
 * Test program to verify gbln_to_string and gbln_string_free work correctly.
 */

#include <stdio.h>
#include <stdlib.h>

// Forward declarations from GBLN FFI
typedef struct GblnValue GblnValue;

extern GblnValue* gbln_value_new_str(const char* value, size_t max_len);
extern char* gbln_to_string(const GblnValue* value);
extern void gbln_string_free(char* s);
extern void gbln_value_free(GblnValue* value);

int main(void) {
    printf("Test: Create string and serialize\n");
    
    // Create string value
    printf("  Creating string value...\n");
    GblnValue* str_val = gbln_value_new_str("Alice", 64);
    if (!str_val) {
        fprintf(stderr, "Failed to create string value\n");
        return 1;
    }
    printf("  Created: %p\n", (void*)str_val);
    
    // Serialize to string
    printf("  Serializing...\n");
    char* gbln_str = gbln_to_string(str_val);
    if (!gbln_str) {
        fprintf(stderr, "Failed to serialize\n");
        gbln_value_free(str_val);
        return 1;
    }
    printf("  Result: %s\n", gbln_str);
    
    // Free the serialized string
    printf("  Freeing serialized string...\n");
    gbln_string_free(gbln_str);
    printf("  Freed serialized string\n");
    
    // Free the value
    printf("  Freeing value...\n");
    gbln_value_free(str_val);
    printf("  Freed value\n");
    
    printf("✓ Success!\n");
    return 0;
}
