//! minixrs, aarch64: definitions for musl-minixrs (musl 1.2.6).
//!
//! musl-minixrs leaves musl's aarch64 type layouts and constants untouched,
//! so everything here is the Linux-musl-aarch64 definition, copied from
//! `src/unix/linux_like/`. Nothing here is derived by hand from a header.
//!
//! Every `pub type`, `pub struct` and `pub const` in this module is checked
//! against the C headers by `verify/check-libc-abi.sh` in the minixrs tooling
//! repo. Adding an item here without adding it to that manifest fails the gate.

use crate::prelude::*;

pub type blkcnt_t = i64;
pub type blksize_t = c_int;
pub type cc_t = c_uchar;
pub type clock_t = c_long;
pub type dev_t = u64;
pub type fsblkcnt_t = u64;
pub type fsfilcnt_t = u64;
pub type ino_t = u64;
pub type mode_t = u32;
pub type nfds_t = c_ulong;
pub type nlink_t = u32;
pub type off_t = i64;
pub type pthread_key_t = c_uint;
pub type pthread_t = *mut c_void;
pub type rlim_t = c_ulonglong;
pub type sa_family_t = u16;
pub type socklen_t = u32;
pub type speed_t = c_uint;
pub type suseconds_t = i64;
pub type tcflag_t = c_uint;
pub type time_t = i64;
pub type wchar_t = u32;

pub const FD_SETSIZE: usize = 1024;
pub const NCCS: usize = 32;

