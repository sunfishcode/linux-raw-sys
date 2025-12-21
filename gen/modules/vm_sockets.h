// Code needed for vm_sockets.h

#include "support.h"

#if LINUX_VERSION_CODE >= KERNEL_VERSION(5,6,0)
#define sa_family_t uint16_t
#include <linux/socket.h>
#include <linux/vm_sockets.h>
#endif

