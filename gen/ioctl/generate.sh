#!/bin/bash
set -ueo pipefail

# Linux's ioctl codes using function-style macros in their definition which
# bindgen is unable to evaluate. To extract the ioctl code values, we have
# a small C program, main.c, which prints the values of the ioctls on the
# platform it's compiled for. This script compiles it for several platforms
# and generates an ioctl.h header file, which bindgen can fully process,
# producing an ioctl module in the Rust bindings.
#
# This is a very simplistic and not yet portable script; if you need it run,
# to add new ioctl codes or a new architecture, and are unable to run it,
# please file an issue in the issue tracker.

cflags="-Wall -I."
out="../modules/ioctl.h"

echo "// This file is generated from the ioctl/generate.sh script." > "$out"

i686-linux-gnu-gcc main.c -o main.exe $cflags
./main.exe >> "$out"
x86_64-linux-gnu-gcc main.c -o main.exe $cflags
./main.exe >> "$out"
aarch64-linux-gnu-gcc main.c -o main.exe $cflags
qemu-aarch64 -L /usr/aarch64-linux-gnu ./main.exe >> "$out"
arm-linux-gnueabihf-gcc main.c -o main.exe $cflags
qemu-arm -L /usr/arm-linux-gnueabihf ./main.exe >> "$out"
powerpc64le-linux-gnu-gcc main.c -o main.exe $cflags
qemu-ppc64le -L /usr/powerpc64le-linux-gnu ./main.exe >> "$out"
riscv64-linux-gnu-gcc main.c -o main.exe $cflags
qemu-riscv64 -L /usr/riscv64-linux-gnu ./main.exe >> "$out"

rm main.exe
