// Copyright (c) 2025 Vivian Burkhard Voss
// SPDX-License-Identifier: Apache-2.0

#include "../include/gbln.h"
#include <stdio.h>

int main() {
    const char* input = "user{id<u32>(12345)name<s32>(Alice)}";
    struct GblnValue* value = NULL;
    
    enum GblnErrorCode err = gbln_parse(input, &value);
    printf("Parse result: %d\n", err);
    
    if (err != Ok) {
        char* msg = gbln_last_error_message();
        if (msg) {
            printf("Error: %s\n", msg);
            gbln_string_free(msg);
        }
        return 1;
    }
    
    const struct GblnValue* id = gbln_object_get(value, "id");
    printf("Object get result: %p\n", (void*)id);
    
    if (id) {
        bool ok = false;
        uint32_t val = gbln_value_as_u32(id, &ok);
        printf("Value as u32: %u, ok: %d\n", val, ok);
    }
    
    gbln_value_free(value);
    return 0;
}
