// This file includes selected Linux header files, and supplementary
// definitions, needed for general-purpose code.

#include "support.h"

// Selected Linux headers.

#include <linux/auxvec.h>
#include <linux/capability.h>
#include <linux/eventpoll.h>
#include <linux/fadvise.h>
#include <linux/falloc.h>
#include <linux/fcntl.h>
#include <linux/fs.h>
#include <linux/futex.h>
#include <linux/in.h>
#include <linux/inotify.h>
#include <linux/ip.h>
#include <linux/in6.h>
#include <linux/ipv6.h>
#include <linux/limits.h>
#include <linux/magic.h>
#include <linux/mman.h>
#include <linux/net.h>
#include <linux/poll.h>
#include <linux/prctl.h>
#include <linux/random.h>
#include <linux/resource.h>
#include <linux/sched.h>
#include <linux/signal.h>
#include <linux/socket.h>
#include <linux/stat.h>
#include <linux/sysinfo.h>
#include <linux/tcp.h>
#include <linux/termios.h>
#include <linux/time.h>
#include <linux/types.h>
#include <linux/uio.h>
#include <linux/un.h>
#include <linux/unistd.h>
#include <linux/utsname.h>
#include <linux/wait.h>

#if LINUX_VERSION_CODE >= KERNEL_VERSION(3,17,0)
#include <linux/memfd.h>
#endif

#if LINUX_VERSION_CODE >= KERNEL_VERSION(4,3,0)
#include <linux/membarrier.h>
#endif

#if LINUX_VERSION_CODE >= KERNEL_VERSION(4,20,0)
#include <linux/timerfd.h>
#endif

#if LINUX_VERSION_CODE >= KERNEL_VERSION(5,6,0)
#include <linux/openat2.h>
#endif

#if LINUX_VERSION_CODE >= KERNEL_VERSION(4,3,0)
#include <linux/userfaultfd.h>
#endif

#if LINUX_VERSION_CODE >= KERNEL_VERSION(5,1,0)
#include <linux/io_uring.h>
#endif

// Miscellaneous definitions which don't appear to be defined in Linux's public
// headers, but which are nonetheless part of the ABI, and necessary for
// interoperability.
//
// When adding definitions here, please only include content needed for
// interoperability with Linux's public ABI, and please only include types
// and constants.
//
// In particular, please don't copy comments from other sources. And please
// don't include any functions or function-style macros, as bindgen isn't
// able to generate bindings for them.
//
// Also, please be aware that libc implementations (and thus the Rust libc
// crate as well) sometimes define types and constants with similar names but
// which are ABI-incompatible with the Linux kernel ABI. This file should
// only describe the kernel ABI.

struct sockaddr {
    struct __kernel_sockaddr_storage __storage;
};
#if LINUX_VERSION_CODE == KERNEL_VERSION(2,6,32)
typedef uint16_t __kernel_sa_family_t;
#endif

struct linger {
    int l_onoff;
    int l_linger;
};

#if LINUX_VERSION_CODE <= KERNEL_VERSION(4,4,0)
typedef long long __kernel_time64_t;
struct __kernel_timespec {
    __kernel_time64_t tv_sec;
    long long         tv_nsec;
};
#endif

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
// And use the libc name. And mips and s390x are special.
#if defined(__mips__) || defined(__s390x__)
typedef __u32 __fsword_t;
#elif defined(__mips64__)
typedef long __fsword_t;
#else
typedef __statfs_word __fsword_t;
#endif

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

#define F_OK 0
#define R_OK 4
#define W_OK 2
#define X_OK 1

#define UTIME_NOW 0x3fffffff
#define UTIME_OMIT 0x3ffffffe

#define MSG_DONTWAIT 0x40

#define AF_UNSPEC     0
#define AF_UNIX       1
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
#undef RWF_HIPRI
#undef RWF_DSYNC
#undef RWF_SYNC
#undef RWF_NOWAIT
#undef RWF_APPEND
#define RWF_HIPRI       0x00000001
#define RWF_DSYNC       0x00000002
#define RWF_SYNC        0x00000004
#define RWF_NOWAIT      0x00000008
#define RWF_APPEND      0x00000010

