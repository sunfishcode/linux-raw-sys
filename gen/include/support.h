#include <linux/version.h>

// Various bits of support needed by Linux headers.

typedef __UINT8_TYPE__ uint8_t;
typedef __UINT16_TYPE__ uint16_t;
typedef __UINT32_TYPE__ uint32_t;
typedef __UINT64_TYPE__ uint64_t;
typedef __INT8_TYPE__ int8_t;
typedef __INT16_TYPE__ int16_t;
typedef __INT32_TYPE__ int32_t;
typedef __INT64_TYPE__ int64_t;
typedef __SIZE_TYPE__ size_t;
typedef __PTRDIFF_TYPE__ ssize_t;
typedef __PTRDIFF_TYPE__ ptrdiff_t;
typedef __INTPTR_TYPE__ intptr_t;
typedef __UINTPTR_TYPE__ uintptr_t;

#define INT_MAX __INT_MAX__
#define INT_MIN	(-__INT_MAX__ - 1)

#define memcpy __builtin_memcpy
#define memset __builtin_memset
#define strlen __builtin_strlen

#define __builtin_bswap_16 __builtin_bswap16
#define __builtin_bswap_32 __builtin_bswap32
#define __builtin_bswap_64 __builtin_bswap64

#if __BYTE_ORDER__ == __ORDER_BIG_ENDIAN__
#define ntohl(x)     (x)
#define ntohs(x)     (x)
#define htonl(x)     (x)
#define htons(x)     (x)
#elif __BYTE_ORDER__ == __ORDER_LITTLE_ENDIAN__
#define ntohl(x)     (__builtin_bswap32(x))
#define ntohs(x)     (__builtin_bswap16(x))
#define htonl(x)     (__builtin_bswap32(x))
#define htons(x)     (__builtin_bswap16(x))
#endif

#define pid_t       __kernel_pid_t
#define ino_t       __kernel_ino_t
#define mode_t      __kernel_mode_t
#define nlink_t     __kernel_nlink_t
#define uid_t       __kernel_uid_t
#define gid_t       __kernel_gid_t
#define off_t       __kernel_off_t
#define time_t      __kernel_time_t
#define timer_t     __kernel_timer_t
#define clock_t     __kernel_clock_t

#if LINUX_VERSION_CODE <= KERNEL_VERSION(2,6,32)
#define sa_family_t uint16_t
#endif
