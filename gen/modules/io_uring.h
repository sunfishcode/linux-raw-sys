// This file includes selected Linux header files, and supplementary
// definitions, needed for io_uring code.

#include "support.h"

// Selected Linux headers.

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
