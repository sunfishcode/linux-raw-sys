#!/usr/bin/env bash
set -ueo pipefail

cd -- "$(dirname -- "${BASH_SOURCE[0]}")"

linux_version="$(sed -n 's/^const LINUX_VERSION: &str = "\(v.*\)";$/\1/p' ../src/main.rs)"

pushd ../linux
git clean -fd
git checkout "$linux_version" -f
git clean -fd
popd

tmp_dir="$(mktemp --tmpdir -d linux-raw-sys-ioctl.XXXXXXXXXX)"
header_dir="$tmp_dir/linux-headers"
mkdir "$header_dir"

cleanup() {
  rm -r "$tmp_dir"
}
trap cleanup EXIT

install_headers() {
  arch="$1"
  rm -r "$header_dir"
  make -C ../linux headers_install ARCH="$arch" INSTALL_HDR_PATH="$header_dir"
}

includes=(
  -nostdinc
  -Iinclude
  "-I$header_dir/include"
)
out="$tmp_dir/ioctl.h"

generate() {
  install_headers "$1"
  "${CLANG:-clang}" --target="$2" "${includes[@]}" -Wall -std=gnu11 \
    -DIOCTL_EXTRACT -fsyntax-only -fno-color-diagnostics -Xclang -ast-dump \
    -Xclang -ast-dump-filter=ioctl_extract_constants list.c > "$tmp_dir/list.ast"
  awk -v condition="$3" -v names="$tmp_dir/generated.txt" \
    -f extract.awk "$tmp_dir/list.ast" >> "$out"
}

echo "// This file is generated from the ioctl/generate.sh script." > "$out"

generate x86 i686-linux-gnu '#ifdef __i386__'
generate x86_64 x86_64-linux-gnu '#ifdef __x86_64__'
generate arm64 aarch64-linux-gnu '#ifdef __aarch64__'
generate arm arm-linux-gnueabihf '#ifdef __arm__'
generate powerpc powerpc64le-linux-gnu '#ifdef __powerpc64__'
generate powerpc powerpc-linux-gnu '#if defined(__powerpc__) && !defined(__powerpc64__)'
generate mips mips64el-linux-gnuabi64 '#if __mips == 64'
generate mips mipsel-linux-gnu '#if __mips == 32'
generate riscv riscv32-linux-gnu '#if defined(__riscv) && __riscv_xlen == 32'
generate riscv riscv64-linux-gnu '#if defined(__riscv) && __riscv_xlen == 64'
generate s390 s390x-linux-gnu '#if defined(__s390x__)'
generate loongarch loongarch64-linux-gnu '#ifdef __loongarch__'
generate csky csky-linux-gnu '#ifdef __csky__'

# clang's m68k structure layout appears to differ from the Linux/GCC ABI.
install_headers m68k
"${M68K_CC:-m68k-linux-gnu-gcc}" "${includes[@]}" -Wall -c list.c -o "$tmp_dir/list.o"
"${M68K_CC:-m68k-linux-gnu-gcc}" -Wall main.c "$tmp_dir/list.o" -o "$tmp_dir/main.exe"
(
  cd "$tmp_dir"
  qemu-m68k -L "${M68K_SYSROOT:-/usr/m68k-linux-gnu}" ./main.exe
) >> "$out"

echo '#include "ioctl-addendum.h"' >> "$out"

mv "$out" ../modules/ioctl.h
mv "$tmp_dir/generated.txt" generated.txt