s! {
    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [c_char; 14],
    }

    pub struct addrinfo {
        pub ai_flags: c_int,
        pub ai_family: c_int,
        pub ai_socktype: c_int,
        pub ai_protocol: c_int,
        pub ai_addrlen: socklen_t,
        pub ai_addr: *mut sockaddr,
        pub ai_canonname: *mut c_char,
        pub ai_next: *mut addrinfo,
    }

    pub struct fd_set {
        fds_bits: [c_ulong; FD_SETSIZE / 64],
    }

    pub struct tm {
        pub tm_sec: c_int,
        pub tm_min: c_int,
        pub tm_hour: c_int,
        pub tm_mday: c_int,
        pub tm_mon: c_int,
        pub tm_year: c_int,
        pub tm_wday: c_int,
        pub tm_yday: c_int,
        pub tm_isdst: c_int,
        pub tm_gmtoff: c_long,
        pub tm_zone: *const c_char,
    }

    pub struct Dl_info {
        pub dli_fname: *const c_char,
        pub dli_fbase: *mut c_void,
        pub dli_sname: *const c_char,
        pub dli_saddr: *mut c_void,
    }

    pub struct lconv {
        pub decimal_point: *mut c_char,
        pub thousands_sep: *mut c_char,
        pub grouping: *mut c_char,
        pub int_curr_symbol: *mut c_char,
        pub currency_symbol: *mut c_char,
        pub mon_decimal_point: *mut c_char,
        pub mon_thousands_sep: *mut c_char,
        pub mon_grouping: *mut c_char,
        pub positive_sign: *mut c_char,
        pub negative_sign: *mut c_char,
        pub int_frac_digits: c_char,
        pub frac_digits: c_char,
        pub p_cs_precedes: c_char,
        pub p_sep_by_space: c_char,
        pub n_cs_precedes: c_char,
        pub n_sep_by_space: c_char,
        pub p_sign_posn: c_char,
        pub n_sign_posn: c_char,
        pub int_p_cs_precedes: c_char,
        pub int_p_sep_by_space: c_char,
        pub int_n_cs_precedes: c_char,
        pub int_n_sep_by_space: c_char,
        pub int_p_sign_posn: c_char,
        pub int_n_sign_posn: c_char,
    }

    pub struct passwd {
        pub pw_name: *mut c_char,
        pub pw_passwd: *mut c_char,
        pub pw_uid: crate::uid_t,
        pub pw_gid: crate::gid_t,
        pub pw_gecos: *mut c_char,
        pub pw_dir: *mut c_char,
        pub pw_shell: *mut c_char,
    }

    pub struct dirent {
        pub d_ino: ino_t,
        pub d_off: off_t,
        pub d_reclen: c_ushort,
        pub d_type: c_uchar,
        pub d_name: [c_char; 256],
    }

    pub struct stat {
        pub st_dev: dev_t,
        pub st_ino: ino_t,
        pub st_mode: mode_t,
        pub st_nlink: nlink_t,
        pub st_uid: crate::uid_t,
        pub st_gid: crate::gid_t,
        pub st_rdev: dev_t,
        __pad0: Padding<c_ulong>,
        pub st_size: off_t,
        pub st_blksize: blksize_t,
        __pad1: Padding<c_int>,
        pub st_blocks: blkcnt_t,
        pub st_atime: time_t,
        pub st_atime_nsec: c_long,
        pub st_mtime: time_t,
        pub st_mtime_nsec: c_long,
        pub st_ctime: time_t,
        pub st_ctime_nsec: c_long,
        __unused: Padding<[c_uint; 2]>,
    }

    pub struct statvfs {
        pub f_bsize: c_ulong,
        pub f_frsize: c_ulong,
        pub f_blocks: fsblkcnt_t,
        pub f_bfree: fsblkcnt_t,
        pub f_bavail: fsblkcnt_t,
        pub f_files: fsfilcnt_t,
        pub f_ffree: fsfilcnt_t,
        pub f_favail: fsfilcnt_t,
        pub f_fsid: c_ulong,
        pub f_flag: c_ulong,
        pub f_namemax: c_ulong,
        __f_reserved: Padding<[c_int; 6]>,
    }

    pub struct termios {
        pub c_iflag: tcflag_t,
        pub c_oflag: tcflag_t,
        pub c_cflag: tcflag_t,
        pub c_lflag: tcflag_t,
        pub c_line: cc_t,
        pub c_cc: [cc_t; NCCS],
        pub __c_ispeed: speed_t,
        pub __c_ospeed: speed_t,
    }

    pub struct sigset_t {
        __val: [c_ulong; 16],
    }

    // FIXME(1.0): This should not implement `PartialEq`
    #[allow(unpredictable_function_pointer_comparisons)]
    pub struct sigaction {
        pub sa_sigaction: crate::sighandler_t,
        pub sa_mask: sigset_t,
        pub sa_flags: c_int,
        pub sa_restorer: Option<extern "C" fn()>,
    }

    pub struct sem_t {
        __val: [c_int; 8],
    }

    // musl declares each pthread object as a union of an int array and a
    // pointer-width array; one u64 array has the same size and alignment.
    pub struct pthread_attr_t {
        __size: [u64; 7],
    }

    pub struct pthread_mutex_t {
        __size: [u64; 5],
    }

    pub struct pthread_cond_t {
        __size: [u64; 6],
    }

    pub struct pthread_rwlock_t {
        __size: [u64; 7],
    }

    pub struct pthread_mutexattr_t {
        __attr: c_uint,
    }

    pub struct pthread_condattr_t {
        __attr: c_uint,
    }

    pub struct pthread_rwlockattr_t {
        __attr: [c_uint; 2],
    }
}

// --- What std needs, added for P4b. Copied from the same Linux-musl-aarch64
// definitions as the items above; values as the libc crate has them there.

pub type Ioctl = c_int;
pub type clockid_t = c_int;

