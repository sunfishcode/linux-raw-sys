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
#elif defined(__powerpc64__)
    printf("#ifdef __powerpc64__\n");
#elif defined(__powerpc__) && !defined(__powerpc64__)
    printf("#if defined(__powerpc__) && !defined(__powerpc64__)\n");
#elif __mips == 32
    printf("#if __mips == 32\n");
#elif __mips == 64
    printf("#if __mips == 64\n");
#elif defined(__riscv) && __riscv_xlen == 64
    printf("#if defined(__riscv) && __riscv_xlen == 64\n");
#elif defined(__riscv) && __riscv_xlen == 32
    printf("#if defined(__riscv) && __riscv_xlen == 32\n");
#elif defined(__s390x__)
    printf("#if defined(__s390x__)\n");
#elif defined(__loongarch__)
    printf("#ifdef __loongarch__\n");
#elif defined(__csky__)
    printf("#ifdef __csky__\n");
#elif defined(__m68k__)
    printf("#ifdef __m68k__\n");
#else
#error "unimplemented architecture"
#endif

    list();

    int r = printf("#endif\n");
    return r != 7;
}
