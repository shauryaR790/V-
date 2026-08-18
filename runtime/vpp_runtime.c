#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#if !defined(_WIN32)
#include <sys/wait.h>
#endif

// v++ v0.2 runtime — ARC strings and arrays. ABI documented in MEMORY_MODEL.md.

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

static char* vpp_strdup(const char* text) {
#if defined(_MSC_VER)
    return _strdup(text);
#else
    return strdup(text);
#endif
}

void vpp_print_int(int64_t value) {
    printf("%lld\n", (long long)value);
    fflush(stdout);
}

void vpp_print_float(double value) {
    printf("%g\n", value);
    fflush(stdout);
}

void vpp_print_bool(int value) {
    printf("%s\n", value ? "true" : "false");
    fflush(stdout);
}

void vpp_print_str(VppString* value) {
    if (value && value->data) {
        printf("%s\n", value->data);
    } else {
        printf("\n");
    }
    fflush(stdout);
}

void* vpp_alloc(size_t size) {
    return malloc(size);
}

VppString* vpp_string_new(const char* text) {
    VppString* s = (VppString*)malloc(sizeof(VppString));
    if (!s) return NULL;
    s->ref_count = 1;
    s->data = text ? vpp_strdup(text) : vpp_strdup("");
    return s;
}

VppString* vpp_string_concat(VppString* a, VppString* b) {
    const char* sa = (a && a->data) ? a->data : "";
    const char* sb = (b && b->data) ? b->data : "";
    size_t la = strlen(sa);
    size_t lb = strlen(sb);
    char* buf = (char*)malloc(la + lb + 1);
    if (!buf) return NULL;
    memcpy(buf, sa, la);
    memcpy(buf + la, sb, lb);
    buf[la + lb] = '\0';
    VppString* out = vpp_string_new(buf);
    free(buf);
    return out;
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

void* vpp_array_index_ptr(VppArray* arr, int64_t idx) {
    if (!arr || idx < 0 || idx >= arr->len) {
        fprintf(stderr, "index out of bounds\n");
        exit(1);
    }
    return (char*)arr->data + (size_t)(idx * arr->elem_size);
}

int64_t vpp_array_len(VppArray* arr) {
    return arr ? arr->len : 0;
}

void* vpp_array_data(VppArray* arr) {
    return arr ? arr->data : NULL;
}

VppArray* vpp_make_array(int64_t len, int64_t elem_size) {
    VppArray* arr = (VppArray*)malloc(sizeof(VppArray));
    if (!arr) return NULL;
    arr->ref_count = 1;
    arr->len = len;
    arr->elem_size = elem_size;
    arr->data = calloc((size_t)len, (size_t)elem_size);
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

void vpp_assert_fail(const char* message) {
    fprintf(stderr, "assertion failed: %s\n", message ? message : "condition is false");
    exit(1);
}

void vpp_assert_eq_fail(const char* message) {
    fprintf(stderr, "assertion failed: %s\n", message ? message : "values not equal");
    exit(1);
}

static const char* vpp_string_cstr(VppString* s) {
    return (s && s->data) ? s->data : "";
}

VppString* vpp_read_file(VppString* path) {
    const char* p = vpp_string_cstr(path);
    FILE* f = fopen(p, "rb");
    if (!f) {
        fprintf(stderr, "read_file failed: cannot open `%s`\n", p);
        exit(1);
    }
    if (fseek(f, 0, SEEK_END) != 0) {
        fclose(f);
        fprintf(stderr, "read_file failed: `%s`\n", p);
        exit(1);
    }
    long size = ftell(f);
    if (size < 0) {
        fclose(f);
        fprintf(stderr, "read_file failed: `%s`\n", p);
        exit(1);
    }
    rewind(f);
    char* buf = (char*)malloc((size_t)size + 1);
    if (!buf) {
        fclose(f);
        exit(1);
    }
    size_t read = fread(buf, 1, (size_t)size, f);
    fclose(f);
    buf[read] = '\0';
    VppString* out = vpp_string_new(buf);
    free(buf);
    return out;
}

void vpp_write_file(VppString* path, VppString* contents) {
    const char* p = vpp_string_cstr(path);
    const char* data = vpp_string_cstr(contents);
    FILE* f = fopen(p, "wb");
    if (!f) {
        fprintf(stderr, "write_file failed: cannot open `%s`\n", p);
        exit(1);
    }
    size_t len = strlen(data);
    if (fwrite(data, 1, len, f) != len) {
        fclose(f);
        fprintf(stderr, "write_file failed: `%s`\n", p);
        exit(1);
    }
    fclose(f);
}

int32_t vpp_file_exists(VppString* path) {
    const char* p = vpp_string_cstr(path);
    FILE* f = fopen(p, "rb");
    if (f) {
        fclose(f);
        return 1;
    }
    return 0;
}

static void vpp_skip_ws(const char** p) {
    while (**p == ' ' || **p == '\t' || **p == '\n' || **p == '\r') {
        (*p)++;
    }
}

static int vpp_json_validate_value(const char** p);

static int vpp_json_validate_string(const char** p) {
    if (**p != '"') return 0;
    (*p)++;
    while (**p) {
        if (**p == '"') {
            (*p)++;
            return 1;
        }
        if (**p == '\\') {
            (*p)++;
            if (**p == '\0') return 0;
        }
        (*p)++;
    }
    return 0;
}

static int vpp_json_validate_number(const char** p) {
    const char* start = *p;
    if (**p == '-') (*p)++;
    if (**p < '0' || **p > '9') return 0;
    while (**p >= '0' && **p <= '9') (*p)++;
    if (**p == '.') {
        (*p)++;
        while (**p >= '0' && **p <= '9') (*p)++;
    }
    return *p > start;
}

static int vpp_json_validate_literal(const char** p, const char* lit) {
    size_t n = strlen(lit);
    if (strncmp(*p, lit, n) != 0) return 0;
    *p += n;
    return 1;
}

static int vpp_json_validate_array(const char** p) {
    if (**p != '[') return 0;
    (*p)++;
    vpp_skip_ws(p);
    if (**p == ']') {
        (*p)++;
        return 1;
    }
    while (1) {
        if (!vpp_json_validate_value(p)) return 0;
        vpp_skip_ws(p);
        if (**p == ']') {
            (*p)++;
            return 1;
        }
        if (**p != ',') return 0;
        (*p)++;
        vpp_skip_ws(p);
    }
}

static int vpp_json_validate_object(const char** p) {
    if (**p != '{') return 0;
    (*p)++;
    vpp_skip_ws(p);
    if (**p == '}') {
        (*p)++;
        return 1;
    }
    while (1) {
        if (!vpp_json_validate_string(p)) return 0;
        vpp_skip_ws(p);
        if (**p != ':') return 0;
        (*p)++;
        vpp_skip_ws(p);
        if (!vpp_json_validate_value(p)) return 0;
        vpp_skip_ws(p);
        if (**p == '}') {
            (*p)++;
            return 1;
        }
        if (**p != ',') return 0;
        (*p)++;
        vpp_skip_ws(p);
    }
}

static int vpp_json_validate_value(const char** p) {
    vpp_skip_ws(p);
    if (**p == '"') return vpp_json_validate_string(p);
    if (**p == '{') return vpp_json_validate_object(p);
    if (**p == '[') return vpp_json_validate_array(p);
    if (**p == '-' || (**p >= '0' && **p <= '9')) return vpp_json_validate_number(p);
    if (vpp_json_validate_literal(p, "true")) return 1;
    if (vpp_json_validate_literal(p, "false")) return 1;
    if (vpp_json_validate_literal(p, "null")) return 1;
    return 0;
}

static int vpp_json_is_valid(const char* text) {
    const char* p = text;
    vpp_skip_ws(&p);
    if (!vpp_json_validate_value(&p)) return 0;
    vpp_skip_ws(&p);
    return *p == '\0';
}

VppString* vpp_json_parse(VppString* raw) {
    const char* text = vpp_string_cstr(raw);
    if (!vpp_json_is_valid(text)) {
        fprintf(stderr, "json_parse failed: invalid JSON\n");
        exit(1);
    }
    return vpp_string_new(text);
}

VppString* vpp_json_stringify(VppString* raw) {
    const char* text = vpp_string_cstr(raw);
    if (text[0] == '{' || text[0] == '[') {
        if (!vpp_json_is_valid(text)) {
            fprintf(stderr, "json_stringify failed: invalid JSON\n");
            exit(1);
        }
        return vpp_string_new(text);
    }
    size_t len = strlen(text);
    char* buf = (char*)malloc(len * 2 + 3);
    if (!buf) exit(1);
    buf[0] = '"';
    size_t j = 1;
    for (size_t i = 0; i < len; i++) {
        char c = text[i];
        if (c == '"' || c == '\\') {
            buf[j++] = '\\';
        }
        buf[j++] = c;
    }
    buf[j++] = '"';
    buf[j] = '\0';
    VppString* out = vpp_string_new(buf);
    free(buf);
    return out;
}

int64_t vpp_process_run(VppString* cmd) {
    const char* command = vpp_string_cstr(cmd);
#if defined(_WIN32)
    char buf[4096];
    snprintf(buf, sizeof(buf), "cmd /C %s", command);
    int code = system(buf);
#else
    int code = system(command);
#endif
    if (code == -1) {
        fprintf(stderr, "process_run failed\n");
        exit(1);
    }
#if defined(_WIN32)
    return (int64_t)code;
#else
    if (WIFEXITED(code)) {
        return (int64_t)WEXITSTATUS(code);
    }
    return 1;
#endif
}
