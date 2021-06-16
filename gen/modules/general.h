// This file includes selected Linux header files, and supplementary
// definitions, needed for general-purpose code.

#include "support.h"

// Selected Linux headers.

#include <linux/fadvise.h>
#include <linux/fcntl.h>
#include <linux/fs.h>
#include <linux/futex.h>
#include <linux/in.h>
#include <linux/limits.h>
#include <linux/magic.h>
#include <linux/mman.h>
#include <linux/net.h>
#include <linux/poll.h>
#include <linux/prctl.h>
#include <linux/random.h>
#include <linux/resource.h>
#include <linux/signal.h>
#include <linux/socket.h>
#include <linux/stat.h>
#include <linux/termios.h>
#include <linux/time.h>
#include <linux/types.h>
#include <linux/uio.h>
#include <linux/un.h>
#include <linux/unistd.h>
#include <linux/wait.h>

#if LINUX_VERSION_CODE >= KERNEL_VERSION(3,17,0)
#include <linux/memfd.h>
#endif

#if LINUX_VERSION_CODE >= KERNEL_VERSION(5,6,0)
#include <linux/openat2.h>
#endif

// Miscellaneous definitions which don't appear to be defined in Linux's UAPI
// headers but which are nonetheless part of the ABI.

struct sockaddr {
    struct __kernel_sockaddr_storage __storage;
};

#define DT_UNKNOWN 0
#define DT_FIFO    1
#define DT_CHR     2
#define DT_DIR     4
#define DT_BLK     6
#define DT_REG     8
#define DT_LNK    10
#define DT_SOCK   12

#define WEXITSTATUS(status) (((status) & 0xff00) >> 8)
#define WIFEXITED(status)   (((status) & 0x7f) == 0)

#define SHUT_RD   0
#define SHUT_WR   1
#define SHUT_RDWR 2

struct linux_dirent64 {
    __UINT64_TYPE__ d_ino;
    __INT64_TYPE__  d_off;
    __u16           d_reclen;
    __u8            d_type;
    char            d_name[];
};

typedef __UINT32_TYPE__ socklen_t;

// Obtain the definitions of structs stat/stat64 and statfs/statfs64.
#include <asm/stat.h>
#include <asm/statfs.h>

// Linux only defines this as a macro; make it available as a typedef.
// And use the libc name. And mips and s930x are special.
#if defined(__mips__) || defined(__s390x__)
typedef __u32 __fsword_t;
#elif defined(__mips64__)
typedef long __fsword_t;
#else
typedef __statfs_word __fsword_t;
#endif

#define major(x) ((__u32)((((x) >> 31 >> 1) & 0xfffff000) | (((x) >> 8) & 0x00000fff)))
#define minor(x) ((__u32)((((x) >> 12) & 0xffffff00) | ((x) & 0x000000ff)))
#define makedev(x,y) \
    ((((x) & 0xfffff000ULL) << 32) | (((x) & 0x00000fffULL) << 8) | \
     (((y) & 0xffffff00ULL) << 12) | (((y) & 0x000000ffULL)))

#if defined(__mips__) || defined(__mips64__)
#define SOCK_STREAM    2
#define SOCK_DGRAM     1
#else
#define SOCK_STREAM    1
#define SOCK_DGRAM     2
#endif
#define SOCK_RAW       3
#define SOCK_RDM       4
#define SOCK_SEQPACKET 5

// `SOCK_CLOEXEC` is defined to `O_CLOEXEC` but it appears bindgen doesn't
// understand that.
#define SOCK_CLOEXEC   02000000

#define F_OK 0
#define R_OK 4
#define W_OK 2
#define X_OK 1

#define UTIME_NOW 0x3fffffff
#define UTIME_OMIT 0x3ffffffe

#define MSG_PEEK 2
#define MSG_DONTWAIT 0x40

#define AF_UNSPEC     0
#define AF_LOCAL      1
#define AF_INET       2
#define AF_AX25       3
#define AF_IPX        4
#define AF_APPLETALK  5
#define AF_NETROM     6
#define AF_BRIDGE     7
#define AF_ATMPVC     8
#define AF_X25        9
#define AF_INET6      10
#define AF_ROSE       11
#define AF_DECnet     12
#define AF_NETBEUI    13
#define AF_SECURITY   14
#define AF_KEY        15
#define AF_NETLINK    16
#define AF_PACKET     17
#define AF_ASH        18
#define AF_ECONET     19
#define AF_ATMSVC     20
#define AF_RDS        21
#define AF_SNA        22
#define AF_IRDA       23
#define AF_PPPOX      24
#define AF_WANPIPE    25
#define AF_LLC        26
#define AF_CAN        29
#define AF_TIPC       30
#define AF_BLUETOOTH  31
#define AF_IUCV       32
#define AF_RXRPC      33
#define AF_ISDN       34
#define AF_PHONET     35
#define AF_IEEE802154 36
#define AF_MAX        37

#define MSG_OOB          0x1
#define MSG_PEEK         0x2
#define MSG_DONTROUTE    0x4
#define MSG_CTRUNC       0x8
#define MSG_PROBE        0x10
#define MSG_TRUNC        0x20
#define MSG_DONTWAIT     0x40
#define MSG_EOR          0x80
#define MSG_WAITALL      0x100
#define MSG_FIN          0x200
#define MSG_SYN          0x400
#define MSG_CONFIRM      0x800
#define MSG_RST          0x1000
#define MSG_ERRQUEUE     0x2000
#define MSG_NOSIGNAL     0x4000
#define MSG_MORE         0x8000
#define MSG_CMSG_CLOEXEC 0x40000000

#define STDIN_FILENO  0
#define STDOUT_FILENO 1
#define STDERR_FILENO 2

// Linux exports these, but the definitions have syntax that bindgen doesn't
// recognize as constants.
#define RWF_HIPRI       0x00000001
#define RWF_DSYNC       0x00000002
#define RWF_SYNC        0x00000004
#define RWF_NOWAIT      0x00000008
#define RWF_APPEND      0x00000010
