/* automatically generated by rust-bindgen 0.71.1 */

pub type __s8 = crate::ctypes::c_schar;
pub type __u8 = crate::ctypes::c_uchar;
pub type __s16 = crate::ctypes::c_short;
pub type __u16 = crate::ctypes::c_ushort;
pub type __s32 = crate::ctypes::c_int;
pub type __u32 = crate::ctypes::c_uint;
pub type __s64 = crate::ctypes::c_longlong;
pub type __u64 = crate::ctypes::c_ulonglong;
pub type __kernel_key_t = crate::ctypes::c_int;
pub type __kernel_mqd_t = crate::ctypes::c_int;
pub type __kernel_daddr_t = crate::ctypes::c_long;
pub type __kernel_long_t = crate::ctypes::c_long;
pub type __kernel_ulong_t = crate::ctypes::c_ulong;
pub type __kernel_ino_t = __kernel_ulong_t;
pub type __kernel_mode_t = crate::ctypes::c_uint;
pub type __kernel_pid_t = crate::ctypes::c_int;
pub type __kernel_ipc_pid_t = crate::ctypes::c_int;
pub type __kernel_uid_t = crate::ctypes::c_uint;
pub type __kernel_gid_t = crate::ctypes::c_uint;
pub type __kernel_suseconds_t = __kernel_long_t;
pub type __kernel_uid32_t = crate::ctypes::c_uint;
pub type __kernel_gid32_t = crate::ctypes::c_uint;
pub type __kernel_old_uid_t = __kernel_uid_t;
pub type __kernel_old_gid_t = __kernel_gid_t;
pub type __kernel_old_dev_t = crate::ctypes::c_uint;
pub type __kernel_size_t = crate::ctypes::c_uint;
pub type __kernel_ssize_t = crate::ctypes::c_int;
pub type __kernel_ptrdiff_t = crate::ctypes::c_int;
pub type __kernel_off_t = __kernel_long_t;
pub type __kernel_loff_t = crate::ctypes::c_longlong;
pub type __kernel_old_time_t = __kernel_long_t;
pub type __kernel_time_t = __kernel_long_t;
pub type __kernel_time64_t = crate::ctypes::c_longlong;
pub type __kernel_clock_t = __kernel_long_t;
pub type __kernel_timer_t = crate::ctypes::c_int;
pub type __kernel_clockid_t = crate::ctypes::c_int;
pub type __kernel_caddr_t = *mut crate::ctypes::c_char;
pub type __kernel_uid16_t = crate::ctypes::c_ushort;
pub type __kernel_gid16_t = crate::ctypes::c_ushort;
pub type __le16 = __u16;
pub type __be16 = __u16;
pub type __le32 = __u32;
pub type __be32 = __u32;
pub type __le64 = __u64;
pub type __be64 = __u64;
pub type __sum16 = __u16;
pub type __wsum = __u32;
pub type __poll_t = crate::ctypes::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sockaddr_pkt {
pub spkt_family: crate::ctypes::c_ushort,
pub spkt_device: [crate::ctypes::c_uchar; 14usize],
pub spkt_protocol: __be16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sockaddr_ll {
pub sll_family: crate::ctypes::c_ushort,
pub sll_protocol: __be16,
pub sll_ifindex: crate::ctypes::c_int,
pub sll_hatype: crate::ctypes::c_ushort,
pub sll_pkttype: crate::ctypes::c_uchar,
pub sll_halen: crate::ctypes::c_uchar,
pub sll_addr: [crate::ctypes::c_uchar; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tpacket_stats {
pub tp_packets: crate::ctypes::c_uint,
pub tp_drops: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tpacket_stats_v3 {
pub tp_packets: crate::ctypes::c_uint,
pub tp_drops: crate::ctypes::c_uint,
pub tp_freeze_q_cnt: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tpacket_rollover_stats {
pub tp_all: __u64,
pub tp_huge: __u64,
pub tp_failed: __u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tpacket_auxdata {
pub tp_status: __u32,
pub tp_len: __u32,
pub tp_snaplen: __u32,
pub tp_mac: __u16,
pub tp_net: __u16,
pub tp_vlan_tci: __u16,
pub tp_vlan_tpid: __u16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tpacket_hdr {
pub tp_status: crate::ctypes::c_ulong,
pub tp_len: crate::ctypes::c_uint,
pub tp_snaplen: crate::ctypes::c_uint,
pub tp_mac: crate::ctypes::c_ushort,
pub tp_net: crate::ctypes::c_ushort,
pub tp_sec: crate::ctypes::c_uint,
pub tp_usec: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tpacket2_hdr {
pub tp_status: __u32,
pub tp_len: __u32,
pub tp_snaplen: __u32,
pub tp_mac: __u16,
pub tp_net: __u16,
pub tp_sec: __u32,
pub tp_nsec: __u32,
pub tp_vlan_tci: __u16,
pub tp_vlan_tpid: __u16,
pub tp_padding: [__u8; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tpacket_hdr_variant1 {
pub tp_rxhash: __u32,
pub tp_vlan_tci: __u32,
pub tp_vlan_tpid: __u16,
pub tp_padding: __u16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpacket3_hdr {
pub tp_next_offset: __u32,
pub tp_sec: __u32,
pub tp_nsec: __u32,
pub tp_snaplen: __u32,
pub tp_len: __u32,
pub tp_status: __u32,
pub tp_mac: __u16,
pub tp_net: __u16,
pub __bindgen_anon_1: tpacket3_hdr__bindgen_ty_1,
pub tp_padding: [__u8; 8usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpacket_bd_ts {
pub ts_sec: crate::ctypes::c_uint,
pub __bindgen_anon_1: tpacket_bd_ts__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpacket_hdr_v1 {
pub block_status: __u32,
pub num_pkts: __u32,
pub offset_to_first_pkt: __u32,
pub blk_len: __u32,
pub seq_num: __u64,
pub ts_first_pkt: tpacket_bd_ts,
pub ts_last_pkt: tpacket_bd_ts,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpacket_block_desc {
pub version: __u32,
pub offset_to_priv: __u32,
pub hdr: tpacket_bd_header_u,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tpacket_req {
pub tp_block_size: crate::ctypes::c_uint,
pub tp_block_nr: crate::ctypes::c_uint,
pub tp_frame_size: crate::ctypes::c_uint,
pub tp_frame_nr: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tpacket_req3 {
pub tp_block_size: crate::ctypes::c_uint,
pub tp_block_nr: crate::ctypes::c_uint,
pub tp_frame_size: crate::ctypes::c_uint,
pub tp_frame_nr: crate::ctypes::c_uint,
pub tp_retire_blk_tov: crate::ctypes::c_uint,
pub tp_sizeof_priv: crate::ctypes::c_uint,
pub tp_feature_req_word: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct packet_mreq {
pub mr_ifindex: crate::ctypes::c_int,
pub mr_type: crate::ctypes::c_ushort,
pub mr_alen: crate::ctypes::c_ushort,
pub mr_address: [crate::ctypes::c_uchar; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fanout_args {
pub type_flags: __u16,
pub id: __u16,
pub max_num_members: __u32,
}
pub const __BIG_ENDIAN: u32 = 4321;
pub const __BITS_PER_LONG_LONG: u32 = 64;
pub const _MIPS_ISA_MIPS1: u32 = 1;
pub const _MIPS_ISA_MIPS2: u32 = 2;
pub const _MIPS_ISA_MIPS3: u32 = 3;
pub const _MIPS_ISA_MIPS4: u32 = 4;
pub const _MIPS_ISA_MIPS5: u32 = 5;
pub const _MIPS_ISA_MIPS32: u32 = 6;
pub const _MIPS_ISA_MIPS64: u32 = 7;
pub const _MIPS_SIM_ABI32: u32 = 1;
pub const _MIPS_SIM_NABI32: u32 = 2;
pub const _MIPS_SIM_ABI64: u32 = 3;
pub const PACKET_HOST: u32 = 0;
pub const PACKET_BROADCAST: u32 = 1;
pub const PACKET_MULTICAST: u32 = 2;
pub const PACKET_OTHERHOST: u32 = 3;
pub const PACKET_OUTGOING: u32 = 4;
pub const PACKET_LOOPBACK: u32 = 5;
pub const PACKET_USER: u32 = 6;
pub const PACKET_KERNEL: u32 = 7;
pub const PACKET_FASTROUTE: u32 = 6;
pub const PACKET_ADD_MEMBERSHIP: u32 = 1;
pub const PACKET_DROP_MEMBERSHIP: u32 = 2;
pub const PACKET_RECV_OUTPUT: u32 = 3;
pub const PACKET_RX_RING: u32 = 5;
pub const PACKET_STATISTICS: u32 = 6;
pub const PACKET_COPY_THRESH: u32 = 7;
pub const PACKET_AUXDATA: u32 = 8;
pub const PACKET_ORIGDEV: u32 = 9;
pub const PACKET_VERSION: u32 = 10;
pub const PACKET_HDRLEN: u32 = 11;
pub const PACKET_RESERVE: u32 = 12;
pub const PACKET_TX_RING: u32 = 13;
pub const PACKET_LOSS: u32 = 14;
pub const PACKET_VNET_HDR: u32 = 15;
pub const PACKET_TX_TIMESTAMP: u32 = 16;
pub const PACKET_TIMESTAMP: u32 = 17;
pub const PACKET_FANOUT: u32 = 18;
pub const PACKET_TX_HAS_OFF: u32 = 19;
pub const PACKET_QDISC_BYPASS: u32 = 20;
pub const PACKET_ROLLOVER_STATS: u32 = 21;
pub const PACKET_FANOUT_DATA: u32 = 22;
pub const PACKET_IGNORE_OUTGOING: u32 = 23;
pub const PACKET_VNET_HDR_SZ: u32 = 24;
pub const PACKET_FANOUT_HASH: u32 = 0;
pub const PACKET_FANOUT_LB: u32 = 1;
pub const PACKET_FANOUT_CPU: u32 = 2;
pub const PACKET_FANOUT_ROLLOVER: u32 = 3;
pub const PACKET_FANOUT_RND: u32 = 4;
pub const PACKET_FANOUT_QM: u32 = 5;
pub const PACKET_FANOUT_CBPF: u32 = 6;
pub const PACKET_FANOUT_EBPF: u32 = 7;
pub const PACKET_FANOUT_FLAG_ROLLOVER: u32 = 4096;
pub const PACKET_FANOUT_FLAG_UNIQUEID: u32 = 8192;
pub const PACKET_FANOUT_FLAG_IGNORE_OUTGOING: u32 = 16384;
pub const PACKET_FANOUT_FLAG_DEFRAG: u32 = 32768;
pub const TP_STATUS_KERNEL: u32 = 0;
pub const TP_STATUS_USER: u32 = 1;
pub const TP_STATUS_COPY: u32 = 2;
pub const TP_STATUS_LOSING: u32 = 4;
pub const TP_STATUS_CSUMNOTREADY: u32 = 8;
pub const TP_STATUS_VLAN_VALID: u32 = 16;
pub const TP_STATUS_BLK_TMO: u32 = 32;
pub const TP_STATUS_VLAN_TPID_VALID: u32 = 64;
pub const TP_STATUS_CSUM_VALID: u32 = 128;
pub const TP_STATUS_GSO_TCP: u32 = 256;
pub const TP_STATUS_AVAILABLE: u32 = 0;
pub const TP_STATUS_SEND_REQUEST: u32 = 1;
pub const TP_STATUS_SENDING: u32 = 2;
pub const TP_STATUS_WRONG_FORMAT: u32 = 4;
pub const TP_STATUS_TS_SOFTWARE: u32 = 536870912;
pub const TP_STATUS_TS_SYS_HARDWARE: u32 = 1073741824;
pub const TP_STATUS_TS_RAW_HARDWARE: u32 = 2147483648;
pub const TP_FT_REQ_FILL_RXHASH: u32 = 1;
pub const TPACKET_ALIGNMENT: u32 = 16;
pub const PACKET_MR_MULTICAST: u32 = 0;
pub const PACKET_MR_PROMISC: u32 = 1;
pub const PACKET_MR_ALLMULTI: u32 = 2;
pub const PACKET_MR_UNICAST: u32 = 3;
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum tpacket_versions {
TPACKET_V1 = 0,
TPACKET_V2 = 1,
TPACKET_V3 = 2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union tpacket_stats_u {
pub stats1: tpacket_stats,
pub stats3: tpacket_stats_v3,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union tpacket3_hdr__bindgen_ty_1 {
pub hv1: tpacket_hdr_variant1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union tpacket_bd_ts__bindgen_ty_1 {
pub ts_usec: crate::ctypes::c_uint,
pub ts_nsec: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union tpacket_bd_header_u {
pub bh1: tpacket_hdr_v1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union tpacket_req_u {
pub req: tpacket_req,
pub req3: tpacket_req3,
}