// errno
pub const E2BIG: c_int = 7;
pub const EACCES: c_int = 13;
pub const EADDRINUSE: c_int = 98;
pub const EADDRNOTAVAIL: c_int = 99;
pub const EAGAIN: c_int = 11;
pub const EBADF: c_int = 9;
pub const EBUSY: c_int = 16;
pub const ECONNABORTED: c_int = 103;
pub const ECONNREFUSED: c_int = 111;
pub const ECONNRESET: c_int = 104;
pub const EDEADLK: c_int = 35;
pub const EDQUOT: c_int = 122;
pub const EEXIST: c_int = 17;
pub const EFBIG: c_int = 27;
pub const EHOSTUNREACH: c_int = 113;
pub const EINPROGRESS: c_int = 115;
pub const EINTR: c_int = 4;
pub const EINVAL: c_int = 22;
pub const EIO: c_int = 5;
pub const EISCONN: c_int = 106;
pub const EISDIR: c_int = 21;
pub const ELOOP: c_int = 40;
pub const EMFILE: c_int = 24;
pub const EMLINK: c_int = 31;
pub const EMSGSIZE: c_int = 90;
pub const ENAMETOOLONG: c_int = 36;
pub const ENETDOWN: c_int = 100;
pub const ENETUNREACH: c_int = 101;
pub const ENFILE: c_int = 23;
pub const ENOENT: c_int = 2;
pub const ENOMEM: c_int = 12;
pub const ENOSPC: c_int = 28;
pub const ENOSYS: c_int = 38;
pub const ENOTCONN: c_int = 107;
pub const ENOTDIR: c_int = 20;
pub const ENOTEMPTY: c_int = 39;
pub const ENOTSUP: c_int = EOPNOTSUPP;
pub const EOPNOTSUPP: c_int = 95;
pub const EPERM: c_int = 1;
pub const EPIPE: c_int = 32;
pub const ERANGE: c_int = 34;
pub const EROFS: c_int = 30;
pub const ESPIPE: c_int = 29;
pub const ESTALE: c_int = 116;
pub const ETIMEDOUT: c_int = 110;
pub const ETXTBSY: c_int = 26;
pub const EWOULDBLOCK: c_int = EAGAIN;
pub const EXDEV: c_int = 18;

// fcntl, open, *at
pub const AT_FDCWD: c_int = -100;
pub const AT_REMOVEDIR: c_int = 0x200;
pub const AT_SYMLINK_NOFOLLOW: c_int = 0x100;
pub const FIOCLEX: Ioctl = 0x5451;
pub const FIONBIO: Ioctl = 0x5421;
pub const F_DUPFD_CLOEXEC: c_int = 1030;
pub const F_GETFD: c_int = 1;
pub const F_GETFL: c_int = 3;
pub const F_SETFL: c_int = 4;
pub const O_ACCMODE: c_int = 0o10000003;
pub const O_APPEND: c_int = 1024;
pub const O_CLOEXEC: c_int = 0x80000;
pub const O_CREAT: c_int = 64;
pub const O_DIRECTORY: c_int = 0x4000;
pub const O_EXCL: c_int = 128;
pub const O_NOFOLLOW: c_int = 0x8000;
pub const O_NONBLOCK: c_int = 2048;
pub const O_RDONLY: c_int = 0;
pub const O_RDWR: c_int = 2;
pub const O_TRUNC: c_int = 512;
pub const O_WRONLY: c_int = 1;
pub const UTIME_OMIT: c_long = 1073741822;

// stat mode bits
pub const S_IFBLK: mode_t = 0o6_0000;
pub const S_IFCHR: mode_t = 0o2_0000;
pub const S_IFDIR: mode_t = 0o4_0000;
pub const S_IFIFO: mode_t = 0o1_0000;
pub const S_IFLNK: mode_t = 0o12_0000;
pub const S_IFMT: mode_t = 0o17_0000;
pub const S_IFREG: mode_t = 0o10_0000;
pub const S_IFSOCK: mode_t = 0o14_0000;
pub const S_IRGRP: mode_t = 0o0040;
pub const S_IROTH: mode_t = 0o0004;
pub const S_IRUSR: mode_t = 0o0400;
pub const S_IWGRP: mode_t = 0o0020;
pub const S_IWOTH: mode_t = 0o0002;
pub const S_IWUSR: mode_t = 0o0200;
pub const S_IXGRP: mode_t = 0o0010;
pub const S_IXOTH: mode_t = 0o0001;
pub const S_IXUSR: mode_t = 0o0100;

// lseek
pub const SEEK_CUR: c_int = 1;
pub const SEEK_END: c_int = 2;
pub const SEEK_SET: c_int = 0;

