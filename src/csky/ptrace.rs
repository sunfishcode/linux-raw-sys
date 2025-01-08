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
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::core::marker::PhantomData<T>, [T; 0]);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_status {
pub mask: __u32,
pub enabled: __u32,
pub failure: __u32,
pub pid: __u32,
pub rate_limit: __u32,
pub backlog_limit: __u32,
pub lost: __u32,
pub backlog: __u32,
pub __bindgen_anon_1: audit_status__bindgen_ty_1,
pub backlog_wait_time: __u32,
pub backlog_wait_time_actual: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct audit_features {
pub vers: __u32,
pub mask: __u32,
pub features: __u32,
pub lock: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct audit_tty_status {
pub enabled: __u32,
pub log_passwd: __u32,
}
#[repr(C)]
#[derive(Debug)]
pub struct audit_rule_data {
pub flags: __u32,
pub action: __u32,
pub field_count: __u32,
pub mask: [__u32; 64usize],
pub fields: [__u32; 64usize],
pub values: [__u32; 64usize],
pub fieldflags: [__u32; 64usize],
pub buflen: __u32,
pub buf: __IncompleteArrayField<crate::ctypes::c_char>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sock_filter {
pub code: __u16,
pub jt: __u8,
pub jf: __u8,
pub k: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sock_fprog {
pub len: crate::ctypes::c_ushort,
pub filter: *mut sock_filter,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ptrace_peeksiginfo_args {
pub off: __u64,
pub flags: __u32,
pub nr: __s32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct seccomp_metadata {
pub filter_off: __u64,
pub flags: __u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptrace_syscall_info {
pub op: __u8,
pub pad: [__u8; 3usize],
pub arch: __u32,
pub instruction_pointer: __u64,
pub stack_pointer: __u64,
pub __bindgen_anon_1: ptrace_syscall_info__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ptrace_syscall_info__bindgen_ty_1__bindgen_ty_1 {
pub nr: __u64,
pub args: [__u64; 6usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ptrace_syscall_info__bindgen_ty_1__bindgen_ty_2 {
pub rval: __s64,
pub is_error: __u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ptrace_syscall_info__bindgen_ty_1__bindgen_ty_3 {
pub nr: __u64,
pub args: [__u64; 6usize],
pub ret_data: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ptrace_rseq_configuration {
pub rseq_abi_pointer: __u64,
pub rseq_abi_size: __u32,
pub signature: __u32,
pub flags: __u32,
pub pad: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ptrace_sud_config {
pub mode: __u64,
pub selector: __u64,
pub offset: __u64,
pub len: __u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pt_regs {
pub tls: crate::ctypes::c_ulong,
pub lr: crate::ctypes::c_ulong,
pub pc: crate::ctypes::c_ulong,
pub sr: crate::ctypes::c_ulong,
pub usp: crate::ctypes::c_ulong,
pub orig_a0: crate::ctypes::c_ulong,
pub a0: crate::ctypes::c_ulong,
pub a1: crate::ctypes::c_ulong,
pub a2: crate::ctypes::c_ulong,
pub a3: crate::ctypes::c_ulong,
pub regs: [crate::ctypes::c_ulong; 10usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct user_fp {
pub vr: [crate::ctypes::c_ulong; 96usize],
pub fcr: crate::ctypes::c_ulong,
pub fesr: crate::ctypes::c_ulong,
pub fid: crate::ctypes::c_ulong,
pub reserved: crate::ctypes::c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct seccomp_data {
pub nr: crate::ctypes::c_int,
pub arch: __u32,
pub instruction_pointer: __u64,
pub args: [__u64; 6usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct seccomp_notif_sizes {
pub seccomp_notif: __u16,
pub seccomp_notif_resp: __u16,
pub seccomp_data: __u16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct seccomp_notif {
pub id: __u64,
pub pid: __u32,
pub flags: __u32,
pub data: seccomp_data,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct seccomp_notif_resp {
pub id: __u64,
pub val: __s64,
pub error: __s32,
pub flags: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct seccomp_notif_addfd {
pub id: __u64,
pub flags: __u32,
pub srcfd: __u32,
pub newfd: __u32,
pub newfd_flags: __u32,
}
pub const __BITS_PER_LONG_LONG: u32 = 64;
pub const EM_NONE: u32 = 0;
pub const EM_M32: u32 = 1;
pub const EM_SPARC: u32 = 2;
pub const EM_386: u32 = 3;
pub const EM_68K: u32 = 4;
pub const EM_88K: u32 = 5;
pub const EM_486: u32 = 6;
pub const EM_860: u32 = 7;
pub const EM_MIPS: u32 = 8;
pub const EM_MIPS_RS3_LE: u32 = 10;
pub const EM_MIPS_RS4_BE: u32 = 10;
pub const EM_PARISC: u32 = 15;
pub const EM_SPARC32PLUS: u32 = 18;
pub const EM_PPC: u32 = 20;
pub const EM_PPC64: u32 = 21;
pub const EM_SPU: u32 = 23;
pub const EM_ARM: u32 = 40;
pub const EM_SH: u32 = 42;
pub const EM_SPARCV9: u32 = 43;
pub const EM_H8_300: u32 = 46;
pub const EM_IA_64: u32 = 50;
pub const EM_X86_64: u32 = 62;
pub const EM_S390: u32 = 22;
pub const EM_CRIS: u32 = 76;
pub const EM_M32R: u32 = 88;
pub const EM_MN10300: u32 = 89;
pub const EM_OPENRISC: u32 = 92;
pub const EM_ARCOMPACT: u32 = 93;
pub const EM_XTENSA: u32 = 94;
pub const EM_BLACKFIN: u32 = 106;
pub const EM_UNICORE: u32 = 110;
pub const EM_ALTERA_NIOS2: u32 = 113;
pub const EM_TI_C6000: u32 = 140;
pub const EM_HEXAGON: u32 = 164;
pub const EM_NDS32: u32 = 167;
pub const EM_AARCH64: u32 = 183;
pub const EM_TILEPRO: u32 = 188;
pub const EM_MICROBLAZE: u32 = 189;
pub const EM_TILEGX: u32 = 191;
pub const EM_ARCV2: u32 = 195;
pub const EM_RISCV: u32 = 243;
pub const EM_BPF: u32 = 247;
pub const EM_CSKY: u32 = 252;
pub const EM_LOONGARCH: u32 = 258;
pub const EM_FRV: u32 = 21569;
pub const EM_ALPHA: u32 = 36902;
pub const EM_CYGNUS_M32R: u32 = 36929;
pub const EM_S390_OLD: u32 = 41872;
pub const EM_CYGNUS_MN10300: u32 = 48879;
pub const AUDIT_GET: u32 = 1000;
pub const AUDIT_SET: u32 = 1001;
pub const AUDIT_LIST: u32 = 1002;
pub const AUDIT_ADD: u32 = 1003;
pub const AUDIT_DEL: u32 = 1004;
pub const AUDIT_USER: u32 = 1005;
pub const AUDIT_LOGIN: u32 = 1006;
pub const AUDIT_WATCH_INS: u32 = 1007;
pub const AUDIT_WATCH_REM: u32 = 1008;
pub const AUDIT_WATCH_LIST: u32 = 1009;
pub const AUDIT_SIGNAL_INFO: u32 = 1010;
pub const AUDIT_ADD_RULE: u32 = 1011;
pub const AUDIT_DEL_RULE: u32 = 1012;
pub const AUDIT_LIST_RULES: u32 = 1013;
pub const AUDIT_TRIM: u32 = 1014;
pub const AUDIT_MAKE_EQUIV: u32 = 1015;
pub const AUDIT_TTY_GET: u32 = 1016;
pub const AUDIT_TTY_SET: u32 = 1017;
pub const AUDIT_SET_FEATURE: u32 = 1018;
pub const AUDIT_GET_FEATURE: u32 = 1019;
pub const AUDIT_FIRST_USER_MSG: u32 = 1100;
pub const AUDIT_USER_AVC: u32 = 1107;
pub const AUDIT_USER_TTY: u32 = 1124;
pub const AUDIT_LAST_USER_MSG: u32 = 1199;
pub const AUDIT_FIRST_USER_MSG2: u32 = 2100;
pub const AUDIT_LAST_USER_MSG2: u32 = 2999;
pub const AUDIT_DAEMON_START: u32 = 1200;
pub const AUDIT_DAEMON_END: u32 = 1201;
pub const AUDIT_DAEMON_ABORT: u32 = 1202;
pub const AUDIT_DAEMON_CONFIG: u32 = 1203;
pub const AUDIT_SYSCALL: u32 = 1300;
pub const AUDIT_PATH: u32 = 1302;
pub const AUDIT_IPC: u32 = 1303;
pub const AUDIT_SOCKETCALL: u32 = 1304;
pub const AUDIT_CONFIG_CHANGE: u32 = 1305;
pub const AUDIT_SOCKADDR: u32 = 1306;
pub const AUDIT_CWD: u32 = 1307;
pub const AUDIT_EXECVE: u32 = 1309;
pub const AUDIT_IPC_SET_PERM: u32 = 1311;
pub const AUDIT_MQ_OPEN: u32 = 1312;
pub const AUDIT_MQ_SENDRECV: u32 = 1313;
pub const AUDIT_MQ_NOTIFY: u32 = 1314;
pub const AUDIT_MQ_GETSETATTR: u32 = 1315;
pub const AUDIT_KERNEL_OTHER: u32 = 1316;
pub const AUDIT_FD_PAIR: u32 = 1317;
pub const AUDIT_OBJ_PID: u32 = 1318;
pub const AUDIT_TTY: u32 = 1319;
pub const AUDIT_EOE: u32 = 1320;
pub const AUDIT_BPRM_FCAPS: u32 = 1321;
pub const AUDIT_CAPSET: u32 = 1322;
pub const AUDIT_MMAP: u32 = 1323;
pub const AUDIT_NETFILTER_PKT: u32 = 1324;
pub const AUDIT_NETFILTER_CFG: u32 = 1325;
pub const AUDIT_SECCOMP: u32 = 1326;
pub const AUDIT_PROCTITLE: u32 = 1327;
pub const AUDIT_FEATURE_CHANGE: u32 = 1328;
pub const AUDIT_REPLACE: u32 = 1329;
pub const AUDIT_KERN_MODULE: u32 = 1330;
pub const AUDIT_FANOTIFY: u32 = 1331;
pub const AUDIT_TIME_INJOFFSET: u32 = 1332;
pub const AUDIT_TIME_ADJNTPVAL: u32 = 1333;
pub const AUDIT_BPF: u32 = 1334;
pub const AUDIT_EVENT_LISTENER: u32 = 1335;
pub const AUDIT_URINGOP: u32 = 1336;
pub const AUDIT_OPENAT2: u32 = 1337;
pub const AUDIT_DM_CTRL: u32 = 1338;
pub const AUDIT_DM_EVENT: u32 = 1339;
pub const AUDIT_AVC: u32 = 1400;
pub const AUDIT_SELINUX_ERR: u32 = 1401;
pub const AUDIT_AVC_PATH: u32 = 1402;
pub const AUDIT_MAC_POLICY_LOAD: u32 = 1403;
pub const AUDIT_MAC_STATUS: u32 = 1404;
pub const AUDIT_MAC_CONFIG_CHANGE: u32 = 1405;
pub const AUDIT_MAC_UNLBL_ALLOW: u32 = 1406;
pub const AUDIT_MAC_CIPSOV4_ADD: u32 = 1407;
pub const AUDIT_MAC_CIPSOV4_DEL: u32 = 1408;
pub const AUDIT_MAC_MAP_ADD: u32 = 1409;
pub const AUDIT_MAC_MAP_DEL: u32 = 1410;
pub const AUDIT_MAC_IPSEC_ADDSA: u32 = 1411;
pub const AUDIT_MAC_IPSEC_DELSA: u32 = 1412;
pub const AUDIT_MAC_IPSEC_ADDSPD: u32 = 1413;
pub const AUDIT_MAC_IPSEC_DELSPD: u32 = 1414;
pub const AUDIT_MAC_IPSEC_EVENT: u32 = 1415;
pub const AUDIT_MAC_UNLBL_STCADD: u32 = 1416;
pub const AUDIT_MAC_UNLBL_STCDEL: u32 = 1417;
pub const AUDIT_MAC_CALIPSO_ADD: u32 = 1418;
pub const AUDIT_MAC_CALIPSO_DEL: u32 = 1419;
pub const AUDIT_IPE_ACCESS: u32 = 1420;
pub const AUDIT_IPE_CONFIG_CHANGE: u32 = 1421;
pub const AUDIT_IPE_POLICY_LOAD: u32 = 1422;
pub const AUDIT_FIRST_KERN_ANOM_MSG: u32 = 1700;
pub const AUDIT_LAST_KERN_ANOM_MSG: u32 = 1799;
pub const AUDIT_ANOM_PROMISCUOUS: u32 = 1700;
pub const AUDIT_ANOM_ABEND: u32 = 1701;
pub const AUDIT_ANOM_LINK: u32 = 1702;
pub const AUDIT_ANOM_CREAT: u32 = 1703;
pub const AUDIT_INTEGRITY_DATA: u32 = 1800;
pub const AUDIT_INTEGRITY_METADATA: u32 = 1801;
pub const AUDIT_INTEGRITY_STATUS: u32 = 1802;
pub const AUDIT_INTEGRITY_HASH: u32 = 1803;
pub const AUDIT_INTEGRITY_PCR: u32 = 1804;
pub const AUDIT_INTEGRITY_RULE: u32 = 1805;
pub const AUDIT_INTEGRITY_EVM_XATTR: u32 = 1806;
pub const AUDIT_INTEGRITY_POLICY_RULE: u32 = 1807;
pub const AUDIT_KERNEL: u32 = 2000;
pub const AUDIT_FILTER_USER: u32 = 0;
pub const AUDIT_FILTER_TASK: u32 = 1;
pub const AUDIT_FILTER_ENTRY: u32 = 2;
pub const AUDIT_FILTER_WATCH: u32 = 3;
pub const AUDIT_FILTER_EXIT: u32 = 4;
pub const AUDIT_FILTER_EXCLUDE: u32 = 5;
pub const AUDIT_FILTER_TYPE: u32 = 5;
pub const AUDIT_FILTER_FS: u32 = 6;
pub const AUDIT_FILTER_URING_EXIT: u32 = 7;
pub const AUDIT_NR_FILTERS: u32 = 8;
pub const AUDIT_FILTER_PREPEND: u32 = 16;
pub const AUDIT_NEVER: u32 = 0;
pub const AUDIT_POSSIBLE: u32 = 1;
pub const AUDIT_ALWAYS: u32 = 2;
pub const AUDIT_MAX_FIELDS: u32 = 64;
pub const AUDIT_MAX_KEY_LEN: u32 = 256;
pub const AUDIT_BITMASK_SIZE: u32 = 64;
pub const AUDIT_SYSCALL_CLASSES: u32 = 16;
pub const AUDIT_CLASS_DIR_WRITE: u32 = 0;
pub const AUDIT_CLASS_DIR_WRITE_32: u32 = 1;
pub const AUDIT_CLASS_CHATTR: u32 = 2;
pub const AUDIT_CLASS_CHATTR_32: u32 = 3;
pub const AUDIT_CLASS_READ: u32 = 4;
pub const AUDIT_CLASS_READ_32: u32 = 5;
pub const AUDIT_CLASS_WRITE: u32 = 6;
pub const AUDIT_CLASS_WRITE_32: u32 = 7;
pub const AUDIT_CLASS_SIGNAL: u32 = 8;
pub const AUDIT_CLASS_SIGNAL_32: u32 = 9;
pub const AUDIT_UNUSED_BITS: u32 = 134216704;
pub const AUDIT_COMPARE_UID_TO_OBJ_UID: u32 = 1;
pub const AUDIT_COMPARE_GID_TO_OBJ_GID: u32 = 2;
pub const AUDIT_COMPARE_EUID_TO_OBJ_UID: u32 = 3;
pub const AUDIT_COMPARE_EGID_TO_OBJ_GID: u32 = 4;
pub const AUDIT_COMPARE_AUID_TO_OBJ_UID: u32 = 5;
pub const AUDIT_COMPARE_SUID_TO_OBJ_UID: u32 = 6;
pub const AUDIT_COMPARE_SGID_TO_OBJ_GID: u32 = 7;
pub const AUDIT_COMPARE_FSUID_TO_OBJ_UID: u32 = 8;
pub const AUDIT_COMPARE_FSGID_TO_OBJ_GID: u32 = 9;
pub const AUDIT_COMPARE_UID_TO_AUID: u32 = 10;
pub const AUDIT_COMPARE_UID_TO_EUID: u32 = 11;
pub const AUDIT_COMPARE_UID_TO_FSUID: u32 = 12;
pub const AUDIT_COMPARE_UID_TO_SUID: u32 = 13;
pub const AUDIT_COMPARE_AUID_TO_FSUID: u32 = 14;
pub const AUDIT_COMPARE_AUID_TO_SUID: u32 = 15;
pub const AUDIT_COMPARE_AUID_TO_EUID: u32 = 16;
pub const AUDIT_COMPARE_EUID_TO_SUID: u32 = 17;
pub const AUDIT_COMPARE_EUID_TO_FSUID: u32 = 18;
pub const AUDIT_COMPARE_SUID_TO_FSUID: u32 = 19;
pub const AUDIT_COMPARE_GID_TO_EGID: u32 = 20;
pub const AUDIT_COMPARE_GID_TO_FSGID: u32 = 21;
pub const AUDIT_COMPARE_GID_TO_SGID: u32 = 22;
pub const AUDIT_COMPARE_EGID_TO_FSGID: u32 = 23;
pub const AUDIT_COMPARE_EGID_TO_SGID: u32 = 24;
pub const AUDIT_COMPARE_SGID_TO_FSGID: u32 = 25;
pub const AUDIT_MAX_FIELD_COMPARE: u32 = 25;
pub const AUDIT_PID: u32 = 0;
pub const AUDIT_UID: u32 = 1;
pub const AUDIT_EUID: u32 = 2;
pub const AUDIT_SUID: u32 = 3;
pub const AUDIT_FSUID: u32 = 4;
pub const AUDIT_GID: u32 = 5;
pub const AUDIT_EGID: u32 = 6;
pub const AUDIT_SGID: u32 = 7;
pub const AUDIT_FSGID: u32 = 8;
pub const AUDIT_LOGINUID: u32 = 9;
pub const AUDIT_PERS: u32 = 10;
pub const AUDIT_ARCH: u32 = 11;
pub const AUDIT_MSGTYPE: u32 = 12;
pub const AUDIT_SUBJ_USER: u32 = 13;
pub const AUDIT_SUBJ_ROLE: u32 = 14;
pub const AUDIT_SUBJ_TYPE: u32 = 15;
pub const AUDIT_SUBJ_SEN: u32 = 16;
pub const AUDIT_SUBJ_CLR: u32 = 17;
pub const AUDIT_PPID: u32 = 18;
pub const AUDIT_OBJ_USER: u32 = 19;
pub const AUDIT_OBJ_ROLE: u32 = 20;
pub const AUDIT_OBJ_TYPE: u32 = 21;
pub const AUDIT_OBJ_LEV_LOW: u32 = 22;
pub const AUDIT_OBJ_LEV_HIGH: u32 = 23;
pub const AUDIT_LOGINUID_SET: u32 = 24;
pub const AUDIT_SESSIONID: u32 = 25;
pub const AUDIT_FSTYPE: u32 = 26;
pub const AUDIT_DEVMAJOR: u32 = 100;
pub const AUDIT_DEVMINOR: u32 = 101;
pub const AUDIT_INODE: u32 = 102;
pub const AUDIT_EXIT: u32 = 103;
pub const AUDIT_SUCCESS: u32 = 104;
pub const AUDIT_WATCH: u32 = 105;
pub const AUDIT_PERM: u32 = 106;
pub const AUDIT_DIR: u32 = 107;
pub const AUDIT_FILETYPE: u32 = 108;
pub const AUDIT_OBJ_UID: u32 = 109;
pub const AUDIT_OBJ_GID: u32 = 110;
pub const AUDIT_FIELD_COMPARE: u32 = 111;
pub const AUDIT_EXE: u32 = 112;
pub const AUDIT_SADDR_FAM: u32 = 113;
pub const AUDIT_ARG0: u32 = 200;
pub const AUDIT_ARG1: u32 = 201;
pub const AUDIT_ARG2: u32 = 202;
pub const AUDIT_ARG3: u32 = 203;
pub const AUDIT_FILTERKEY: u32 = 210;
pub const AUDIT_NEGATE: u32 = 2147483648;
pub const AUDIT_BIT_MASK: u32 = 134217728;
pub const AUDIT_LESS_THAN: u32 = 268435456;
pub const AUDIT_GREATER_THAN: u32 = 536870912;
pub const AUDIT_NOT_EQUAL: u32 = 805306368;
pub const AUDIT_EQUAL: u32 = 1073741824;
pub const AUDIT_BIT_TEST: u32 = 1207959552;
pub const AUDIT_LESS_THAN_OR_EQUAL: u32 = 1342177280;
pub const AUDIT_GREATER_THAN_OR_EQUAL: u32 = 1610612736;
pub const AUDIT_OPERATORS: u32 = 2013265920;
pub const AUDIT_STATUS_ENABLED: u32 = 1;
pub const AUDIT_STATUS_FAILURE: u32 = 2;
pub const AUDIT_STATUS_PID: u32 = 4;
pub const AUDIT_STATUS_RATE_LIMIT: u32 = 8;
pub const AUDIT_STATUS_BACKLOG_LIMIT: u32 = 16;
pub const AUDIT_STATUS_BACKLOG_WAIT_TIME: u32 = 32;
pub const AUDIT_STATUS_LOST: u32 = 64;
pub const AUDIT_STATUS_BACKLOG_WAIT_TIME_ACTUAL: u32 = 128;
pub const AUDIT_FEATURE_BITMAP_BACKLOG_LIMIT: u32 = 1;
pub const AUDIT_FEATURE_BITMAP_BACKLOG_WAIT_TIME: u32 = 2;
pub const AUDIT_FEATURE_BITMAP_EXECUTABLE_PATH: u32 = 4;
pub const AUDIT_FEATURE_BITMAP_EXCLUDE_EXTEND: u32 = 8;
pub const AUDIT_FEATURE_BITMAP_SESSIONID_FILTER: u32 = 16;
pub const AUDIT_FEATURE_BITMAP_LOST_RESET: u32 = 32;
pub const AUDIT_FEATURE_BITMAP_FILTER_FS: u32 = 64;
pub const AUDIT_FEATURE_BITMAP_ALL: u32 = 127;
pub const AUDIT_VERSION_LATEST: u32 = 127;
pub const AUDIT_VERSION_BACKLOG_LIMIT: u32 = 1;
pub const AUDIT_VERSION_BACKLOG_WAIT_TIME: u32 = 2;
pub const AUDIT_FAIL_SILENT: u32 = 0;
pub const AUDIT_FAIL_PRINTK: u32 = 1;
pub const AUDIT_FAIL_PANIC: u32 = 2;
pub const __AUDIT_ARCH_CONVENTION_MASK: u32 = 805306368;
pub const __AUDIT_ARCH_CONVENTION_MIPS64_N32: u32 = 536870912;
pub const __AUDIT_ARCH_64BIT: u32 = 2147483648;
pub const __AUDIT_ARCH_LE: u32 = 1073741824;
pub const AUDIT_ARCH_AARCH64: u32 = 3221225655;
pub const AUDIT_ARCH_ALPHA: u32 = 3221262374;
pub const AUDIT_ARCH_ARCOMPACT: u32 = 1073741917;
pub const AUDIT_ARCH_ARCOMPACTBE: u32 = 93;
pub const AUDIT_ARCH_ARCV2: u32 = 1073742019;
pub const AUDIT_ARCH_ARCV2BE: u32 = 195;
pub const AUDIT_ARCH_ARM: u32 = 1073741864;
pub const AUDIT_ARCH_ARMEB: u32 = 40;
pub const AUDIT_ARCH_C6X: u32 = 1073741964;
pub const AUDIT_ARCH_C6XBE: u32 = 140;
pub const AUDIT_ARCH_CRIS: u32 = 1073741900;
pub const AUDIT_ARCH_CSKY: u32 = 1073742076;
pub const AUDIT_ARCH_FRV: u32 = 21569;
pub const AUDIT_ARCH_H8300: u32 = 46;
pub const AUDIT_ARCH_HEXAGON: u32 = 164;
pub const AUDIT_ARCH_I386: u32 = 1073741827;
pub const AUDIT_ARCH_IA64: u32 = 3221225522;
pub const AUDIT_ARCH_M32R: u32 = 88;
pub const AUDIT_ARCH_M68K: u32 = 4;
pub const AUDIT_ARCH_MICROBLAZE: u32 = 189;
pub const AUDIT_ARCH_MIPS: u32 = 8;
pub const AUDIT_ARCH_MIPSEL: u32 = 1073741832;
pub const AUDIT_ARCH_MIPS64: u32 = 2147483656;
pub const AUDIT_ARCH_MIPS64N32: u32 = 2684354568;
pub const AUDIT_ARCH_MIPSEL64: u32 = 3221225480;
pub const AUDIT_ARCH_MIPSEL64N32: u32 = 3758096392;
pub const AUDIT_ARCH_NDS32: u32 = 1073741991;
pub const AUDIT_ARCH_NDS32BE: u32 = 167;
pub const AUDIT_ARCH_NIOS2: u32 = 1073741937;
pub const AUDIT_ARCH_OPENRISC: u32 = 92;
pub const AUDIT_ARCH_PARISC: u32 = 15;
pub const AUDIT_ARCH_PARISC64: u32 = 2147483663;
pub const AUDIT_ARCH_PPC: u32 = 20;
pub const AUDIT_ARCH_PPC64: u32 = 2147483669;
pub const AUDIT_ARCH_PPC64LE: u32 = 3221225493;
pub const AUDIT_ARCH_RISCV32: u32 = 1073742067;
pub const AUDIT_ARCH_RISCV64: u32 = 3221225715;
pub const AUDIT_ARCH_S390: u32 = 22;
pub const AUDIT_ARCH_S390X: u32 = 2147483670;
pub const AUDIT_ARCH_SH: u32 = 42;
pub const AUDIT_ARCH_SHEL: u32 = 1073741866;
pub const AUDIT_ARCH_SH64: u32 = 2147483690;
pub const AUDIT_ARCH_SHEL64: u32 = 3221225514;
pub const AUDIT_ARCH_SPARC: u32 = 2;
pub const AUDIT_ARCH_SPARC64: u32 = 2147483691;
pub const AUDIT_ARCH_TILEGX: u32 = 3221225663;
pub const AUDIT_ARCH_TILEGX32: u32 = 1073742015;
pub const AUDIT_ARCH_TILEPRO: u32 = 1073742012;
pub const AUDIT_ARCH_UNICORE: u32 = 1073741934;
pub const AUDIT_ARCH_X86_64: u32 = 3221225534;
pub const AUDIT_ARCH_XTENSA: u32 = 94;
pub const AUDIT_ARCH_LOONGARCH32: u32 = 1073742082;
pub const AUDIT_ARCH_LOONGARCH64: u32 = 3221225730;
pub const AUDIT_PERM_EXEC: u32 = 1;
pub const AUDIT_PERM_WRITE: u32 = 2;
pub const AUDIT_PERM_READ: u32 = 4;
pub const AUDIT_PERM_ATTR: u32 = 8;
pub const AUDIT_MESSAGE_TEXT_MAX: u32 = 8560;
pub const AUDIT_FEATURE_VERSION: u32 = 1;
pub const AUDIT_FEATURE_ONLY_UNSET_LOGINUID: u32 = 0;
pub const AUDIT_FEATURE_LOGINUID_IMMUTABLE: u32 = 1;
pub const AUDIT_LAST_FEATURE: u32 = 1;
pub const BPF_LD: u32 = 0;
pub const BPF_LDX: u32 = 1;
pub const BPF_ST: u32 = 2;
pub const BPF_STX: u32 = 3;
pub const BPF_ALU: u32 = 4;
pub const BPF_JMP: u32 = 5;
pub const BPF_RET: u32 = 6;
pub const BPF_MISC: u32 = 7;
pub const BPF_W: u32 = 0;
pub const BPF_H: u32 = 8;
pub const BPF_B: u32 = 16;
pub const BPF_IMM: u32 = 0;
pub const BPF_ABS: u32 = 32;
pub const BPF_IND: u32 = 64;
pub const BPF_MEM: u32 = 96;
pub const BPF_LEN: u32 = 128;
pub const BPF_MSH: u32 = 160;
pub const BPF_ADD: u32 = 0;
pub const BPF_SUB: u32 = 16;
pub const BPF_MUL: u32 = 32;
pub const BPF_DIV: u32 = 48;
pub const BPF_OR: u32 = 64;
pub const BPF_AND: u32 = 80;
pub const BPF_LSH: u32 = 96;
pub const BPF_RSH: u32 = 112;
pub const BPF_NEG: u32 = 128;
pub const BPF_MOD: u32 = 144;
pub const BPF_XOR: u32 = 160;
pub const BPF_JA: u32 = 0;
pub const BPF_JEQ: u32 = 16;
pub const BPF_JGT: u32 = 32;
pub const BPF_JGE: u32 = 48;
pub const BPF_JSET: u32 = 64;
pub const BPF_K: u32 = 0;
pub const BPF_X: u32 = 8;
pub const BPF_MAXINSNS: u32 = 4096;
pub const BPF_MAJOR_VERSION: u32 = 1;
pub const BPF_MINOR_VERSION: u32 = 1;
pub const BPF_A: u32 = 16;
pub const BPF_TAX: u32 = 0;
pub const BPF_TXA: u32 = 128;
pub const BPF_MEMWORDS: u32 = 16;
pub const SKF_AD_OFF: i32 = -4096;
pub const SKF_AD_PROTOCOL: u32 = 0;
pub const SKF_AD_PKTTYPE: u32 = 4;
pub const SKF_AD_IFINDEX: u32 = 8;
pub const SKF_AD_NLATTR: u32 = 12;
pub const SKF_AD_NLATTR_NEST: u32 = 16;
pub const SKF_AD_MARK: u32 = 20;
pub const SKF_AD_QUEUE: u32 = 24;
pub const SKF_AD_HATYPE: u32 = 28;
pub const SKF_AD_RXHASH: u32 = 32;
pub const SKF_AD_CPU: u32 = 36;
pub const SKF_AD_ALU_XOR_X: u32 = 40;
pub const SKF_AD_VLAN_TAG: u32 = 44;
pub const SKF_AD_VLAN_TAG_PRESENT: u32 = 48;
pub const SKF_AD_PAY_OFFSET: u32 = 52;
pub const SKF_AD_RANDOM: u32 = 56;
pub const SKF_AD_VLAN_TPID: u32 = 60;
pub const SKF_AD_MAX: u32 = 64;
pub const SKF_NET_OFF: i32 = -1048576;
pub const SKF_LL_OFF: i32 = -2097152;
pub const BPF_NET_OFF: i32 = -1048576;
pub const BPF_LL_OFF: i32 = -2097152;
pub const PTRACE_TRACEME: u32 = 0;
pub const PTRACE_PEEKTEXT: u32 = 1;
pub const PTRACE_PEEKDATA: u32 = 2;
pub const PTRACE_PEEKUSR: u32 = 3;
pub const PTRACE_POKETEXT: u32 = 4;
pub const PTRACE_POKEDATA: u32 = 5;
pub const PTRACE_POKEUSR: u32 = 6;
pub const PTRACE_CONT: u32 = 7;
pub const PTRACE_KILL: u32 = 8;
pub const PTRACE_SINGLESTEP: u32 = 9;
pub const PTRACE_ATTACH: u32 = 16;
pub const PTRACE_DETACH: u32 = 17;
pub const PTRACE_SYSCALL: u32 = 24;
pub const PTRACE_SETOPTIONS: u32 = 16896;
pub const PTRACE_GETEVENTMSG: u32 = 16897;
pub const PTRACE_GETSIGINFO: u32 = 16898;
pub const PTRACE_SETSIGINFO: u32 = 16899;
pub const PTRACE_GETREGSET: u32 = 16900;
pub const PTRACE_SETREGSET: u32 = 16901;
pub const PTRACE_SEIZE: u32 = 16902;
pub const PTRACE_INTERRUPT: u32 = 16903;
pub const PTRACE_LISTEN: u32 = 16904;
pub const PTRACE_PEEKSIGINFO: u32 = 16905;
pub const PTRACE_GETSIGMASK: u32 = 16906;
pub const PTRACE_SETSIGMASK: u32 = 16907;
pub const PTRACE_SECCOMP_GET_FILTER: u32 = 16908;
pub const PTRACE_SECCOMP_GET_METADATA: u32 = 16909;
pub const PTRACE_GET_SYSCALL_INFO: u32 = 16910;
pub const PTRACE_SYSCALL_INFO_NONE: u32 = 0;
pub const PTRACE_SYSCALL_INFO_ENTRY: u32 = 1;
pub const PTRACE_SYSCALL_INFO_EXIT: u32 = 2;
pub const PTRACE_SYSCALL_INFO_SECCOMP: u32 = 3;
pub const PTRACE_GET_RSEQ_CONFIGURATION: u32 = 16911;
pub const PTRACE_SET_SYSCALL_USER_DISPATCH_CONFIG: u32 = 16912;
pub const PTRACE_GET_SYSCALL_USER_DISPATCH_CONFIG: u32 = 16913;
pub const PTRACE_EVENTMSG_SYSCALL_ENTRY: u32 = 1;
pub const PTRACE_EVENTMSG_SYSCALL_EXIT: u32 = 2;
pub const PTRACE_PEEKSIGINFO_SHARED: u32 = 1;
pub const PTRACE_EVENT_FORK: u32 = 1;
pub const PTRACE_EVENT_VFORK: u32 = 2;
pub const PTRACE_EVENT_CLONE: u32 = 3;
pub const PTRACE_EVENT_EXEC: u32 = 4;
pub const PTRACE_EVENT_VFORK_DONE: u32 = 5;
pub const PTRACE_EVENT_EXIT: u32 = 6;
pub const PTRACE_EVENT_SECCOMP: u32 = 7;
pub const PTRACE_EVENT_STOP: u32 = 128;
pub const PTRACE_O_TRACESYSGOOD: u32 = 1;
pub const PTRACE_O_TRACEFORK: u32 = 2;
pub const PTRACE_O_TRACEVFORK: u32 = 4;
pub const PTRACE_O_TRACECLONE: u32 = 8;
pub const PTRACE_O_TRACEEXEC: u32 = 16;
pub const PTRACE_O_TRACEVFORKDONE: u32 = 32;
pub const PTRACE_O_TRACEEXIT: u32 = 64;
pub const PTRACE_O_TRACESECCOMP: u32 = 128;
pub const PTRACE_O_EXITKILL: u32 = 1048576;
pub const PTRACE_O_SUSPEND_SECCOMP: u32 = 2097152;
pub const PTRACE_O_MASK: u32 = 3145983;
pub const SECCOMP_MODE_DISABLED: u32 = 0;
pub const SECCOMP_MODE_STRICT: u32 = 1;
pub const SECCOMP_MODE_FILTER: u32 = 2;
pub const SECCOMP_SET_MODE_STRICT: u32 = 0;
pub const SECCOMP_SET_MODE_FILTER: u32 = 1;
pub const SECCOMP_GET_ACTION_AVAIL: u32 = 2;
pub const SECCOMP_GET_NOTIF_SIZES: u32 = 3;
pub const SECCOMP_FILTER_FLAG_TSYNC: u32 = 1;
pub const SECCOMP_FILTER_FLAG_LOG: u32 = 2;
pub const SECCOMP_FILTER_FLAG_SPEC_ALLOW: u32 = 4;
pub const SECCOMP_FILTER_FLAG_NEW_LISTENER: u32 = 8;
pub const SECCOMP_FILTER_FLAG_TSYNC_ESRCH: u32 = 16;
pub const SECCOMP_FILTER_FLAG_WAIT_KILLABLE_RECV: u32 = 32;
pub const SECCOMP_RET_KILL_PROCESS: u32 = 2147483648;
pub const SECCOMP_RET_KILL_THREAD: u32 = 0;
pub const SECCOMP_RET_KILL: u32 = 0;
pub const SECCOMP_RET_TRAP: u32 = 196608;
pub const SECCOMP_RET_ERRNO: u32 = 327680;
pub const SECCOMP_RET_USER_NOTIF: u32 = 2143289344;
pub const SECCOMP_RET_TRACE: u32 = 2146435072;
pub const SECCOMP_RET_LOG: u32 = 2147221504;
pub const SECCOMP_RET_ALLOW: u32 = 2147418112;
pub const SECCOMP_RET_ACTION_FULL: u32 = 4294901760;
pub const SECCOMP_RET_ACTION: u32 = 2147418112;
pub const SECCOMP_RET_DATA: u32 = 65535;
pub const SECCOMP_USER_NOTIF_FLAG_CONTINUE: u32 = 1;
pub const SECCOMP_USER_NOTIF_FD_SYNC_WAKE_UP: u32 = 1;
pub const SECCOMP_ADDFD_FLAG_SETFD: u32 = 1;
pub const SECCOMP_ADDFD_FLAG_SEND: u32 = 2;
pub const SECCOMP_IOC_MAGIC: u8 = 33u8;
pub const Audit_equal: _bindgen_ty_1 = _bindgen_ty_1::Audit_equal;
pub const Audit_not_equal: _bindgen_ty_1 = _bindgen_ty_1::Audit_not_equal;
pub const Audit_bitmask: _bindgen_ty_1 = _bindgen_ty_1::Audit_bitmask;
pub const Audit_bittest: _bindgen_ty_1 = _bindgen_ty_1::Audit_bittest;
pub const Audit_lt: _bindgen_ty_1 = _bindgen_ty_1::Audit_lt;
pub const Audit_gt: _bindgen_ty_1 = _bindgen_ty_1::Audit_gt;
pub const Audit_le: _bindgen_ty_1 = _bindgen_ty_1::Audit_le;
pub const Audit_ge: _bindgen_ty_1 = _bindgen_ty_1::Audit_ge;
pub const Audit_bad: _bindgen_ty_1 = _bindgen_ty_1::Audit_bad;
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_1 {
Audit_equal = 0,
Audit_not_equal = 1,
Audit_bitmask = 2,
Audit_bittest = 3,
Audit_lt = 4,
Audit_gt = 5,
Audit_le = 6,
Audit_ge = 7,
Audit_bad = 8,
}
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum audit_nlgrps {
AUDIT_NLGRP_NONE = 0,
AUDIT_NLGRP_READLOG = 1,
__AUDIT_NLGRP_MAX = 2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union audit_status__bindgen_ty_1 {
pub version: __u32,
pub feature_bitmap: __u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ptrace_syscall_info__bindgen_ty_1 {
pub entry: ptrace_syscall_info__bindgen_ty_1__bindgen_ty_1,
pub exit: ptrace_syscall_info__bindgen_ty_1__bindgen_ty_2,
pub seccomp: ptrace_syscall_info__bindgen_ty_1__bindgen_ty_3,
}
impl<T> __IncompleteArrayField<T> {
#[inline]
pub const fn new() -> Self {
__IncompleteArrayField(::core::marker::PhantomData, [])
}
#[inline]
pub fn as_ptr(&self) -> *const T {
self as *const _ as *const T
}
#[inline]
pub fn as_mut_ptr(&mut self) -> *mut T {
self as *mut _ as *mut T
}
#[inline]
pub unsafe fn as_slice(&self, len: usize) -> &[T] {
::core::slice::from_raw_parts(self.as_ptr(), len)
}
#[inline]
pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
::core::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
}
}
impl<T> ::core::fmt::Debug for __IncompleteArrayField<T> {
fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
fmt.write_str("__IncompleteArrayField")
}
}
