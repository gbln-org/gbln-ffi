#include "../include/gbln.h"
#include <stdio.h>

int main() {
    const char* input = "{tags<s16>[rust python golang]}";
    struct GblnValue* value = NULL;
    
    enum GblnErrorCode err = gbln_parse(input, &value);
    printf("Parse: %d\n", err);
    
    if (err != Ok) {
        char* msg = gbln_last_error_message();
        if (msg) {
            printf("Error: %s\n", msg);
            gbln_string_free(msg);
        }
    }
    
    return 0;
}