// signals
pub const SIGABRT: c_int = 6;
pub const SIGALRM: c_int = 14;
pub const SIGBUS: c_int = 7;
pub const SIGCHLD: c_int = 17;
pub const SIGCONT: c_int = 18;
pub const SIGFPE: c_int = 8;
pub const SIGHUP: c_int = 1;
pub const SIGILL: c_int = 4;
pub const SIGINT: c_int = 2;
pub const SIGIO: c_int = 29;
pub const SIGKILL: c_int = 9;
pub const SIGPIPE: c_int = 13;
pub const SIGPROF: c_int = 27;
pub const SIGQUIT: c_int = 3;
pub const SIGSEGV: c_int = 11;
pub const SIGSTOP: c_int = 19;
pub const SIGSYS: c_int = 31;
pub const SIGTERM: c_int = 15;
pub const SIGTRAP: c_int = 5;
pub const SIGTSTP: c_int = 20;
pub const SIGTTIN: c_int = 21;
pub const SIGTTOU: c_int = 22;
pub const SIGURG: c_int = 23;
pub const SIGUSR1: c_int = 10;
pub const SIGUSR2: c_int = 12;
pub const SIGVTALRM: c_int = 26;
pub const SIGWINCH: c_int = 28;
pub const SIGXCPU: c_int = 24;
pub const SIGXFSZ: c_int = 25;

// wait, exit
pub const EXIT_FAILURE: c_int = 1;
pub const EXIT_SUCCESS: c_int = 0;
pub const WNOHANG: c_int = 0x00000001;

// flock
pub const LOCK_EX: c_int = 2;
pub const LOCK_NB: c_int = 4;
pub const LOCK_SH: c_int = 1;
pub const LOCK_UN: c_int = 8;

// sockets
pub const AF_INET: c_int = 2;
pub const AF_INET6: c_int = 10;
pub const AF_UNIX: c_int = 1;
pub const EAI_SYSTEM: c_int = -11;
pub const IPV6_ADD_MEMBERSHIP: c_int = 20;
pub const IPV6_DROP_MEMBERSHIP: c_int = 21;
pub const IPV6_MULTICAST_LOOP: c_int = 19;
pub const IPV6_V6ONLY: c_int = 26;
pub const IP_ADD_MEMBERSHIP: c_int = 35;
pub const IP_DROP_MEMBERSHIP: c_int = 36;
pub const IP_MULTICAST_LOOP: c_int = 34;
pub const IP_MULTICAST_TTL: c_int = 33;
pub const IP_TTL: c_int = 2;
pub const MSG_NOSIGNAL: c_int = 0x4000;
pub const MSG_PEEK: c_int = 2;
pub const SHUT_RD: c_int = 0;
pub const SHUT_RDWR: c_int = 2;
pub const SHUT_WR: c_int = 1;
pub const SOCK_CLOEXEC: c_int = O_CLOEXEC;
pub const SOCK_DGRAM: c_int = 2;
pub const SOCK_STREAM: c_int = 1;
pub const SOL_SOCKET: c_int = 1;
pub const SOMAXCONN: c_int = 128;
pub const SO_BROADCAST: c_int = 6;
pub const SO_ERROR: c_int = 4;
pub const SO_KEEPALIVE: c_int = 9;
pub const SO_LINGER: c_int = 13;
pub const SO_RCVTIMEO: c_int = 20;
pub const SO_REUSEADDR: c_int = 2;
pub const SO_SNDTIMEO: c_int = 21;
pub const TCP_NODELAY: c_int = 1;

// poll
pub const POLLERR: c_short = 0x8;
pub const POLLHUP: c_short = 0x10;
pub const POLLIN: c_short = 0x1;
pub const POLLNVAL: c_short = 0x20;
pub const POLLOUT: c_short = 0x4;

// clocks
pub const CLOCK_MONOTONIC: crate::clockid_t = 1;
pub const CLOCK_REALTIME: crate::clockid_t = 0;
pub const TIMER_ABSTIME: c_int = 1;

// pthread
pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t { __size: [0; 6] };
pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t { __size: [0; 5] };
pub const PTHREAD_MUTEX_NORMAL: c_int = 0;
pub const PTHREAD_STACK_MIN: size_t = 2048;

// sysconf
pub const _SC_GETPW_R_SIZE_MAX: c_int = 70;
pub const _SC_HOST_NAME_MAX: c_int = 180;
pub const _SC_NPROCESSORS_ONLN: c_int = 84;
pub const _SC_PAGESIZE: c_int = 30;

