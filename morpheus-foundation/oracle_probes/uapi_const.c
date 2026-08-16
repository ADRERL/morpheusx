#define _GNU_SOURCE 1
#include <stdio.h>
#include <errno.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <dirent.h>
#include <time.h>
#include <sys/wait.h>
#include <sys/mman.h>
#include <signal.h>
#include <poll.h>
#include <unistd.h>
#if __has_include(<sys/epoll.h>)
#include <sys/epoll.h>
#endif
#if __has_include(<sys/socket.h>)
#include <sys/socket.h>
#endif
#if __has_include(<netinet/in.h>)
#include <netinet/in.h>
#endif
#if __has_include(<netinet/tcp.h>)
#include <netinet/tcp.h>
#endif
#if __has_include(<sys/ioctl.h>)
#include <sys/ioctl.h>
#endif
#if __has_include(<linux/futex.h>)
#include <linux/futex.h>
#endif
#if __has_include(<sys/random.h>)
#include <sys/random.h>
#endif

/* dumps the host libc's values of the abi macros morpheus-foundation mirrors,
 * as rust consts on stdout; build.rs captures it for tests/uapi_const.rs.
 * macros absent on this host print None so the test skips them instead of
 * false-passing. */

int main(void) {
    printf("pub const PROBE_AVAILABLE: bool = true;\n");
#ifdef EPERM
    printf("pub const P_EPERM: Option<i128> = Some(%lld);\n", (long long)(EPERM));
#else
    printf("pub const P_EPERM: Option<i128> = None;\n");
#endif
#ifdef ENOENT
    printf("pub const P_ENOENT: Option<i128> = Some(%lld);\n", (long long)(ENOENT));
#else
    printf("pub const P_ENOENT: Option<i128> = None;\n");
#endif
#ifdef ESRCH
    printf("pub const P_ESRCH: Option<i128> = Some(%lld);\n", (long long)(ESRCH));
#else
    printf("pub const P_ESRCH: Option<i128> = None;\n");
#endif
#ifdef EINTR
    printf("pub const P_EINTR: Option<i128> = Some(%lld);\n", (long long)(EINTR));
#else
    printf("pub const P_EINTR: Option<i128> = None;\n");
#endif
#ifdef EIO
    printf("pub const P_EIO: Option<i128> = Some(%lld);\n", (long long)(EIO));
#else
    printf("pub const P_EIO: Option<i128> = None;\n");
#endif
#ifdef ENXIO
    printf("pub const P_ENXIO: Option<i128> = Some(%lld);\n", (long long)(ENXIO));
#else
    printf("pub const P_ENXIO: Option<i128> = None;\n");
#endif
#ifdef E2BIG
    printf("pub const P_E2BIG: Option<i128> = Some(%lld);\n", (long long)(E2BIG));
#else
    printf("pub const P_E2BIG: Option<i128> = None;\n");
#endif
#ifdef ENOEXEC
    printf("pub const P_ENOEXEC: Option<i128> = Some(%lld);\n", (long long)(ENOEXEC));
#else
    printf("pub const P_ENOEXEC: Option<i128> = None;\n");
#endif
#ifdef EBADF
    printf("pub const P_EBADF: Option<i128> = Some(%lld);\n", (long long)(EBADF));
#else
    printf("pub const P_EBADF: Option<i128> = None;\n");
#endif
#ifdef ECHILD
    printf("pub const P_ECHILD: Option<i128> = Some(%lld);\n", (long long)(ECHILD));
#else
    printf("pub const P_ECHILD: Option<i128> = None;\n");
#endif
#ifdef EAGAIN
    printf("pub const P_EAGAIN: Option<i128> = Some(%lld);\n", (long long)(EAGAIN));
#else
    printf("pub const P_EAGAIN: Option<i128> = None;\n");
#endif
#ifdef ENOMEM
    printf("pub const P_ENOMEM: Option<i128> = Some(%lld);\n", (long long)(ENOMEM));
#else
    printf("pub const P_ENOMEM: Option<i128> = None;\n");
#endif
#ifdef EACCES
    printf("pub const P_EACCES: Option<i128> = Some(%lld);\n", (long long)(EACCES));
#else
    printf("pub const P_EACCES: Option<i128> = None;\n");
#endif
#ifdef EFAULT
    printf("pub const P_EFAULT: Option<i128> = Some(%lld);\n", (long long)(EFAULT));
#else
    printf("pub const P_EFAULT: Option<i128> = None;\n");
#endif
#ifdef EBUSY
    printf("pub const P_EBUSY: Option<i128> = Some(%lld);\n", (long long)(EBUSY));
#else
    printf("pub const P_EBUSY: Option<i128> = None;\n");
#endif
#ifdef EEXIST
    printf("pub const P_EEXIST: Option<i128> = Some(%lld);\n", (long long)(EEXIST));
#else
    printf("pub const P_EEXIST: Option<i128> = None;\n");
#endif
#ifdef EXDEV
    printf("pub const P_EXDEV: Option<i128> = Some(%lld);\n", (long long)(EXDEV));
#else
    printf("pub const P_EXDEV: Option<i128> = None;\n");
#endif
#ifdef ENODEV
    printf("pub const P_ENODEV: Option<i128> = Some(%lld);\n", (long long)(ENODEV));
#else
    printf("pub const P_ENODEV: Option<i128> = None;\n");
#endif
#ifdef ENOTDIR
    printf("pub const P_ENOTDIR: Option<i128> = Some(%lld);\n", (long long)(ENOTDIR));
#else
    printf("pub const P_ENOTDIR: Option<i128> = None;\n");
#endif
#ifdef EISDIR
    printf("pub const P_EISDIR: Option<i128> = Some(%lld);\n", (long long)(EISDIR));
#else
    printf("pub const P_EISDIR: Option<i128> = None;\n");
#endif
#ifdef EINVAL
    printf("pub const P_EINVAL: Option<i128> = Some(%lld);\n", (long long)(EINVAL));
#else
    printf("pub const P_EINVAL: Option<i128> = None;\n");
#endif
#ifdef ENFILE
    printf("pub const P_ENFILE: Option<i128> = Some(%lld);\n", (long long)(ENFILE));
#else
    printf("pub const P_ENFILE: Option<i128> = None;\n");
#endif
#ifdef EMFILE
    printf("pub const P_EMFILE: Option<i128> = Some(%lld);\n", (long long)(EMFILE));
#else
    printf("pub const P_EMFILE: Option<i128> = None;\n");
#endif
#ifdef ENOTTY
    printf("pub const P_ENOTTY: Option<i128> = Some(%lld);\n", (long long)(ENOTTY));
#else
    printf("pub const P_ENOTTY: Option<i128> = None;\n");
#endif
#ifdef ETXTBSY
    printf("pub const P_ETXTBSY: Option<i128> = Some(%lld);\n", (long long)(ETXTBSY));
#else
    printf("pub const P_ETXTBSY: Option<i128> = None;\n");
#endif
#ifdef EFBIG
    printf("pub const P_EFBIG: Option<i128> = Some(%lld);\n", (long long)(EFBIG));
#else
    printf("pub const P_EFBIG: Option<i128> = None;\n");
#endif
#ifdef ENOSPC
    printf("pub const P_ENOSPC: Option<i128> = Some(%lld);\n", (long long)(ENOSPC));
#else
    printf("pub const P_ENOSPC: Option<i128> = None;\n");
#endif
#ifdef ESPIPE
    printf("pub const P_ESPIPE: Option<i128> = Some(%lld);\n", (long long)(ESPIPE));
#else
    printf("pub const P_ESPIPE: Option<i128> = None;\n");
#endif
#ifdef EROFS
    printf("pub const P_EROFS: Option<i128> = Some(%lld);\n", (long long)(EROFS));
#else
    printf("pub const P_EROFS: Option<i128> = None;\n");
#endif
#ifdef EMLINK
    printf("pub const P_EMLINK: Option<i128> = Some(%lld);\n", (long long)(EMLINK));
#else
    printf("pub const P_EMLINK: Option<i128> = None;\n");
#endif
#ifdef EPIPE
    printf("pub const P_EPIPE: Option<i128> = Some(%lld);\n", (long long)(EPIPE));
#else
    printf("pub const P_EPIPE: Option<i128> = None;\n");
#endif
#ifdef EDOM
    printf("pub const P_EDOM: Option<i128> = Some(%lld);\n", (long long)(EDOM));
#else
    printf("pub const P_EDOM: Option<i128> = None;\n");
#endif
#ifdef ERANGE
    printf("pub const P_ERANGE: Option<i128> = Some(%lld);\n", (long long)(ERANGE));
#else
    printf("pub const P_ERANGE: Option<i128> = None;\n");
#endif
#ifdef EDEADLK
    printf("pub const P_EDEADLK: Option<i128> = Some(%lld);\n", (long long)(EDEADLK));
#else
    printf("pub const P_EDEADLK: Option<i128> = None;\n");
#endif
#ifdef ENAMETOOLONG
    printf("pub const P_ENAMETOOLONG: Option<i128> = Some(%lld);\n", (long long)(ENAMETOOLONG));
#else
    printf("pub const P_ENAMETOOLONG: Option<i128> = None;\n");
#endif
#ifdef ENOLCK
    printf("pub const P_ENOLCK: Option<i128> = Some(%lld);\n", (long long)(ENOLCK));
#else
    printf("pub const P_ENOLCK: Option<i128> = None;\n");
#endif
#ifdef ENOSYS
    printf("pub const P_ENOSYS: Option<i128> = Some(%lld);\n", (long long)(ENOSYS));
#else
    printf("pub const P_ENOSYS: Option<i128> = None;\n");
#endif
#ifdef ENOTEMPTY
    printf("pub const P_ENOTEMPTY: Option<i128> = Some(%lld);\n", (long long)(ENOTEMPTY));
#else
    printf("pub const P_ENOTEMPTY: Option<i128> = None;\n");
#endif
#ifdef ELOOP
    printf("pub const P_ELOOP: Option<i128> = Some(%lld);\n", (long long)(ELOOP));
#else
    printf("pub const P_ELOOP: Option<i128> = None;\n");
#endif
#ifdef ENOMSG
    printf("pub const P_ENOMSG: Option<i128> = Some(%lld);\n", (long long)(ENOMSG));
#else
    printf("pub const P_ENOMSG: Option<i128> = None;\n");
#endif
#ifdef EPROTO
    printf("pub const P_EPROTO: Option<i128> = Some(%lld);\n", (long long)(EPROTO));
#else
    printf("pub const P_EPROTO: Option<i128> = None;\n");
#endif
#ifdef EOVERFLOW
    printf("pub const P_EOVERFLOW: Option<i128> = Some(%lld);\n", (long long)(EOVERFLOW));
#else
    printf("pub const P_EOVERFLOW: Option<i128> = None;\n");
#endif
#ifdef ENOTSOCK
    printf("pub const P_ENOTSOCK: Option<i128> = Some(%lld);\n", (long long)(ENOTSOCK));
#else
    printf("pub const P_ENOTSOCK: Option<i128> = None;\n");
#endif
#ifdef EDESTADDRREQ
    printf("pub const P_EDESTADDRREQ: Option<i128> = Some(%lld);\n", (long long)(EDESTADDRREQ));
#else
    printf("pub const P_EDESTADDRREQ: Option<i128> = None;\n");
#endif
#ifdef EMSGSIZE
    printf("pub const P_EMSGSIZE: Option<i128> = Some(%lld);\n", (long long)(EMSGSIZE));
#else
    printf("pub const P_EMSGSIZE: Option<i128> = None;\n");
#endif
#ifdef EPROTOTYPE
    printf("pub const P_EPROTOTYPE: Option<i128> = Some(%lld);\n", (long long)(EPROTOTYPE));
#else
    printf("pub const P_EPROTOTYPE: Option<i128> = None;\n");
#endif
#ifdef ENOPROTOOPT
    printf("pub const P_ENOPROTOOPT: Option<i128> = Some(%lld);\n", (long long)(ENOPROTOOPT));
#else
    printf("pub const P_ENOPROTOOPT: Option<i128> = None;\n");
#endif
#ifdef EPROTONOSUPPORT
    printf("pub const P_EPROTONOSUPPORT: Option<i128> = Some(%lld);\n", (long long)(EPROTONOSUPPORT));
#else
    printf("pub const P_EPROTONOSUPPORT: Option<i128> = None;\n");
#endif
#ifdef ESOCKTNOSUPPORT
    printf("pub const P_ESOCKTNOSUPPORT: Option<i128> = Some(%lld);\n", (long long)(ESOCKTNOSUPPORT));
#else
    printf("pub const P_ESOCKTNOSUPPORT: Option<i128> = None;\n");
#endif
#ifdef EOPNOTSUPP
    printf("pub const P_EOPNOTSUPP: Option<i128> = Some(%lld);\n", (long long)(EOPNOTSUPP));
#else
    printf("pub const P_EOPNOTSUPP: Option<i128> = None;\n");
#endif
#ifdef EPFNOSUPPORT
    printf("pub const P_EPFNOSUPPORT: Option<i128> = Some(%lld);\n", (long long)(EPFNOSUPPORT));
#else
    printf("pub const P_EPFNOSUPPORT: Option<i128> = None;\n");
#endif
#ifdef EAFNOSUPPORT
    printf("pub const P_EAFNOSUPPORT: Option<i128> = Some(%lld);\n", (long long)(EAFNOSUPPORT));
#else
    printf("pub const P_EAFNOSUPPORT: Option<i128> = None;\n");
#endif
#ifdef EADDRINUSE
    printf("pub const P_EADDRINUSE: Option<i128> = Some(%lld);\n", (long long)(EADDRINUSE));
#else
    printf("pub const P_EADDRINUSE: Option<i128> = None;\n");
#endif
#ifdef EADDRNOTAVAIL
    printf("pub const P_EADDRNOTAVAIL: Option<i128> = Some(%lld);\n", (long long)(EADDRNOTAVAIL));
#else
    printf("pub const P_EADDRNOTAVAIL: Option<i128> = None;\n");
#endif
#ifdef ENETDOWN
    printf("pub const P_ENETDOWN: Option<i128> = Some(%lld);\n", (long long)(ENETDOWN));
#else
    printf("pub const P_ENETDOWN: Option<i128> = None;\n");
#endif
#ifdef ENETUNREACH
    printf("pub const P_ENETUNREACH: Option<i128> = Some(%lld);\n", (long long)(ENETUNREACH));
#else
    printf("pub const P_ENETUNREACH: Option<i128> = None;\n");
#endif
#ifdef ENETRESET
    printf("pub const P_ENETRESET: Option<i128> = Some(%lld);\n", (long long)(ENETRESET));
#else
    printf("pub const P_ENETRESET: Option<i128> = None;\n");
#endif
#ifdef ECONNABORTED
    printf("pub const P_ECONNABORTED: Option<i128> = Some(%lld);\n", (long long)(ECONNABORTED));
#else
    printf("pub const P_ECONNABORTED: Option<i128> = None;\n");
#endif
#ifdef ECONNRESET
    printf("pub const P_ECONNRESET: Option<i128> = Some(%lld);\n", (long long)(ECONNRESET));
#else
    printf("pub const P_ECONNRESET: Option<i128> = None;\n");
#endif
#ifdef ENOBUFS
    printf("pub const P_ENOBUFS: Option<i128> = Some(%lld);\n", (long long)(ENOBUFS));
#else
    printf("pub const P_ENOBUFS: Option<i128> = None;\n");
#endif
#ifdef EISCONN
    printf("pub const P_EISCONN: Option<i128> = Some(%lld);\n", (long long)(EISCONN));
#else
    printf("pub const P_EISCONN: Option<i128> = None;\n");
#endif
#ifdef ENOTCONN
    printf("pub const P_ENOTCONN: Option<i128> = Some(%lld);\n", (long long)(ENOTCONN));
#else
    printf("pub const P_ENOTCONN: Option<i128> = None;\n");
#endif
#ifdef ESHUTDOWN
    printf("pub const P_ESHUTDOWN: Option<i128> = Some(%lld);\n", (long long)(ESHUTDOWN));
#else
    printf("pub const P_ESHUTDOWN: Option<i128> = None;\n");
#endif
#ifdef ETIMEDOUT
    printf("pub const P_ETIMEDOUT: Option<i128> = Some(%lld);\n", (long long)(ETIMEDOUT));
#else
    printf("pub const P_ETIMEDOUT: Option<i128> = None;\n");
#endif
#ifdef ECONNREFUSED
    printf("pub const P_ECONNREFUSED: Option<i128> = Some(%lld);\n", (long long)(ECONNREFUSED));
#else
    printf("pub const P_ECONNREFUSED: Option<i128> = None;\n");
#endif
#ifdef EHOSTDOWN
    printf("pub const P_EHOSTDOWN: Option<i128> = Some(%lld);\n", (long long)(EHOSTDOWN));
#else
    printf("pub const P_EHOSTDOWN: Option<i128> = None;\n");
#endif
#ifdef EHOSTUNREACH
    printf("pub const P_EHOSTUNREACH: Option<i128> = Some(%lld);\n", (long long)(EHOSTUNREACH));
#else
    printf("pub const P_EHOSTUNREACH: Option<i128> = None;\n");
#endif
#ifdef EALREADY
    printf("pub const P_EALREADY: Option<i128> = Some(%lld);\n", (long long)(EALREADY));
#else
    printf("pub const P_EALREADY: Option<i128> = None;\n");
#endif
#ifdef EINPROGRESS
    printf("pub const P_EINPROGRESS: Option<i128> = Some(%lld);\n", (long long)(EINPROGRESS));
#else
    printf("pub const P_EINPROGRESS: Option<i128> = None;\n");
#endif
#ifdef ESTALE
    printf("pub const P_ESTALE: Option<i128> = Some(%lld);\n", (long long)(ESTALE));
#else
    printf("pub const P_ESTALE: Option<i128> = None;\n");
#endif
#ifdef EDQUOT
    printf("pub const P_EDQUOT: Option<i128> = Some(%lld);\n", (long long)(EDQUOT));
#else
    printf("pub const P_EDQUOT: Option<i128> = None;\n");
#endif
#ifdef ECANCELED
    printf("pub const P_ECANCELED: Option<i128> = Some(%lld);\n", (long long)(ECANCELED));
#else
    printf("pub const P_ECANCELED: Option<i128> = None;\n");
#endif
#ifdef O_NONBLOCK
    printf("pub const P_O_NONBLOCK: Option<i128> = Some(%lld);\n", (long long)(O_NONBLOCK));
#else
    printf("pub const P_O_NONBLOCK: Option<i128> = None;\n");
#endif
#ifdef O_CLOEXEC
    printf("pub const P_O_CLOEXEC: Option<i128> = Some(%lld);\n", (long long)(O_CLOEXEC));
#else
    printf("pub const P_O_CLOEXEC: Option<i128> = None;\n");
#endif
#ifdef S_IFMT
    printf("pub const P_S_IFMT: Option<i128> = Some(%lld);\n", (long long)(S_IFMT));
#else
    printf("pub const P_S_IFMT: Option<i128> = None;\n");
#endif
#ifdef S_IFSOCK
    printf("pub const P_S_IFSOCK: Option<i128> = Some(%lld);\n", (long long)(S_IFSOCK));
#else
    printf("pub const P_S_IFSOCK: Option<i128> = None;\n");
#endif
#ifdef S_IFLNK
    printf("pub const P_S_IFLNK: Option<i128> = Some(%lld);\n", (long long)(S_IFLNK));
#else
    printf("pub const P_S_IFLNK: Option<i128> = None;\n");
#endif
#ifdef S_IFREG
    printf("pub const P_S_IFREG: Option<i128> = Some(%lld);\n", (long long)(S_IFREG));
#else
    printf("pub const P_S_IFREG: Option<i128> = None;\n");
#endif
#ifdef S_IFBLK
    printf("pub const P_S_IFBLK: Option<i128> = Some(%lld);\n", (long long)(S_IFBLK));
#else
    printf("pub const P_S_IFBLK: Option<i128> = None;\n");
#endif
#ifdef S_IFDIR
    printf("pub const P_S_IFDIR: Option<i128> = Some(%lld);\n", (long long)(S_IFDIR));
#else
    printf("pub const P_S_IFDIR: Option<i128> = None;\n");
#endif
#ifdef S_IFCHR
    printf("pub const P_S_IFCHR: Option<i128> = Some(%lld);\n", (long long)(S_IFCHR));
#else
    printf("pub const P_S_IFCHR: Option<i128> = None;\n");
#endif
#ifdef S_IFIFO
    printf("pub const P_S_IFIFO: Option<i128> = Some(%lld);\n", (long long)(S_IFIFO));
#else
    printf("pub const P_S_IFIFO: Option<i128> = None;\n");
#endif
#ifdef DT_UNKNOWN
    printf("pub const P_DT_UNKNOWN: Option<i128> = Some(%lld);\n", (long long)(DT_UNKNOWN));
#else
    printf("pub const P_DT_UNKNOWN: Option<i128> = None;\n");
#endif
#ifdef DT_FIFO
    printf("pub const P_DT_FIFO: Option<i128> = Some(%lld);\n", (long long)(DT_FIFO));
#else
    printf("pub const P_DT_FIFO: Option<i128> = None;\n");
#endif
#ifdef DT_CHR
    printf("pub const P_DT_CHR: Option<i128> = Some(%lld);\n", (long long)(DT_CHR));
#else
    printf("pub const P_DT_CHR: Option<i128> = None;\n");
#endif
#ifdef DT_DIR
    printf("pub const P_DT_DIR: Option<i128> = Some(%lld);\n", (long long)(DT_DIR));
#else
    printf("pub const P_DT_DIR: Option<i128> = None;\n");
#endif
#ifdef DT_BLK
    printf("pub const P_DT_BLK: Option<i128> = Some(%lld);\n", (long long)(DT_BLK));
#else
    printf("pub const P_DT_BLK: Option<i128> = None;\n");
#endif
#ifdef DT_REG
    printf("pub const P_DT_REG: Option<i128> = Some(%lld);\n", (long long)(DT_REG));
#else
    printf("pub const P_DT_REG: Option<i128> = None;\n");
#endif
#ifdef DT_LNK
    printf("pub const P_DT_LNK: Option<i128> = Some(%lld);\n", (long long)(DT_LNK));
#else
    printf("pub const P_DT_LNK: Option<i128> = None;\n");
#endif
#ifdef DT_SOCK
    printf("pub const P_DT_SOCK: Option<i128> = Some(%lld);\n", (long long)(DT_SOCK));
#else
    printf("pub const P_DT_SOCK: Option<i128> = None;\n");
#endif
#ifdef CLOCK_REALTIME
    printf("pub const P_CLOCK_REALTIME: Option<i128> = Some(%lld);\n", (long long)(CLOCK_REALTIME));
#else
    printf("pub const P_CLOCK_REALTIME: Option<i128> = None;\n");
#endif
#ifdef CLOCK_MONOTONIC
    printf("pub const P_CLOCK_MONOTONIC: Option<i128> = Some(%lld);\n", (long long)(CLOCK_MONOTONIC));
#else
    printf("pub const P_CLOCK_MONOTONIC: Option<i128> = None;\n");
#endif
#ifdef WNOHANG
    printf("pub const P_WNOHANG: Option<i128> = Some(%lld);\n", (long long)(WNOHANG));
#else
    printf("pub const P_WNOHANG: Option<i128> = None;\n");
#endif
#ifdef WUNTRACED
    printf("pub const P_WUNTRACED: Option<i128> = Some(%lld);\n", (long long)(WUNTRACED));
#else
    printf("pub const P_WUNTRACED: Option<i128> = None;\n");
#endif
#ifdef WCONTINUED
    printf("pub const P_WCONTINUED: Option<i128> = Some(%lld);\n", (long long)(WCONTINUED));
#else
    printf("pub const P_WCONTINUED: Option<i128> = None;\n");
#endif
#ifdef F_DUPFD
    printf("pub const P_F_DUPFD: Option<i128> = Some(%lld);\n", (long long)(F_DUPFD));
#else
    printf("pub const P_F_DUPFD: Option<i128> = None;\n");
#endif
#ifdef F_GETFD
    printf("pub const P_F_GETFD: Option<i128> = Some(%lld);\n", (long long)(F_GETFD));
#else
    printf("pub const P_F_GETFD: Option<i128> = None;\n");
#endif
#ifdef F_SETFD
    printf("pub const P_F_SETFD: Option<i128> = Some(%lld);\n", (long long)(F_SETFD));
#else
    printf("pub const P_F_SETFD: Option<i128> = None;\n");
#endif
#ifdef F_GETFL
    printf("pub const P_F_GETFL: Option<i128> = Some(%lld);\n", (long long)(F_GETFL));
#else
    printf("pub const P_F_GETFL: Option<i128> = None;\n");
#endif
#ifdef F_SETFL
    printf("pub const P_F_SETFL: Option<i128> = Some(%lld);\n", (long long)(F_SETFL));
#else
    printf("pub const P_F_SETFL: Option<i128> = None;\n");
#endif
#ifdef F_DUPFD_CLOEXEC
    printf("pub const P_F_DUPFD_CLOEXEC: Option<i128> = Some(%lld);\n", (long long)(F_DUPFD_CLOEXEC));
#else
    printf("pub const P_F_DUPFD_CLOEXEC: Option<i128> = None;\n");
#endif
#ifdef FD_CLOEXEC
    printf("pub const P_FD_CLOEXEC: Option<i128> = Some(%lld);\n", (long long)(FD_CLOEXEC));
#else
    printf("pub const P_FD_CLOEXEC: Option<i128> = None;\n");
#endif
#ifdef EPOLL_CLOEXEC
    printf("pub const P_EPOLL_CLOEXEC: Option<i128> = Some(%lld);\n", (long long)(EPOLL_CLOEXEC));
#else
    printf("pub const P_EPOLL_CLOEXEC: Option<i128> = None;\n");
#endif
#ifdef EPOLL_CTL_ADD
    printf("pub const P_EPOLL_CTL_ADD: Option<i128> = Some(%lld);\n", (long long)(EPOLL_CTL_ADD));
#else
    printf("pub const P_EPOLL_CTL_ADD: Option<i128> = None;\n");
#endif
#ifdef EPOLL_CTL_DEL
    printf("pub const P_EPOLL_CTL_DEL: Option<i128> = Some(%lld);\n", (long long)(EPOLL_CTL_DEL));
#else
    printf("pub const P_EPOLL_CTL_DEL: Option<i128> = None;\n");
#endif
#ifdef EPOLL_CTL_MOD
    printf("pub const P_EPOLL_CTL_MOD: Option<i128> = Some(%lld);\n", (long long)(EPOLL_CTL_MOD));
#else
    printf("pub const P_EPOLL_CTL_MOD: Option<i128> = None;\n");
#endif
#ifdef EPOLLIN
    printf("pub const P_EPOLLIN: Option<i128> = Some(%lld);\n", (long long)(EPOLLIN));
#else
    printf("pub const P_EPOLLIN: Option<i128> = None;\n");
#endif
#ifdef EPOLLPRI
    printf("pub const P_EPOLLPRI: Option<i128> = Some(%lld);\n", (long long)(EPOLLPRI));
#else
    printf("pub const P_EPOLLPRI: Option<i128> = None;\n");
#endif
#ifdef EPOLLOUT
    printf("pub const P_EPOLLOUT: Option<i128> = Some(%lld);\n", (long long)(EPOLLOUT));
#else
    printf("pub const P_EPOLLOUT: Option<i128> = None;\n");
#endif
#ifdef EPOLLERR
    printf("pub const P_EPOLLERR: Option<i128> = Some(%lld);\n", (long long)(EPOLLERR));
#else
    printf("pub const P_EPOLLERR: Option<i128> = None;\n");
#endif
#ifdef EPOLLHUP
    printf("pub const P_EPOLLHUP: Option<i128> = Some(%lld);\n", (long long)(EPOLLHUP));
#else
    printf("pub const P_EPOLLHUP: Option<i128> = None;\n");
#endif
#ifdef EPOLLRDHUP
    printf("pub const P_EPOLLRDHUP: Option<i128> = Some(%lld);\n", (long long)(EPOLLRDHUP));
#else
    printf("pub const P_EPOLLRDHUP: Option<i128> = None;\n");
#endif
#ifdef EPOLLONESHOT
    printf("pub const P_EPOLLONESHOT: Option<i128> = Some(%lld);\n", (long long)(EPOLLONESHOT));
#else
    printf("pub const P_EPOLLONESHOT: Option<i128> = None;\n");
#endif
#ifdef EPOLLET
    printf("pub const P_EPOLLET: Option<i128> = Some(%lld);\n", (long long)(EPOLLET));
#else
    printf("pub const P_EPOLLET: Option<i128> = None;\n");
#endif
#ifdef AT_SYMLINK_NOFOLLOW
    printf("pub const P_AT_SYMLINK_NOFOLLOW: Option<i128> = Some(%lld);\n", (long long)(AT_SYMLINK_NOFOLLOW));
#else
    printf("pub const P_AT_SYMLINK_NOFOLLOW: Option<i128> = None;\n");
#endif
#ifdef MAP_SHARED
    printf("pub const P_MAP_SHARED: Option<i128> = Some(%lld);\n", (long long)(MAP_SHARED));
#else
    printf("pub const P_MAP_SHARED: Option<i128> = None;\n");
#endif
#ifdef MAP_PRIVATE
    printf("pub const P_MAP_PRIVATE: Option<i128> = Some(%lld);\n", (long long)(MAP_PRIVATE));
#else
    printf("pub const P_MAP_PRIVATE: Option<i128> = None;\n");
#endif
#ifdef MAP_FIXED
    printf("pub const P_MAP_FIXED: Option<i128> = Some(%lld);\n", (long long)(MAP_FIXED));
#else
    printf("pub const P_MAP_FIXED: Option<i128> = None;\n");
#endif
#ifdef MAP_ANONYMOUS
    printf("pub const P_MAP_ANONYMOUS: Option<i128> = Some(%lld);\n", (long long)(MAP_ANONYMOUS));
#else
    printf("pub const P_MAP_ANONYMOUS: Option<i128> = None;\n");
#endif
#ifdef FUTEX_WAIT
    printf("pub const P_FUTEX_WAIT: Option<i128> = Some(%lld);\n", (long long)(FUTEX_WAIT));
#else
    printf("pub const P_FUTEX_WAIT: Option<i128> = None;\n");
#endif
#ifdef FUTEX_WAKE
    printf("pub const P_FUTEX_WAKE: Option<i128> = Some(%lld);\n", (long long)(FUTEX_WAKE));
#else
    printf("pub const P_FUTEX_WAKE: Option<i128> = None;\n");
#endif
#ifdef FUTEX_WAIT_BITSET
    printf("pub const P_FUTEX_WAIT_BITSET: Option<i128> = Some(%lld);\n", (long long)(FUTEX_WAIT_BITSET));
#else
    printf("pub const P_FUTEX_WAIT_BITSET: Option<i128> = None;\n");
#endif
#ifdef FUTEX_WAKE_BITSET
    printf("pub const P_FUTEX_WAKE_BITSET: Option<i128> = Some(%lld);\n", (long long)(FUTEX_WAKE_BITSET));
#else
    printf("pub const P_FUTEX_WAKE_BITSET: Option<i128> = None;\n");
#endif
#ifdef FIONREAD
    printf("pub const P_FIONREAD: Option<i128> = Some(%lld);\n", (long long)(FIONREAD));
#else
    printf("pub const P_FIONREAD: Option<i128> = None;\n");
#endif
#ifdef FIONBIO
    printf("pub const P_FIONBIO: Option<i128> = Some(%lld);\n", (long long)(FIONBIO));
#else
    printf("pub const P_FIONBIO: Option<i128> = None;\n");
#endif
#ifdef TIOCGWINSZ
    printf("pub const P_TIOCGWINSZ: Option<i128> = Some(%lld);\n", (long long)(TIOCGWINSZ));
#else
    printf("pub const P_TIOCGWINSZ: Option<i128> = None;\n");
#endif
#ifdef POLLIN
    printf("pub const P_POLLIN: Option<i128> = Some(%lld);\n", (long long)(POLLIN));
#else
    printf("pub const P_POLLIN: Option<i128> = None;\n");
#endif
#ifdef POLLPRI
    printf("pub const P_POLLPRI: Option<i128> = Some(%lld);\n", (long long)(POLLPRI));
#else
    printf("pub const P_POLLPRI: Option<i128> = None;\n");
#endif
#ifdef POLLOUT
    printf("pub const P_POLLOUT: Option<i128> = Some(%lld);\n", (long long)(POLLOUT));
#else
    printf("pub const P_POLLOUT: Option<i128> = None;\n");
#endif
#ifdef POLLERR
    printf("pub const P_POLLERR: Option<i128> = Some(%lld);\n", (long long)(POLLERR));
#else
    printf("pub const P_POLLERR: Option<i128> = None;\n");
#endif
#ifdef POLLHUP
    printf("pub const P_POLLHUP: Option<i128> = Some(%lld);\n", (long long)(POLLHUP));
#else
    printf("pub const P_POLLHUP: Option<i128> = None;\n");
#endif
#ifdef POLLNVAL
    printf("pub const P_POLLNVAL: Option<i128> = Some(%lld);\n", (long long)(POLLNVAL));
#else
    printf("pub const P_POLLNVAL: Option<i128> = None;\n");
#endif
#ifdef POLLRDHUP
    printf("pub const P_POLLRDHUP: Option<i128> = Some(%lld);\n", (long long)(POLLRDHUP));
#else
    printf("pub const P_POLLRDHUP: Option<i128> = None;\n");
#endif
#ifdef SIGINT
    printf("pub const P_SIGINT: Option<i128> = Some(%lld);\n", (long long)(SIGINT));
#else
    printf("pub const P_SIGINT: Option<i128> = None;\n");
#endif
#ifdef SIGKILL
    printf("pub const P_SIGKILL: Option<i128> = Some(%lld);\n", (long long)(SIGKILL));
#else
    printf("pub const P_SIGKILL: Option<i128> = None;\n");
#endif
#ifdef SIGSEGV
    printf("pub const P_SIGSEGV: Option<i128> = Some(%lld);\n", (long long)(SIGSEGV));
#else
    printf("pub const P_SIGSEGV: Option<i128> = None;\n");
#endif
#ifdef SIGTERM
    printf("pub const P_SIGTERM: Option<i128> = Some(%lld);\n", (long long)(SIGTERM));
#else
    printf("pub const P_SIGTERM: Option<i128> = None;\n");
#endif
#ifdef SIGCHLD
    printf("pub const P_SIGCHLD: Option<i128> = Some(%lld);\n", (long long)(SIGCHLD));
#else
    printf("pub const P_SIGCHLD: Option<i128> = None;\n");
#endif
#ifdef SIGCONT
    printf("pub const P_SIGCONT: Option<i128> = Some(%lld);\n", (long long)(SIGCONT));
#else
    printf("pub const P_SIGCONT: Option<i128> = None;\n");
#endif
#ifdef SIGSTOP
    printf("pub const P_SIGSTOP: Option<i128> = Some(%lld);\n", (long long)(SIGSTOP));
#else
    printf("pub const P_SIGSTOP: Option<i128> = None;\n");
#endif
#ifdef AF_UNSPEC
    printf("pub const P_AF_UNSPEC: Option<i128> = Some(%lld);\n", (long long)(AF_UNSPEC));
#else
    printf("pub const P_AF_UNSPEC: Option<i128> = None;\n");
#endif
#ifdef AF_INET
    printf("pub const P_AF_INET: Option<i128> = Some(%lld);\n", (long long)(AF_INET));
#else
    printf("pub const P_AF_INET: Option<i128> = None;\n");
#endif
#ifdef AF_INET6
    printf("pub const P_AF_INET6: Option<i128> = Some(%lld);\n", (long long)(AF_INET6));
#else
    printf("pub const P_AF_INET6: Option<i128> = None;\n");
#endif
#ifdef SOCK_STREAM
    printf("pub const P_SOCK_STREAM: Option<i128> = Some(%lld);\n", (long long)(SOCK_STREAM));
#else
    printf("pub const P_SOCK_STREAM: Option<i128> = None;\n");
#endif
#ifdef SOCK_DGRAM
    printf("pub const P_SOCK_DGRAM: Option<i128> = Some(%lld);\n", (long long)(SOCK_DGRAM));
#else
    printf("pub const P_SOCK_DGRAM: Option<i128> = None;\n");
#endif
#ifdef SOCK_NONBLOCK
    printf("pub const P_SOCK_NONBLOCK: Option<i128> = Some(%lld);\n", (long long)(SOCK_NONBLOCK));
#else
    printf("pub const P_SOCK_NONBLOCK: Option<i128> = None;\n");
#endif
#ifdef SOCK_CLOEXEC
    printf("pub const P_SOCK_CLOEXEC: Option<i128> = Some(%lld);\n", (long long)(SOCK_CLOEXEC));
#else
    printf("pub const P_SOCK_CLOEXEC: Option<i128> = None;\n");
#endif
#ifdef IPPROTO_IP
    printf("pub const P_IPPROTO_IP: Option<i128> = Some(%lld);\n", (long long)(IPPROTO_IP));
#else
    printf("pub const P_IPPROTO_IP: Option<i128> = None;\n");
#endif
#ifdef IPPROTO_TCP
    printf("pub const P_IPPROTO_TCP: Option<i128> = Some(%lld);\n", (long long)(IPPROTO_TCP));
#else
    printf("pub const P_IPPROTO_TCP: Option<i128> = None;\n");
#endif
#ifdef IPPROTO_UDP
    printf("pub const P_IPPROTO_UDP: Option<i128> = Some(%lld);\n", (long long)(IPPROTO_UDP));
#else
    printf("pub const P_IPPROTO_UDP: Option<i128> = None;\n");
#endif
#ifdef IPPROTO_IPV6
    printf("pub const P_IPPROTO_IPV6: Option<i128> = Some(%lld);\n", (long long)(IPPROTO_IPV6));
#else
    printf("pub const P_IPPROTO_IPV6: Option<i128> = None;\n");
#endif
#ifdef SOL_SOCKET
    printf("pub const P_SOL_SOCKET: Option<i128> = Some(%lld);\n", (long long)(SOL_SOCKET));
#else
    printf("pub const P_SOL_SOCKET: Option<i128> = None;\n");
#endif
#ifdef SO_REUSEADDR
    printf("pub const P_SO_REUSEADDR: Option<i128> = Some(%lld);\n", (long long)(SO_REUSEADDR));
#else
    printf("pub const P_SO_REUSEADDR: Option<i128> = None;\n");
#endif
#ifdef SO_ERROR
    printf("pub const P_SO_ERROR: Option<i128> = Some(%lld);\n", (long long)(SO_ERROR));
#else
    printf("pub const P_SO_ERROR: Option<i128> = None;\n");
#endif
#ifdef SO_BROADCAST
    printf("pub const P_SO_BROADCAST: Option<i128> = Some(%lld);\n", (long long)(SO_BROADCAST));
#else
    printf("pub const P_SO_BROADCAST: Option<i128> = None;\n");
#endif
#ifdef SO_SNDBUF
    printf("pub const P_SO_SNDBUF: Option<i128> = Some(%lld);\n", (long long)(SO_SNDBUF));
#else
    printf("pub const P_SO_SNDBUF: Option<i128> = None;\n");
#endif
#ifdef SO_RCVBUF
    printf("pub const P_SO_RCVBUF: Option<i128> = Some(%lld);\n", (long long)(SO_RCVBUF));
#else
    printf("pub const P_SO_RCVBUF: Option<i128> = None;\n");
#endif
#ifdef SO_KEEPALIVE
    printf("pub const P_SO_KEEPALIVE: Option<i128> = Some(%lld);\n", (long long)(SO_KEEPALIVE));
#else
    printf("pub const P_SO_KEEPALIVE: Option<i128> = None;\n");
#endif
#ifdef SO_LINGER
    printf("pub const P_SO_LINGER: Option<i128> = Some(%lld);\n", (long long)(SO_LINGER));
#else
    printf("pub const P_SO_LINGER: Option<i128> = None;\n");
#endif
#ifdef SO_REUSEPORT
    printf("pub const P_SO_REUSEPORT: Option<i128> = Some(%lld);\n", (long long)(SO_REUSEPORT));
#else
    printf("pub const P_SO_REUSEPORT: Option<i128> = None;\n");
#endif
#ifdef SO_RCVTIMEO
    printf("pub const P_SO_RCVTIMEO: Option<i128> = Some(%lld);\n", (long long)(SO_RCVTIMEO));
#else
    printf("pub const P_SO_RCVTIMEO: Option<i128> = None;\n");
#endif
#ifdef SO_SNDTIMEO
    printf("pub const P_SO_SNDTIMEO: Option<i128> = Some(%lld);\n", (long long)(SO_SNDTIMEO));
#else
    printf("pub const P_SO_SNDTIMEO: Option<i128> = None;\n");
#endif
#ifdef TCP_NODELAY
    printf("pub const P_TCP_NODELAY: Option<i128> = Some(%lld);\n", (long long)(TCP_NODELAY));
#else
    printf("pub const P_TCP_NODELAY: Option<i128> = None;\n");
#endif
#ifdef TCP_KEEPIDLE
    printf("pub const P_TCP_KEEPIDLE: Option<i128> = Some(%lld);\n", (long long)(TCP_KEEPIDLE));
#else
    printf("pub const P_TCP_KEEPIDLE: Option<i128> = None;\n");
#endif
#ifdef TCP_KEEPINTVL
    printf("pub const P_TCP_KEEPINTVL: Option<i128> = Some(%lld);\n", (long long)(TCP_KEEPINTVL));
#else
    printf("pub const P_TCP_KEEPINTVL: Option<i128> = None;\n");
#endif
#ifdef TCP_KEEPCNT
    printf("pub const P_TCP_KEEPCNT: Option<i128> = Some(%lld);\n", (long long)(TCP_KEEPCNT));
#else
    printf("pub const P_TCP_KEEPCNT: Option<i128> = None;\n");
#endif
#ifdef IP_TTL
    printf("pub const P_IP_TTL: Option<i128> = Some(%lld);\n", (long long)(IP_TTL));
#else
    printf("pub const P_IP_TTL: Option<i128> = None;\n");
#endif
#ifdef IP_MULTICAST_TTL
    printf("pub const P_IP_MULTICAST_TTL: Option<i128> = Some(%lld);\n", (long long)(IP_MULTICAST_TTL));
#else
    printf("pub const P_IP_MULTICAST_TTL: Option<i128> = None;\n");
#endif
#ifdef IP_MULTICAST_LOOP
    printf("pub const P_IP_MULTICAST_LOOP: Option<i128> = Some(%lld);\n", (long long)(IP_MULTICAST_LOOP));
#else
    printf("pub const P_IP_MULTICAST_LOOP: Option<i128> = None;\n");
#endif
#ifdef IP_ADD_MEMBERSHIP
    printf("pub const P_IP_ADD_MEMBERSHIP: Option<i128> = Some(%lld);\n", (long long)(IP_ADD_MEMBERSHIP));
#else
    printf("pub const P_IP_ADD_MEMBERSHIP: Option<i128> = None;\n");
#endif
#ifdef IP_DROP_MEMBERSHIP
    printf("pub const P_IP_DROP_MEMBERSHIP: Option<i128> = Some(%lld);\n", (long long)(IP_DROP_MEMBERSHIP));
#else
    printf("pub const P_IP_DROP_MEMBERSHIP: Option<i128> = None;\n");
#endif
#ifdef IPV6_MULTICAST_LOOP
    printf("pub const P_IPV6_MULTICAST_LOOP: Option<i128> = Some(%lld);\n", (long long)(IPV6_MULTICAST_LOOP));
#else
    printf("pub const P_IPV6_MULTICAST_LOOP: Option<i128> = None;\n");
#endif
#ifdef IPV6_ADD_MEMBERSHIP
    printf("pub const P_IPV6_ADD_MEMBERSHIP: Option<i128> = Some(%lld);\n", (long long)(IPV6_ADD_MEMBERSHIP));
#else
    printf("pub const P_IPV6_ADD_MEMBERSHIP: Option<i128> = None;\n");
#endif
#ifdef IPV6_DROP_MEMBERSHIP
    printf("pub const P_IPV6_DROP_MEMBERSHIP: Option<i128> = Some(%lld);\n", (long long)(IPV6_DROP_MEMBERSHIP));
#else
    printf("pub const P_IPV6_DROP_MEMBERSHIP: Option<i128> = None;\n");
#endif
#ifdef IPV6_V6ONLY
    printf("pub const P_IPV6_V6ONLY: Option<i128> = Some(%lld);\n", (long long)(IPV6_V6ONLY));
#else
    printf("pub const P_IPV6_V6ONLY: Option<i128> = None;\n");
#endif
#ifdef SHUT_RD
    printf("pub const P_SHUT_RD: Option<i128> = Some(%lld);\n", (long long)(SHUT_RD));
#else
    printf("pub const P_SHUT_RD: Option<i128> = None;\n");
#endif
#ifdef SHUT_WR
    printf("pub const P_SHUT_WR: Option<i128> = Some(%lld);\n", (long long)(SHUT_WR));
#else
    printf("pub const P_SHUT_WR: Option<i128> = None;\n");
#endif
#ifdef SHUT_RDWR
    printf("pub const P_SHUT_RDWR: Option<i128> = Some(%lld);\n", (long long)(SHUT_RDWR));
#else
    printf("pub const P_SHUT_RDWR: Option<i128> = None;\n");
#endif
#ifdef MSG_PEEK
    printf("pub const P_MSG_PEEK: Option<i128> = Some(%lld);\n", (long long)(MSG_PEEK));
#else
    printf("pub const P_MSG_PEEK: Option<i128> = None;\n");
#endif
#ifdef MSG_DONTWAIT
    printf("pub const P_MSG_DONTWAIT: Option<i128> = Some(%lld);\n", (long long)(MSG_DONTWAIT));
#else
    printf("pub const P_MSG_DONTWAIT: Option<i128> = None;\n");
#endif
#ifdef MSG_NOSIGNAL
    printf("pub const P_MSG_NOSIGNAL: Option<i128> = Some(%lld);\n", (long long)(MSG_NOSIGNAL));
#else
    printf("pub const P_MSG_NOSIGNAL: Option<i128> = None;\n");
#endif
#ifdef SEEK_SET
    printf("pub const P_SEEK_SET: Option<i128> = Some(%lld);\n", (long long)(SEEK_SET));
#else
    printf("pub const P_SEEK_SET: Option<i128> = None;\n");
#endif
#ifdef SEEK_CUR
    printf("pub const P_SEEK_CUR: Option<i128> = Some(%lld);\n", (long long)(SEEK_CUR));
#else
    printf("pub const P_SEEK_CUR: Option<i128> = None;\n");
#endif
#ifdef SEEK_END
    printf("pub const P_SEEK_END: Option<i128> = Some(%lld);\n", (long long)(SEEK_END));
#else
    printf("pub const P_SEEK_END: Option<i128> = None;\n");
#endif
#ifdef GRND_NONBLOCK
    printf("pub const P_GRND_NONBLOCK: Option<i128> = Some(%lld);\n", (long long)(GRND_NONBLOCK));
#else
    printf("pub const P_GRND_NONBLOCK: Option<i128> = None;\n");
#endif
#ifdef PROT_READ
    printf("pub const P_PROT_READ: Option<i128> = Some(%lld);\n", (long long)(PROT_READ));
#else
    printf("pub const P_PROT_READ: Option<i128> = None;\n");
#endif
#ifdef PROT_WRITE
    printf("pub const P_PROT_WRITE: Option<i128> = Some(%lld);\n", (long long)(PROT_WRITE));
#else
    printf("pub const P_PROT_WRITE: Option<i128> = None;\n");
#endif
#ifdef PROT_EXEC
    printf("pub const P_PROT_EXEC: Option<i128> = Some(%lld);\n", (long long)(PROT_EXEC));
#else
    printf("pub const P_PROT_EXEC: Option<i128> = None;\n");
#endif
#ifdef PROT_NONE
    printf("pub const P_PROT_NONE: Option<i128> = Some(%lld);\n", (long long)(PROT_NONE));
#else
    printf("pub const P_PROT_NONE: Option<i128> = None;\n");
#endif
#ifdef O_CREAT
    printf("pub const P_O_CREAT: Option<i128> = Some(%lld);\n", (long long)(O_CREAT));
#else
    printf("pub const P_O_CREAT: Option<i128> = None;\n");
#endif
#ifdef O_EXCL
    printf("pub const P_O_EXCL: Option<i128> = Some(%lld);\n", (long long)(O_EXCL));
#else
    printf("pub const P_O_EXCL: Option<i128> = None;\n");
#endif
#ifdef O_TRUNC
    printf("pub const P_O_TRUNC: Option<i128> = Some(%lld);\n", (long long)(O_TRUNC));
#else
    printf("pub const P_O_TRUNC: Option<i128> = None;\n");
#endif
#ifdef O_APPEND
    printf("pub const P_O_APPEND: Option<i128> = Some(%lld);\n", (long long)(O_APPEND));
#else
    printf("pub const P_O_APPEND: Option<i128> = None;\n");
#endif
    return 0;
}
