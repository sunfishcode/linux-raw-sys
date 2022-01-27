//! A small C program which prints out ioctl codes to generate a header file
//! that bindgen can generate bindings from.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void list(void);

static FILE *generated;

void entry(const char *s) {
    if (fprintf(generated, "%s\n", s) != strlen(s) + 1) {
	fprintf(stderr, "can't write to generated.txt: %m\n");
	exit(1);
    }
}

int main(void) {
    generated = fopen("generated.txt", "w");
    if (generated == NULL) {
	fprintf(stderr, "can't open generated.txt: %m\n");
	exit(EXIT_FAILURE);
    }

#if defined(__i386__)
    printf("#ifdef __i386__\n");
#elif defined(__x86_64__)
    printf("#ifdef __x86_64__\n");
#elif defined(__arm__)
    printf("#ifdef __arm__\n");
#elif defined(__aarch64__)
    printf("#ifdef __aarch64__\n");
#elif defined(__powerpc__)
    printf("#ifdef __powerpc__\n");
#elif defined(__powerpc64__)
    printf("#ifdef __powerpc64__\n");
#elif defined(__riscv) && __WORDSIZE == 64
    printf("#if defined(__riscv) && __WORDSIZE == 64\n");
#else
#error "unimplemented architecture"
#endif

    list();

    int r = printf("#endif\n");
    return r != 7;
}
