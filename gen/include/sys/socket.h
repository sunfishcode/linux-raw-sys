#ifndef _SYS_SOCKET_H
#define _SYS_SOCKET_H

#include <linux/socket.h>

struct sockaddr {
    struct __kernel_sockaddr_storage __storage;
};

#endif
