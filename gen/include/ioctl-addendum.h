// Additional definitions to add to the ioctl.h module.

// On PowerPC, the kernel does not define a `struct termios2` or its associated
// ioctls, and the regular `termios` has the extra `termios2` fields.
#if defined(__powerpc__) || defined(__powerpc64__)
#define TCGETS2 TCGETS
#define TCSETS2 TCSETS
#define TCSETSF2 TCSETSF
#define TCSETSW2 TCSETSW
#endif

// Ioctls from headers removed in recent kernels. Preserved for backward
// compatibility.
//
// reiserfs_fs.h (removed), cm4000_cs.h (removed), meye.h (removed)
#define REISERFS_IOC_UNPACK _IOW(0xCD, 1, long)
#define CM_IOCGATR _IOWR('c', 1, unsigned long long)
#define CM_IOSDBGLVL _IOW('c', 250, unsigned long long)
#define MEYEIOC_SYNC _IOWR('v', 195, int)
