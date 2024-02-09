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

cflags="-Wall"
out="../modules/ioctl.h"

echo "// This file is generated from the ioctl/generate.sh script." > "$out"

i686-linux-gnu-gcc -Iinclude -c list.c $cflags
i686-linux-gnu-gcc main.c list.o -o main.exe $cflags
./main.exe >> "$out"
x86_64-linux-gnu-gcc -Iinclude -c list.c $cflags
x86_64-linux-gnu-gcc main.c list.o -o main.exe $cflags
./main.exe >> "$out"
aarch64-linux-gnu-gcc -Iinclude -c list.c $cflags
aarch64-linux-gnu-gcc main.c list.o -o main.exe $cflags
qemu-aarch64 -L /usr/aarch64-linux-gnu ./main.exe >> "$out"
arm-linux-gnueabihf-gcc -Iinclude -c list.c $cflags
arm-linux-gnueabihf-gcc main.c list.o -o main.exe $cflags
qemu-arm -L /usr/arm-linux-gnueabihf ./main.exe >> "$out"
powerpc64le-linux-gnu-gcc -Iinclude -c list.c $cflags
powerpc64le-linux-gnu-gcc main.c list.o -o main.exe $cflags
qemu-ppc64le -L /usr/powerpc64le-linux-gnu ./main.exe >> "$out"
mips64el-linux-gnuabi64-gcc -Iinclude -c list.c $cflags
mips64el-linux-gnuabi64-gcc main.c list.o -o main.exe $cflags
qemu-mips64el -L /usr/mips64el-linux-gnuabi64 ./main.exe >> "$out"
mipsel-linux-gnu-gcc -Iinclude -c list.c $cflags
mipsel-linux-gnu-gcc main.c list.o -o main.exe $cflags
qemu-mipsel -L /usr/mipsel-linux-gnu ./main.exe >> "$out"

# RISCV32 tolchains are not yet packaged by major distributions e.g. debian etc.
# Therefore download it from https://github.com/riscv-collab/riscv-gnu-toolchain/releases
# e.g. riscv32-glibc-ubuntu-22.04-gcc-nightly-2024.02.02-nightly.tar.gz
# install it into /opt and then running below commands will succeed
# /opt/riscv/bin/riscv32-unknown-linux-gnu-gcc --sysroot=/opt/riscv/sysroot/ -Iinclude -c list.c $cflags
# /opt/riscv/bin/riscv32-unknown-linux-gnu-gcc --sysroot=/opt/riscv/sysroot/ main.c list.o -o main.exe $cflags
# /opt/riscv/bin/qemu-riscv32 -L /opt/riscv/sysroot/ ./main.exe >> "$out"
cat riscv32-ioctls.txt >> "$out"

riscv64-linux-gnu-gcc -Iinclude -c list.c $cflags
riscv64-linux-gnu-gcc main.c list.o -o main.exe $cflags
qemu-riscv64 -L /usr/riscv64-linux-gnu ./main.exe >> "$out"
s390x-linux-gnu-gcc -Iinclude -c list.c $cflags
s390x-linux-gnu-gcc main.c list.o -o main.exe $cflags
qemu-s390x -L /usr/s390x-linux-gnu ./main.exe >> "$out"
# As LoongArch and CSKY cross toolchain is not yet packaged in mainstream distros yet,
# pre-generated output is used for the time being
cat loongarch-ioctls.txt >> "$out"
# csky-linux-gnuabiv2-gcc -Iinclude -I../linux/usr/include/ -c list.c $cflags
# csky-linux-gnuabiv2-gcc  main.c list.o -o main.exe $cflags
# qemu-csky -L /usr/csky-linux-gnuabiv2 ./main.exe >> "$out"
cat csky-ioctls.txt >> "$out"

# Add any extra custom definitions at the end.
echo "#include \"ioctl-addendum.h\"" >> "$out"

rm list.o main.exe
