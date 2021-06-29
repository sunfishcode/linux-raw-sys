// Linux 4.2 mips include/asm/siginfo.h includes uapi/asm-generic/siginfo.h
// but it doesn't exist in that version. Provide it.
#include <asm-generic/siginfo.h>