s! {
    pub struct in_addr {
        pub s_addr: crate::in_addr_t,
    }

    pub struct ip_mreq {
        pub imr_multiaddr: in_addr,
        pub imr_interface: in_addr,
    }

    pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: crate::in_port_t,
        pub sin_addr: crate::in_addr,
        pub sin_zero: [u8; 8],
    }

    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: crate::in_port_t,
        pub sin6_flowinfo: u32,
        pub sin6_addr: crate::in6_addr,
        pub sin6_scope_id: u32,
    }

    pub struct sockaddr_storage {
        pub ss_family: sa_family_t,
        __ss_pad2: Padding<[u8; 128 - 2 - 8]>,
        __ss_align: size_t,
    }

    pub struct sockaddr_un {
        pub sun_family: sa_family_t,
        pub sun_path: [c_char; 108],
    }
}

extern "C" {
    pub fn accept4(fd: c_int, addr: *mut crate::sockaddr, len: *mut socklen_t, flg: c_int)
        -> c_int;
    pub fn bind(
        socket: c_int,
        address: *const crate::sockaddr,
        address_len: crate::socklen_t,
    ) -> c_int;
    pub fn clock_gettime(clk_id: crate::clockid_t, tp: *mut crate::timespec) -> c_int;
    pub fn clock_nanosleep(
        clk_id: crate::clockid_t,
        flags: c_int,
        rqtp: *const crate::timespec,
        rmtp: *mut crate::timespec,
    ) -> c_int;
    pub fn dirfd(dirp: *mut crate::DIR) -> c_int;
    pub fn fdatasync(fd: c_int) -> c_int;
    pub fn futimens(fd: c_int, times: *const crate::timespec) -> c_int;
    pub fn getpwuid_r(
        uid: crate::uid_t,
        pwd: *mut passwd,
        buf: *mut c_char,
        buflen: size_t,
        result: *mut *mut passwd,
    ) -> c_int;
    pub fn ioctl(fd: c_int, request: Ioctl, ...) -> c_int;
    pub fn pipe2(fds: *mut c_int, flags: c_int) -> c_int;
    pub fn preadv(
        fd: c_int,
        iov: *const crate::iovec,
        iovcnt: c_int,
        offset: crate::off_t,
    ) -> ssize_t;
    pub fn pthread_condattr_setclock(
        attr: *mut crate::pthread_condattr_t,
        clock_id: crate::clockid_t,
    ) -> c_int;
    pub fn pthread_create(
        native: *mut crate::pthread_t,
        attr: *const crate::pthread_attr_t,
        f: extern "C" fn(*mut c_void) -> *mut c_void,
        value: *mut c_void,
    ) -> c_int;
    pub fn pwritev(
        fd: c_int,
        iov: *const crate::iovec,
        iovcnt: c_int,
        offset: crate::off_t,
    ) -> ssize_t;
    pub fn readv(fd: c_int, iov: *const crate::iovec, iovcnt: c_int) -> ssize_t;
    pub fn recvfrom(
        socket: c_int,
        buf: *mut c_void,
        len: size_t,
        flags: c_int,
        addr: *mut crate::sockaddr,
        addrlen: *mut crate::socklen_t,
    ) -> ssize_t;
    pub fn setgroups(ngroups: size_t, ptr: *const crate::gid_t) -> c_int;
    pub fn utimensat(
        dirfd: c_int,
        path: *const c_char,
        times: *const crate::timespec,
        flag: c_int,
    ) -> c_int;
    pub fn writev(fd: c_int, iov: *const crate::iovec, iovcnt: c_int) -> ssize_t;
}

safe_f! {
    pub const safe fn WCOREDUMP(status: c_int) -> bool {
        (status & 0x80) != 0
    }

    pub const safe fn WEXITSTATUS(status: c_int) -> c_int {
        (status >> 8) & 0xff
    }

    pub const safe fn WIFCONTINUED(status: c_int) -> bool {
        status == 0xffff
    }

    pub const safe fn WIFEXITED(status: c_int) -> bool {
        (status & 0x7f) == 0
    }

    pub const safe fn WIFSIGNALED(status: c_int) -> bool {
        ((status & 0x7f) + 1) as i8 >= 2
    }

    pub const safe fn WIFSTOPPED(status: c_int) -> bool {
        (status & 0xff) == 0x7f
    }

    pub const safe fn WSTOPSIG(status: c_int) -> c_int {
        (status >> 8) & 0xff
    }

    pub const safe fn WTERMSIG(status: c_int) -> c_int {
        status & 0x7f
    }
}
