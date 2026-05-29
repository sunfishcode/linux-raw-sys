// ALSA/sound userspace API headers.

#include "support.h"

// Ensure the kernel-style include path (linux/types.h, asm/byteorder.h)
// is used instead of the userspace path (endian.h, sys/ioctl.h), which
// may not resolve under -nostdinc on all architectures.
#ifndef __linux__
#define __linux__
#endif

#include <sound/asound.h>
