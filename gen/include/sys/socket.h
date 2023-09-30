// Define `sockaddr` ourselves because the uapi headers don't define it but
// some use it.
struct sockaddr {
    struct __kernel_sockaddr_storage __storage;
};
