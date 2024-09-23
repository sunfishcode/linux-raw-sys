/* automatically generated by rust-bindgen 0.70.1 */

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
pub type __kernel_old_uid_t = crate::ctypes::c_ushort;
pub type __kernel_old_gid_t = crate::ctypes::c_ushort;
pub type __kernel_old_dev_t = crate::ctypes::c_ulong;
pub type __kernel_long_t = crate::ctypes::c_long;
pub type __kernel_ulong_t = crate::ctypes::c_ulong;
pub type __kernel_ino_t = __kernel_ulong_t;
pub type __kernel_mode_t = crate::ctypes::c_uint;
pub type __kernel_pid_t = crate::ctypes::c_int;
pub type __kernel_ipc_pid_t = crate::ctypes::c_int;
pub type __kernel_uid_t = crate::ctypes::c_uint;
pub type __kernel_gid_t = crate::ctypes::c_uint;
pub type __kernel_suseconds_t = __kernel_long_t;
pub type __kernel_daddr_t = crate::ctypes::c_int;
pub type __kernel_uid32_t = crate::ctypes::c_uint;
pub type __kernel_gid32_t = crate::ctypes::c_uint;
pub type __kernel_size_t = __kernel_ulong_t;
pub type __kernel_ssize_t = __kernel_long_t;
pub type __kernel_ptrdiff_t = __kernel_long_t;
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
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ethhdr {
pub h_dest: [crate::ctypes::c_uchar; 6usize],
pub h_source: [crate::ctypes::c_uchar; 6usize],
pub h_proto: __be16,
}
pub const ETH_ALEN: u32 = 6;
pub const ETH_TLEN: u32 = 2;
pub const ETH_HLEN: u32 = 14;
pub const ETH_ZLEN: u32 = 60;
pub const ETH_DATA_LEN: u32 = 1500;
pub const ETH_FRAME_LEN: u32 = 1514;
pub const ETH_FCS_LEN: u32 = 4;
pub const ETH_MIN_MTU: u32 = 68;
pub const ETH_MAX_MTU: u32 = 65535;
pub const ETH_P_LOOP: u32 = 96;
pub const ETH_P_PUP: u32 = 512;
pub const ETH_P_PUPAT: u32 = 513;
pub const ETH_P_TSN: u32 = 8944;
pub const ETH_P_ERSPAN2: u32 = 8939;
pub const ETH_P_IP: u32 = 2048;
pub const ETH_P_X25: u32 = 2053;
pub const ETH_P_ARP: u32 = 2054;
pub const ETH_P_BPQ: u32 = 2303;
pub const ETH_P_IEEEPUP: u32 = 2560;
pub const ETH_P_IEEEPUPAT: u32 = 2561;
pub const ETH_P_BATMAN: u32 = 17157;
pub const ETH_P_DEC: u32 = 24576;
pub const ETH_P_DNA_DL: u32 = 24577;
pub const ETH_P_DNA_RC: u32 = 24578;
pub const ETH_P_DNA_RT: u32 = 24579;
pub const ETH_P_LAT: u32 = 24580;
pub const ETH_P_DIAG: u32 = 24581;
pub const ETH_P_CUST: u32 = 24582;
pub const ETH_P_SCA: u32 = 24583;
pub const ETH_P_TEB: u32 = 25944;
pub const ETH_P_RARP: u32 = 32821;
pub const ETH_P_ATALK: u32 = 32923;
pub const ETH_P_AARP: u32 = 33011;
pub const ETH_P_8021Q: u32 = 33024;
pub const ETH_P_ERSPAN: u32 = 35006;
pub const ETH_P_IPX: u32 = 33079;
pub const ETH_P_IPV6: u32 = 34525;
pub const ETH_P_PAUSE: u32 = 34824;
pub const ETH_P_SLOW: u32 = 34825;
pub const ETH_P_WCCP: u32 = 34878;
pub const ETH_P_MPLS_UC: u32 = 34887;
pub const ETH_P_MPLS_MC: u32 = 34888;
pub const ETH_P_ATMMPOA: u32 = 34892;
pub const ETH_P_PPP_DISC: u32 = 34915;
pub const ETH_P_PPP_SES: u32 = 34916;
pub const ETH_P_LINK_CTL: u32 = 34924;
pub const ETH_P_ATMFATE: u32 = 34948;
pub const ETH_P_PAE: u32 = 34958;
pub const ETH_P_PROFINET: u32 = 34962;
pub const ETH_P_REALTEK: u32 = 34969;
pub const ETH_P_AOE: u32 = 34978;
pub const ETH_P_ETHERCAT: u32 = 34980;
pub const ETH_P_8021AD: u32 = 34984;
pub const ETH_P_802_EX1: u32 = 34997;
pub const ETH_P_PREAUTH: u32 = 35015;
pub const ETH_P_TIPC: u32 = 35018;
pub const ETH_P_LLDP: u32 = 35020;
pub const ETH_P_MRP: u32 = 35043;
pub const ETH_P_MACSEC: u32 = 35045;
pub const ETH_P_8021AH: u32 = 35047;
pub const ETH_P_MVRP: u32 = 35061;
pub const ETH_P_1588: u32 = 35063;
pub const ETH_P_NCSI: u32 = 35064;
pub const ETH_P_PRP: u32 = 35067;
pub const ETH_P_CFM: u32 = 35074;
pub const ETH_P_FCOE: u32 = 35078;
pub const ETH_P_IBOE: u32 = 35093;
pub const ETH_P_TDLS: u32 = 35085;
pub const ETH_P_FIP: u32 = 35092;
pub const ETH_P_80221: u32 = 35095;
pub const ETH_P_HSR: u32 = 35119;
pub const ETH_P_NSH: u32 = 35151;
pub const ETH_P_LOOPBACK: u32 = 36864;
pub const ETH_P_QINQ1: u32 = 37120;
pub const ETH_P_QINQ2: u32 = 37376;
pub const ETH_P_QINQ3: u32 = 37632;
pub const ETH_P_EDSA: u32 = 56026;
pub const ETH_P_DSA_8021Q: u32 = 56027;
pub const ETH_P_DSA_A5PSW: u32 = 57345;
pub const ETH_P_IFE: u32 = 60734;
pub const ETH_P_AF_IUCV: u32 = 64507;
pub const ETH_P_802_3_MIN: u32 = 1536;
pub const ETH_P_802_3: u32 = 1;
pub const ETH_P_AX25: u32 = 2;
pub const ETH_P_ALL: u32 = 3;
pub const ETH_P_802_2: u32 = 4;
pub const ETH_P_SNAP: u32 = 5;
pub const ETH_P_DDCMP: u32 = 6;
pub const ETH_P_WAN_PPP: u32 = 7;
pub const ETH_P_PPP_MP: u32 = 8;
pub const ETH_P_LOCALTALK: u32 = 9;
pub const ETH_P_CAN: u32 = 12;
pub const ETH_P_CANFD: u32 = 13;
pub const ETH_P_CANXL: u32 = 14;
pub const ETH_P_PPPTALK: u32 = 16;
pub const ETH_P_TR_802_2: u32 = 17;
pub const ETH_P_MOBITEX: u32 = 21;
pub const ETH_P_CONTROL: u32 = 22;
pub const ETH_P_IRDA: u32 = 23;
pub const ETH_P_ECONET: u32 = 24;
pub const ETH_P_HDLC: u32 = 25;
pub const ETH_P_ARCNET: u32 = 26;
pub const ETH_P_DSA: u32 = 27;
pub const ETH_P_TRAILER: u32 = 28;
pub const ETH_P_PHONET: u32 = 245;
pub const ETH_P_IEEE802154: u32 = 246;
pub const ETH_P_CAIF: u32 = 247;
pub const ETH_P_XDSA: u32 = 248;
pub const ETH_P_MAP: u32 = 249;
pub const ETH_P_MCTP: u32 = 250;