// Linux doesn't appear to export <linux/eventfd.h> at all.
#define EFD_SEMAPHORE   1
#define EFD_CLOEXEC     O_CLOEXEC
#define EFD_NONBLOCK    O_NONBLOCK

// Flags for epoll_create1.
#define EPOLL_CLOEXEC O_CLOEXEC

// Constants for `epoll_ctl`.
#define EPOLL_CTL_ADD 1
#define EPOLL_CTL_DEL 2
#define EPOLL_CTL_MOD 3

// Flags for epoll events
#define EPOLLIN        0x00000001
#define EPOLLPRI       0x00000002
#define EPOLLOUT       0x00000004
#define EPOLLERR       0x00000008
#define EPOLLHUP       0x00000010
#define EPOLLNVAL      0x00000020
#define EPOLLRDNORM    0x00000040
#define EPOLLRDBAND    0x00000080
#define EPOLLWRNORM    0x00000100
#define EPOLLWRBAND    0x00000200
#define EPOLLMSG       0x00000400
#define EPOLLRDHUP     0x00002000
#define EPOLLEXCLUSIVE 0x10000000
#define EPOLLWAKEUP    0x20000000
#define EPOLLONESHOT   0x40000000
#define EPOLLET        0x80000000

// Flags for timerfd
#define TFD_CLOEXEC            O_CLOEXEC
#define TFD_NONBLOCK           O_NONBLOCK
#define TFD_SHARED_FCNTL_FLAGS (TFD_CLOEXEC | TFD_NONBLOCK)
#define TFD_CREATE_FLAGS       TFD_SHARED_FCNTL_FLAGS
#define TFD_SETTIME_FLAGS      TFD_TIMER_ABSTIME

struct user_desc {
    unsigned entry_number;
    unsigned base_addr;
    unsigned limit;
    unsigned seg_32bit:1;
    unsigned contents:2;
    unsigned read_exec_only:1;
    unsigned limit_in_pages:1;
    unsigned seg_not_present:1;
    unsigned useable:1;
#ifdef __x86_64__
    unsigned lm:1;
#endif
};

#if defined(__x86_64__) || defined(__i386__)
#define ARCH_SET_FS 0x1002
#endif

struct msghdr {
    void         *msg_name;
    int           msg_namelen;
    struct iovec *msg_iov;
    size_t        msg_iovlen;
    void         *msg_control;
    size_t        msg_controllen;
    unsigned int  msg_flags;
};

struct cmsghdr {
    size_t cmsg_len;
    int    cmsg_level;
    int    cmsg_type;
};


#define SCM_RIGHTS      0x01
#define SCM_CREDENTIALS 0x02
#define SCM_SECURITY    0x03

struct ucred {
    __u32 pid;
    __u32 uid;
    __u32 gid;
};

struct mmsghdr {
    struct msghdr msg_hdr;
    unsigned int  msg_len;
};

#define UFFD_API 0xAA

#define UFFDIO_REGISTER_MODE_MISSING  0x1
#define UFFDIO_REGISTER_MODE_WP       0x2
#define UFFDIO_REGISTER_MODE_MINOR    0x4

#define UFFDIO_COPY_MODE_DONTWAKE     0x1
#define UFFDIO_COPY_MODE_WP           0x2

#define UFFDIO_ZEROPAGE_MODE_DONTWAKE 0x1

#if defined(__mips)
#define POLLWRNORM 0x4

#undef TCSANOW
#undef TCSADRAIN
#undef TCSAFLUSH
#define TCSANOW 0x540e
#define TCSADRAIN 0x540f
#define TCSAFLUSH 0x5410
#endif

#define SPLICE_F_MOVE      1
#define SPLICE_F_NONBLOCK  2
#define SPLICE_F_MORE      4
#define SPLICE_F_GIFT      8
