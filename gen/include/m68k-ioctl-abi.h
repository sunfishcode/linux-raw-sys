#ifndef LINUX_RAW_SYS_M68K_IOCTL_ABI_H
#define LINUX_RAW_SYS_M68K_IOCTL_ABI_H

#ifdef __m68k__
#ifndef __ASSEMBLY__

/*
 * Linux m68k uses asm-generic/int-ll64.h, but GCC gives unsigned long long
 * 2-byte alignment while Clang's m68k target gives it 8-byte alignment. For
 * ioctl macro evaluation, define the UAPI integer typedefs with GCC-compatible
 * alignment before asm-generic/int-ll64.h is included.
 */
#ifndef _ASM_GENERIC_INT_LL64_H
#define _ASM_GENERIC_INT_LL64_H

#include <asm/bitsperlong.h>

typedef __signed__ char __s8;
typedef unsigned char __u8;

typedef __signed__ short __s16;
typedef unsigned short __u16;

typedef __signed__ int __s32;
typedef unsigned int __u32;

#ifdef __GNUC__
__extension__ typedef __signed__ long long __s64 __attribute__((aligned(2)));
__extension__ typedef unsigned long long __u64 __attribute__((aligned(2)));
#else
typedef __signed__ long long __s64 __attribute__((aligned(2)));
typedef unsigned long long __u64 __attribute__((aligned(2)));
#endif

#endif /* _ASM_GENERIC_INT_LL64_H */

#endif /* __ASSEMBLY__ */
#endif /* __m68k__ */

#endif /* LINUX_RAW_SYS_M68K_IOCTL_ABI_H */
