/* Automatically generated by sc-gen 0.1.0 */

// pub const _LLSEEK: usize = __NR__llseek;
// pub const _NEWSELECT: usize = __NR__newselect;
// pub const _SYSCTL: usize = __NR__sysctl;
pub const ACCEPT: usize = 202;
pub const ACCEPT4: usize = 242;
// pub const ACCESS: usize = __NR_access;
pub const ACCT: usize = 89;
pub const ADD_KEY: usize = 217;
pub const ADJTIMEX: usize = 171;
// pub const ALARM: usize = __NR_alarm;
// pub const ARM_FADVISE64_64: usize = __NR_arm_fadvise64_64;
// pub const ARM_SYNC_FILE_RANGE: usize = __NR_arm_sync_file_range;
// pub const BDFLUSH: usize = __NR_bdflush;
pub const BIND: usize = 200;
pub const BRK: usize = 214;
pub const CAPGET: usize = 90;
pub const CAPSET: usize = 91;
pub const CHDIR: usize = 49;
// pub const CHMOD: usize = __NR_chmod;
// pub const CHOWN: usize = __NR_chown;
// pub const CHOWN32: usize = __NR_chown32;
pub const CHROOT: usize = 51;
pub const CLOCK_ADJTIME: usize = 266;
pub const CLOCK_GETRES: usize = 114;
pub const CLOCK_GETTIME: usize = 113;
pub const CLOCK_NANOSLEEP: usize = 115;
pub const CLOCK_SETTIME: usize = 112;
pub const CLONE: usize = 220;
pub const CLOSE: usize = 57;
pub const CONNECT: usize = 203;
// pub const CREAT: usize = __NR_creat;
pub const DELETE_MODULE: usize = 106;
pub const DUP: usize = 23;
// pub const DUP2: usize = __NR_dup2;
pub const DUP3: usize = 24;
// pub const EPOLL_CREATE: usize = __NR_epoll_create;
pub const EPOLL_CREATE1: usize = 20;
pub const EPOLL_CTL: usize = 21;
pub const EPOLL_PWAIT: usize = 22;
// pub const EPOLL_WAIT: usize = __NR_epoll_wait;
// pub const EVENTFD: usize = __NR_eventfd;
pub const EVENTFD2: usize = 19;
pub const EXECVE: usize = 221;
pub const EXIT: usize = 93;
pub const EXIT_GROUP: usize = 94;
pub const FACCESSAT: usize = 48;
pub const FALLOCATE: usize = 47;
pub const FANOTIFY_INIT: usize = 262;
pub const FANOTIFY_MARK: usize = 263;
pub const FCHDIR: usize = 50;
pub const FCHMOD: usize = 52;
pub const FCHMODAT: usize = 53;
pub const FCHOWN: usize = 55;
// pub const FCHOWN32: usize = __NR_fchown32;
pub const FCHOWNAT: usize = 54;
pub const FCNTL: usize = 25;
// pub const FCNTL64: usize = __NR_fcntl64;
pub const FDATASYNC: usize = 83;
pub const FGETXATTR: usize = 10;
pub const FINIT_MODULE: usize = 273;
pub const FLISTXATTR: usize = 13;
pub const FLOCK: usize = 32;
// pub const FORK: usize = __NR_fork;
pub const FREMOVEXATTR: usize = 16;
pub const FSETXATTR: usize = 7;
pub const FSTAT: usize = 80;
// pub const FSTAT64: usize = __NR_fstat64;
// pub const FSTATAT64: usize = __NR_fstatat64;
pub const FSTATFS: usize = 44;
// pub const FSTATFS64: usize = __NR_fstatfs64;
pub const FSYNC: usize = 82;
pub const FTRUNCATE: usize = 46;
// pub const FTRUNCATE64: usize = __NR_ftruncate64;
pub const FUTEX: usize = 98;
// pub const FUTIMESAT: usize = __NR_futimesat;
pub const GET_MEMPOLICY: usize = 236;
pub const GET_ROBUST_LIST: usize = 100;
pub const GETCPU: usize = 168;
pub const GETCWD: usize = 17;
// pub const GETDENTS: usize = __NR_getdents;
pub const GETDENTS64: usize = 61;
pub const GETEGID: usize = 177;
// pub const GETEGID32: usize = __NR_getegid32;
pub const GETEUID: usize = 175;
// pub const GETEUID32: usize = __NR_geteuid32;
pub const GETGID: usize = 176;
// pub const GETGID32: usize = __NR_getgid32;
pub const GETGROUPS: usize = 158;
// pub const GETGROUPS32: usize = __NR_getgroups32;
pub const GETITIMER: usize = 102;
pub const GETPEERNAME: usize = 205;
pub const GETPGID: usize = 155;
// pub const GETPGRP: usize = __NR_getpgrp;
pub const GETPID: usize = 172;
pub const GETPPID: usize = 173;
pub const GETPRIORITY: usize = 141;
pub const GETRESGID: usize = 150;
// pub const GETRESGID32: usize = __NR_getresgid32;
pub const GETRESUID: usize = 148;
// pub const GETRESUID32: usize = __NR_getresuid32;
pub const GETRLIMIT: usize = 163;
pub const GETRUSAGE: usize = 165;
pub const GETSID: usize = 156;
pub const GETSOCKNAME: usize = 204;
pub const GETSOCKOPT: usize = 209;
pub const GETTID: usize = 178;
pub const GETTIMEOFDAY: usize = 169;
pub const GETUID: usize = 174;
// pub const GETUID32: usize = __NR_getuid32;
pub const GETXATTR: usize = 8;
pub const INIT_MODULE: usize = 105;
pub const INOTIFY_ADD_WATCH: usize = 27;
// pub const INOTIFY_INIT: usize = __NR_inotify_init;
pub const INOTIFY_INIT1: usize = 26;
pub const INOTIFY_RM_WATCH: usize = 28;
pub const IO_CANCEL: usize = 3;
pub const IO_DESTROY: usize = 1;
pub const IO_GETEVENTS: usize = 4;
pub const IO_SETUP: usize = 0;
pub const IO_SUBMIT: usize = 2;
pub const IOCTL: usize = 29;
pub const IOPRIO_GET: usize = 31;
pub const IOPRIO_SET: usize = 30;
// pub const IPC: usize = __NR_ipc;
pub const KCMP: usize = 272;
pub const KEXEC_LOAD: usize = 104;
pub const KEYCTL: usize = 219;
pub const KILL: usize = 129;
// pub const LCHOWN: usize = __NR_lchown;
// pub const LCHOWN32: usize = __NR_lchown32;
pub const LGETXATTR: usize = 9;
// pub const LINK: usize = __NR_link;
pub const LINKAT: usize = 37;
pub const LISTEN: usize = 201;
pub const LISTXATTR: usize = 11;
pub const LLISTXATTR: usize = 12;
pub const LOOKUP_DCOOKIE: usize = 18;
pub const LREMOVEXATTR: usize = 15;
pub const LSEEK: usize = 62;
pub const LSETXATTR: usize = 6;
// pub const LSTAT: usize = __NR_lstat;
// pub const LSTAT64: usize = __NR_lstat64;
pub const MADVISE: usize = 233;
pub const MBIND: usize = 235;
pub const MINCORE: usize = 232;
// pub const MKDIR: usize = __NR_mkdir;
pub const MKDIRAT: usize = 34;
// pub const MKNOD: usize = __NR_mknod;
pub const MKNODAT: usize = 33;
pub const MLOCK: usize = 228;
pub const MLOCKALL: usize = 230;
pub const MMAP: usize = 222;
// pub const MMAP2: usize = __NR_mmap2;
pub const MOUNT: usize = 40;
pub const MOVE_PAGES: usize = 239;
pub const MPROTECT: usize = 226;
pub const MQ_GETSETATTR: usize = 185;
pub const MQ_NOTIFY: usize = 184;
pub const MQ_OPEN: usize = 180;
pub const MQ_TIMEDRECEIVE: usize = 183;
pub const MQ_TIMEDSEND: usize = 182;
pub const MQ_UNLINK: usize = 181;
pub const MREMAP: usize = 216;
pub const MSGCTL: usize = 187;
pub const MSGGET: usize = 186;
pub const MSGRCV: usize = 188;
pub const MSGSND: usize = 189;
pub const MSYNC: usize = 227;
pub const MUNLOCK: usize = 229;
pub const MUNLOCKALL: usize = 231;
pub const MUNMAP: usize = 215;
pub const NAME_TO_HANDLE_AT: usize = 264;
pub const NANOSLEEP: usize = 101;
pub const NEWFSTATAT: usize = 79;
pub const NFSSERVCTL: usize = 42;
// pub const NICE: usize = __NR_nice;
// pub const OPEN: usize = __NR_open;
pub const OPEN_BY_HANDLE_AT: usize = 265;
pub const OPENAT: usize = 56;
// pub const PAUSE: usize = __NR_pause;
// pub const PCICONFIG_IOBASE: usize = __NR_pciconfig_iobase;
// pub const PCICONFIG_READ: usize = __NR_pciconfig_read;
// pub const PCICONFIG_WRITE: usize = __NR_pciconfig_write;
pub const PERF_EVENT_OPEN: usize = 241;
pub const PERSONALITY: usize = 92;
// pub const PIPE: usize = __NR_pipe;
pub const PIPE2: usize = 59;
pub const PIVOT_ROOT: usize = 41;
// pub const POLL: usize = __NR_poll;
pub const PPOLL: usize = 73;
pub const PRCTL: usize = 167;
pub const PREAD64: usize = 67;
pub const PREADV: usize = 69;
pub const PRLIMIT64: usize = 261;
pub const PROCESS_VM_READV: usize = 270;
pub const PROCESS_VM_WRITEV: usize = 271;
pub const PSELECT6: usize = 72;
pub const PTRACE: usize = 117;
pub const PWRITE64: usize = 68;
pub const PWRITEV: usize = 70;
pub const QUOTACTL: usize = 60;
pub const READ: usize = 63;
pub const READAHEAD: usize = 213;
// pub const READDIR: usize = __NR_readdir;
// pub const READLINK: usize = __NR_readlink;
pub const READLINKAT: usize = 78;
pub const READV: usize = 65;
pub const REBOOT: usize = 142;
// pub const RECV: usize = __NR_recv;
pub const RECVFROM: usize = 207;
pub const RECVMMSG: usize = 243;
pub const RECVMSG: usize = 212;
pub const REMAP_FILE_PAGES: usize = 234;
pub const REMOVEXATTR: usize = 14;
// pub const RENAME: usize = __NR_rename;
pub const RENAMEAT: usize = 38;
pub const REQUEST_KEY: usize = 218;
pub const RESTART_SYSCALL: usize = 128;
// pub const RMDIR: usize = __NR_rmdir;
pub const RT_SIGACTION: usize = 134;
pub const RT_SIGPENDING: usize = 136;
pub const RT_SIGPROCMASK: usize = 135;
pub const RT_SIGQUEUEINFO: usize = 138;
pub const RT_SIGRETURN: usize = 139;
pub const RT_SIGSUSPEND: usize = 133;
pub const RT_SIGTIMEDWAIT: usize = 137;
pub const RT_TGSIGQUEUEINFO: usize = 240;
pub const SCHED_GET_PRIORITY_MAX: usize = 125;
pub const SCHED_GET_PRIORITY_MIN: usize = 126;
pub const SCHED_GETAFFINITY: usize = 123;
pub const SCHED_GETPARAM: usize = 121;
pub const SCHED_GETSCHEDULER: usize = 120;
pub const SCHED_RR_GET_INTERVAL: usize = 127;
pub const SCHED_SETAFFINITY: usize = 122;
pub const SCHED_SETPARAM: usize = 118;
pub const SCHED_SETSCHEDULER: usize = 119;
pub const SCHED_YIELD: usize = 124;
// pub const SELECT: usize = __NR_select;
pub const SEMCTL: usize = 191;
pub const SEMGET: usize = 190;
pub const SEMOP: usize = 193;
pub const SEMTIMEDOP: usize = 192;
// pub const SEND: usize = __NR_send;
pub const SENDFILE: usize = 71;
// pub const SENDFILE64: usize = __NR_sendfile64;
pub const SENDMMSG: usize = 269;
pub const SENDMSG: usize = 211;
pub const SENDTO: usize = 206;
pub const SET_MEMPOLICY: usize = 237;
pub const SET_ROBUST_LIST: usize = 99;
pub const SET_TID_ADDRESS: usize = 96;
pub const SETDOMAINNAME: usize = 162;
pub const SETFSGID: usize = 152;
// pub const SETFSGID32: usize = __NR_setfsgid32;
pub const SETFSUID: usize = 151;
// pub const SETFSUID32: usize = __NR_setfsuid32;
pub const SETGID: usize = 144;
// pub const SETGID32: usize = __NR_setgid32;
pub const SETGROUPS: usize = 159;
// pub const SETGROUPS32: usize = __NR_setgroups32;
pub const SETHOSTNAME: usize = 161;
pub const SETITIMER: usize = 103;
pub const SETNS: usize = 268;
pub const SETPGID: usize = 154;
pub const SETPRIORITY: usize = 140;
pub const SETREGID: usize = 143;
// pub const SETREGID32: usize = __NR_setregid32;
pub const SETRESGID: usize = 149;
// pub const SETRESGID32: usize = __NR_setresgid32;
pub const SETRESUID: usize = 147;
// pub const SETRESUID32: usize = __NR_setresuid32;
pub const SETREUID: usize = 145;
// pub const SETREUID32: usize = __NR_setreuid32;
pub const SETRLIMIT: usize = 164;
pub const SETSID: usize = 157;
pub const SETSOCKOPT: usize = 208;
pub const SETTIMEOFDAY: usize = 170;
pub const SETUID: usize = 146;
// pub const SETUID32: usize = __NR_setuid32;
pub const SETXATTR: usize = 5;
pub const SHMAT: usize = 196;
pub const SHMCTL: usize = 195;
pub const SHMDT: usize = 197;
pub const SHMGET: usize = 194;
pub const SHUTDOWN: usize = 210;
// pub const SIGACTION: usize = __NR_sigaction;
pub const SIGALTSTACK: usize = 132;
// pub const SIGNALFD: usize = __NR_signalfd;
pub const SIGNALFD4: usize = 74;
// pub const SIGPENDING: usize = __NR_sigpending;
// pub const SIGPROCMASK: usize = __NR_sigprocmask;
// pub const SIGRETURN: usize = __NR_sigreturn;
// pub const SIGSUSPEND: usize = __NR_sigsuspend;
pub const SOCKET: usize = 198;
// pub const SOCKETCALL: usize = __NR_socketcall;
pub const SOCKETPAIR: usize = 199;
pub const SPLICE: usize = 76;
// pub const STAT: usize = __NR_stat;
// pub const STAT64: usize = __NR_stat64;
pub const STATFS: usize = 43;
// pub const STATFS64: usize = __NR_statfs64;
// pub const STIME: usize = __NR_stime;
pub const SWAPOFF: usize = 225;
pub const SWAPON: usize = 224;
// pub const SYMLINK: usize = __NR_symlink;
pub const SYMLINKAT: usize = 36;
pub const SYNC: usize = 81;
// pub const SYNC_FILE_RANGE2: usize = __NR_sync_file_range2;
pub const SYNCFS: usize = 267;
// pub const SYSCALL: usize = __NR_syscall;
// pub const SYSFS: usize = __NR_sysfs;
pub const SYSINFO: usize = 179;
pub const SYSLOG: usize = 116;
pub const TEE: usize = 77;
pub const TGKILL: usize = 131;
// pub const TIME: usize = __NR_time;
pub const TIMER_CREATE: usize = 107;
pub const TIMER_DELETE: usize = 111;
pub const TIMER_GETOVERRUN: usize = 109;
pub const TIMER_GETTIME: usize = 108;
pub const TIMER_SETTIME: usize = 110;
pub const TIMERFD_CREATE: usize = 85;
pub const TIMERFD_GETTIME: usize = 87;
pub const TIMERFD_SETTIME: usize = 86;
pub const TIMES: usize = 153;
pub const TKILL: usize = 130;
pub const TRUNCATE: usize = 45;
// pub const TRUNCATE64: usize = __NR_truncate64;
// pub const UGETRLIMIT: usize = __NR_ugetrlimit;
pub const UMASK: usize = 166;
// pub const UMOUNT: usize = __NR_umount;
pub const UMOUNT2: usize = 39;
pub const UNAME: usize = 160;
// pub const UNLINK: usize = __NR_unlink;
pub const UNLINKAT: usize = 35;
pub const UNSHARE: usize = 97;
// pub const USELIB: usize = __NR_uselib;
// pub const USTAT: usize = __NR_ustat;
// pub const UTIME: usize = __NR_utime;
pub const UTIMENSAT: usize = 88;
// pub const UTIMES: usize = __NR_utimes;
// pub const VFORK: usize = __NR_vfork;
pub const VHANGUP: usize = 58;
pub const VMSPLICE: usize = 75;
// pub const VSERVER: usize = __NR_vserver;
pub const WAIT4: usize = 260;
pub const WAITID: usize = 95;
pub const WRITE: usize = 64;
pub const WRITEV: usize = 66;
