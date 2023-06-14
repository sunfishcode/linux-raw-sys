// Additional definitions to add to the ioctl.h module.

// On PowerPC, the kernel does not define a `struct termios2` or its associated
// ioctls, and the regular `termios` has the extra `termios2` fields.
#if defined(__powerpc__) || defined(__powerpc64__)
#define TCGETS2 TCGETS
#define TCSETS2 TCSETS
#define TCSETSF2 TCSETSF
#define TCSETSW2 TCSETSW
#endif
