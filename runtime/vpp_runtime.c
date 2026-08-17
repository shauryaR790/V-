#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>

// Hybrid memory model: ARC reference counting for heap strings and arrays.

typedef struct {
    char* data;
    int64_t ref_count;
} VppString;

typedef struct {
    void* data;
    int64_t len;
    int64_t elem_size;
    int64_t ref_count;
} VppArray;

void vpp_print_int(int64_t value) {
    printf("%lld\n", (long long)value);
}

void vpp_print_float(double value) {
    printf("%g\n", value);
}

void vpp_print_bool(int value) {
    printf("%s\n", value ? "true" : "false");
}

void vpp_print_str(VppString* value) {
    if (value && value->data) {
        printf("%s\n", value->data);
    }
}

void* vpp_alloc(size_t size) {
    return malloc(size);
}

VppString* vpp_string_new(const char* text) {
    VppString* s = (VppString*)malloc(sizeof(VppString));
    s->ref_count = 1;
    s->data = _strdup(text);
    return s;
}

VppString* vpp_string_retain(VppString* s) {
    if (s) {
        s->ref_count += 1;
    }
    return s;
}

void vpp_string_release(VppString* s) {
    if (!s) return;
    s->ref_count -= 1;
    if (s->ref_count <= 0) {
        free(s->data);
        free(s);
    }
}

int64_t vpp_array_len(VppArray* arr) {
    return arr ? arr->len : 0;
}

void* vpp_array_data(VppArray* arr) {
    return arr ? arr->data : NULL;
}

VppArray* vpp_make_array(int64_t len, int64_t elem_size) {
    VppArray* arr = (VppArray*)malloc(sizeof(VppArray));
    arr->ref_count = 1;
    arr->len = len;
    arr->elem_size = elem_size;
    arr->data = malloc((size_t)(len * elem_size));
    return arr;
}

VppArray* vpp_array_retain(VppArray* arr) {
    if (arr) {
        arr->ref_count += 1;
    }
    return arr;
}

void vpp_array_release(VppArray* arr) {
    if (!arr) return;
    arr->ref_count -= 1;
    if (arr->ref_count <= 0) {
        free(arr->data);
        free(arr);
    }
}

int64_t vpp_strlen(VppString* s) {
    return s && s->data ? (int64_t)strlen(s->data) : 0;
}
