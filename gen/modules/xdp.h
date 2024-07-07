// This file includes selected Linux header files, and supplementary
// definitions, needed for AF_XDP code.

#include "support.h"

// Selected Linux headers.

#if LINUX_VERSION_CODE >= KERNEL_VERSION(4,18,0)
#include <linux/if_xdp.h>

// Previous versions of xdp structs. They are defined in linux/net/xdp/xsk.c and
// don't appear in header files, so they are defined here for backwards compatibility.

// https://github.com/torvalds/linux/blob/v6.6/net/xdp/xsk.h#L14-L18
struct xdp_ring_offset_v1 {
    __u64 producer;
    __u64 consumer;
    __u64 desc;
};

// https://github.com/torvalds/linux/blob/v6.6/net/xdp/xsk.h#L20-L25
struct xdp_mmap_offsets_v1 {
    struct xdp_ring_offset_v1 rx;
    struct xdp_ring_offset_v1 tx;
    struct xdp_ring_offset_v1 fr;
    struct xdp_ring_offset_v1 cr;
};

// https://github.com/torvalds/linux/blob/v6.6/net/xdp/xsk.c#L1251-L1256
struct xdp_umem_reg_v1 {
    __u64 addr;
    __u64 len;
    __u32 chunk_size;
    __u32 headroom;
};

// https://github.com/torvalds/linux/blob/v6.8/net/xdp/xsk.c#L1334-L1340
struct xdp_umem_reg_v2 {
	__u64 addr;
	__u64 len;
	__u32 chunk_size;
	__u32 headroom;
	__u32 flags;
};

// https://github.com/torvalds/linux/blob/v6.6/net/xdp/xsk.c#L1367-L1371
struct xdp_statistics_v1 {
    __u64 rx_dropped;
    __u64 rx_invalid_descs;
    __u64 tx_invalid_descs;
};

#endif

