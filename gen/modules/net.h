// This file includes selected Linux header files, and supplementary
// definitions, needed for general-purpose network code.

#include "support.h"

// Selected Linux headers.

#include <linux/in.h>
#include <linux/ip.h>
#include <linux/in6.h>
#include <linux/ipv6.h>
#include <linux/net.h>
#include <linux/socket.h>
#include <linux/tcp.h>
#include <linux/un.h>
#include <linux/netfilter_ipv6/ip6_tables.h>
#include <linux/netfilter_ipv4.h>

#if LINUX_VERSION_CODE >= KERNEL_VERSION(3,7,0)
#include <linux/net_tstamp.h>
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

struct linger {
    int l_onoff;
    int l_linger;
};

#define SHUT_RD   0
#define SHUT_WR   1
#define SHUT_RDWR 2

typedef __UINT32_TYPE__ socklen_t;

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
#define AF_CAIF       37
#define AF_ALG        38
#define AF_NFC        39
#define AF_VSOCK      40
#define AF_KCM        41
#define AF_QIPCRTR    42
#define AF_SMC        43
#define AF_XDP        44
#define AF_MCTP       45
#define AF_MAX        46

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

#define SOL_IP		0
#define SOL_TCP		6
#define SOL_UDP		17
#define SOL_IPV6	41
#define SOL_ICMPV6	58
#define SOL_SCTP	132
#define SOL_UDPLITE	136
#define SOL_RAW		255
#define SOL_IPX		256
#define SOL_AX25	257
#define SOL_ATALK	258
#define SOL_NETROM	259
#define SOL_ROSE	260
#define SOL_DECNET	261
#define	SOL_X25		262
#define SOL_PACKET	263
#define SOL_ATM		264
#define SOL_AAL		265
#define SOL_IRDA    266
#define SOL_NETBEUI	267
#define SOL_LLC		268
#define SOL_DCCP	269
#define SOL_NETLINK	270
#define SOL_TIPC	271
#define SOL_RXRPC	272
#define SOL_PPPOL2TP	273
#define SOL_BLUETOOTH	274
#define SOL_PNPIPE	275
#define SOL_RDS		276
#define SOL_IUCV	277
#define SOL_CAIF	278
#define SOL_ALG		279
#define SOL_NFC		280
#define SOL_KCM		281
#define SOL_TLS		282
#define SOL_XDP		283
#define SOL_MPTCP	284
#define SOL_MCTP	285
#define SOL_SMC		286
