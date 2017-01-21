/* Automatically generated by sc-gen 0.1.0 */

pub const _LLSEEK: usize = 4140;
pub const _NEWSELECT: usize = 4142;
pub const _SYSCTL: usize = 4153;
pub const ACCEPT: usize = 4168;
pub const ACCEPT4: usize = 4334;
pub const ACCESS: usize = 4033;
pub const ACCT: usize = 4051;
pub const ADD_KEY: usize = 4280;
pub const ADJTIMEX: usize = 4124;
pub const ALARM: usize = 4027;
// pub const ARM_FADVISE64_64: usize = __NR_arm_fadvise64_64;
// pub const ARM_SYNC_FILE_RANGE: usize = __NR_arm_sync_file_range;
pub const BDFLUSH: usize = 4134;
pub const BIND: usize = 4169;
pub const BRK: usize = 4045;
pub const CAPGET: usize = 4204;
pub const CAPSET: usize = 4205;
pub const CHDIR: usize = 4012;
pub const CHMOD: usize = 4015;
pub const CHOWN: usize = 4202;
// pub const CHOWN32: usize = __NR_chown32;
pub const CHROOT: usize = 4061;
pub const CLOCK_ADJTIME: usize = 4341;
pub const CLOCK_GETRES: usize = 4264;
pub const CLOCK_GETTIME: usize = 4263;
pub const CLOCK_NANOSLEEP: usize = 4265;
pub const CLOCK_SETTIME: usize = 4262;
pub const CLONE: usize = 4120;
pub const CLOSE: usize = 4006;
pub const CONNECT: usize = 4170;
pub const CREAT: usize = 4008;
pub const DELETE_MODULE: usize = 4129;
pub const DUP: usize = 4041;
pub const DUP2: usize = 4063;
pub const DUP3: usize = 4327;
pub const EPOLL_CREATE: usize = 4248;
pub const EPOLL_CREATE1: usize = 4326;
pub const EPOLL_CTL: usize = 4249;
pub const EPOLL_PWAIT: usize = 4313;
pub const EPOLL_WAIT: usize = 4250;
pub const EVENTFD: usize = 4319;
pub const EVENTFD2: usize = 4325;
pub const EXECVE: usize = 4011;
pub const EXIT: usize = 4001;
pub const EXIT_GROUP: usize = 4246;
pub const FACCESSAT: usize = 4300;
pub const FALLOCATE: usize = 4320;
pub const FANOTIFY_INIT: usize = 4336;
pub const FANOTIFY_MARK: usize = 4337;
pub const FCHDIR: usize = 4133;
pub const FCHMOD: usize = 4094;
pub const FCHMODAT: usize = 4299;
pub const FCHOWN: usize = 4095;
// pub const FCHOWN32: usize = __NR_fchown32;
pub const FCHOWNAT: usize = 4291;
pub const FCNTL: usize = 4055;
pub const FCNTL64: usize = 4220;
pub const FDATASYNC: usize = 4152;
pub const FGETXATTR: usize = 4229;
pub const FINIT_MODULE: usize = 4348;
pub const FLISTXATTR: usize = 4232;
pub const FLOCK: usize = 4143;
pub const FORK: usize = 4002;
pub const FREMOVEXATTR: usize = 4235;
pub const FSETXATTR: usize = 4226;
pub const FSTAT: usize = 4108;
pub const FSTAT64: usize = 4215;
pub const FSTATAT64: usize = 4293;
pub const FSTATFS: usize = 4100;
pub const FSTATFS64: usize = 4256;
pub const FSYNC: usize = 4118;
pub const FTRUNCATE: usize = 4093;
pub const FTRUNCATE64: usize = 4212;
pub const FUTEX: usize = 4238;
pub const FUTIMESAT: usize = 4292;
pub const GET_MEMPOLICY: usize = 4269;
pub const GET_ROBUST_LIST: usize = 4310;
pub const GETCPU: usize = 4312;
pub const GETCWD: usize = 4203;
pub const GETDENTS: usize = 4141;
pub const GETDENTS64: usize = 4219;
pub const GETEGID: usize = 4050;
// pub const GETEGID32: usize = __NR_getegid32;
pub const GETEUID: usize = 4049;
// pub const GETEUID32: usize = __NR_geteuid32;
pub const GETGID: usize = 4047;
// pub const GETGID32: usize = __NR_getgid32;
pub const GETGROUPS: usize = 4080;
// pub const GETGROUPS32: usize = __NR_getgroups32;
pub const GETITIMER: usize = 4105;
pub const GETPEERNAME: usize = 4171;
pub const GETPGID: usize = 4132;
pub const GETPGRP: usize = 4065;
pub const GETPID: usize = 4020;
pub const GETPPID: usize = 4064;
pub const GETPRIORITY: usize = 4096;
pub const GETRESGID: usize = 4191;
// pub const GETRESGID32: usize = __NR_getresgid32;
pub const GETRESUID: usize = 4186;
// pub const GETRESUID32: usize = __NR_getresuid32;
pub const GETRLIMIT: usize = 4076;
pub const GETRUSAGE: usize = 4077;
pub const GETSID: usize = 4151;
pub const GETSOCKNAME: usize = 4172;
pub const GETSOCKOPT: usize = 4173;
pub const GETTID: usize = 4222;
pub const GETTIMEOFDAY: usize = 4078;
pub const GETUID: usize = 4024;
// pub const GETUID32: usize = __NR_getuid32;
pub const GETXATTR: usize = 4227;
pub const INIT_MODULE: usize = 4128;
pub const INOTIFY_ADD_WATCH: usize = 4285;
pub const INOTIFY_INIT: usize = 4284;
pub const INOTIFY_INIT1: usize = 4329;
pub const INOTIFY_RM_WATCH: usize = 4286;
pub const IO_CANCEL: usize = 4245;
pub const IO_DESTROY: usize = 4242;
pub const IO_GETEVENTS: usize = 4243;
pub const IO_SETUP: usize = 4241;
pub const IO_SUBMIT: usize = 4244;
pub const IOCTL: usize = 4054;
pub const IOPRIO_GET: usize = 4315;
pub const IOPRIO_SET: usize = 4314;
pub const IPC: usize = 4117;
pub const KCMP: usize = 4347;
pub const KEXEC_LOAD: usize = 4311;
pub const KEYCTL: usize = 4282;
pub const KILL: usize = 4037;
pub const LCHOWN: usize = 4016;
// pub const LCHOWN32: usize = __NR_lchown32;
pub const LGETXATTR: usize = 4228;
pub const LINK: usize = 4009;
pub const LINKAT: usize = 4296;
pub const LISTEN: usize = 4174;
pub const LISTXATTR: usize = 4230;
pub const LLISTXATTR: usize = 4231;
pub const LOOKUP_DCOOKIE: usize = 4247;
pub const LREMOVEXATTR: usize = 4234;
pub const LSEEK: usize = 4019;
pub const LSETXATTR: usize = 4225;
pub const LSTAT: usize = 4107;
pub const LSTAT64: usize = 4214;
pub const MADVISE: usize = 4218;
pub const MBIND: usize = 4268;
pub const MINCORE: usize = 4217;
pub const MKDIR: usize = 4039;
pub const MKDIRAT: usize = 4289;
pub const MKNOD: usize = 4014;
pub const MKNODAT: usize = 4290;
pub const MLOCK: usize = 4154;
pub const MLOCKALL: usize = 4156;
pub const MMAP: usize = 4090;
pub const MMAP2: usize = 4210;
pub const MOUNT: usize = 4021;
pub const MOVE_PAGES: usize = 4308;
pub const MPROTECT: usize = 4125;
pub const MQ_GETSETATTR: usize = 4276;
pub const MQ_NOTIFY: usize = 4275;
pub const MQ_OPEN: usize = 4271;
pub const MQ_TIMEDRECEIVE: usize = 4274;
pub const MQ_TIMEDSEND: usize = 4273;
pub const MQ_UNLINK: usize = 4272;
pub const MREMAP: usize = 4167;
// pub const MSGCTL: usize = __NR_msgctl;
// pub const MSGGET: usize = __NR_msgget;
// pub const MSGRCV: usize = __NR_msgrcv;
// pub const MSGSND: usize = __NR_msgsnd;
pub const MSYNC: usize = 4144;
pub const MUNLOCK: usize = 4155;
pub const MUNLOCKALL: usize = 4157;
pub const MUNMAP: usize = 4091;
pub const NAME_TO_HANDLE_AT: usize = 4339;
pub const NANOSLEEP: usize = 4166;
// pub const NEWFSTATAT: usize = __NR_newfstatat;
pub const NFSSERVCTL: usize = 4189;
pub const NICE: usize = 4034;
pub const OPEN: usize = 4005;
pub const OPEN_BY_HANDLE_AT: usize = 4340;
pub const OPENAT: usize = 4288;
pub const PAUSE: usize = 4029;
// pub const PCICONFIG_IOBASE: usize = __NR_pciconfig_iobase;
// pub const PCICONFIG_READ: usize = __NR_pciconfig_read;
// pub const PCICONFIG_WRITE: usize = __NR_pciconfig_write;
pub const PERF_EVENT_OPEN: usize = 4333;
pub const PERSONALITY: usize = 4136;
pub const PIPE: usize = 4042;
pub const PIPE2: usize = 4328;
pub const PIVOT_ROOT: usize = 4216;
pub const POLL: usize = 4188;
pub const PPOLL: usize = 4302;
pub const PRCTL: usize = 4192;
pub const PREAD64: usize = 4200;
pub const PREADV: usize = 4330;
pub const PRLIMIT64: usize = 4338;
pub const PROCESS_VM_READV: usize = 4345;
pub const PROCESS_VM_WRITEV: usize = 4346;
pub const PSELECT6: usize = 4301;
pub const PTRACE: usize = 4026;
pub const PWRITE64: usize = 4201;
pub const PWRITEV: usize = 4331;
pub const QUOTACTL: usize = 4131;
pub const READ: usize = 4003;
pub const READAHEAD: usize = 4223;
pub const READDIR: usize = 4089;
pub const READLINK: usize = 4085;
pub const READLINKAT: usize = 4298;
pub const READV: usize = 4145;
pub const REBOOT: usize = 4088;
pub const RECV: usize = 4175;
pub const RECVFROM: usize = 4176;
pub const RECVMMSG: usize = 4335;
pub const RECVMSG: usize = 4177;
pub const REMAP_FILE_PAGES: usize = 4251;
pub const REMOVEXATTR: usize = 4233;
pub const RENAME: usize = 4038;
pub const RENAMEAT: usize = 4295;
pub const REQUEST_KEY: usize = 4281;
pub const RESTART_SYSCALL: usize = 4253;
pub const RMDIR: usize = 4040;
pub const RT_SIGACTION: usize = 4194;
pub const RT_SIGPENDING: usize = 4196;
pub const RT_SIGPROCMASK: usize = 4195;
pub const RT_SIGQUEUEINFO: usize = 4198;
pub const RT_SIGRETURN: usize = 4193;
pub const RT_SIGSUSPEND: usize = 4199;
pub const RT_SIGTIMEDWAIT: usize = 4197;
pub const RT_TGSIGQUEUEINFO: usize = 4332;
pub const SCHED_GET_PRIORITY_MAX: usize = 4163;
pub const SCHED_GET_PRIORITY_MIN: usize = 4164;
pub const SCHED_GETAFFINITY: usize = 4240;
pub const SCHED_GETPARAM: usize = 4159;
pub const SCHED_GETSCHEDULER: usize = 4161;
pub const SCHED_RR_GET_INTERVAL: usize = 4165;
pub const SCHED_SETAFFINITY: usize = 4239;
pub const SCHED_SETPARAM: usize = 4158;
pub const SCHED_SETSCHEDULER: usize = 4160;
pub const SCHED_YIELD: usize = 4162;
// pub const SELECT: usize = __NR_select;
// pub const SEMCTL: usize = __NR_semctl;
// pub const SEMGET: usize = __NR_semget;
// pub const SEMOP: usize = __NR_semop;
// pub const SEMTIMEDOP: usize = __NR_semtimedop;
pub const SEND: usize = 4178;
pub const SENDFILE: usize = 4207;
pub const SENDFILE64: usize = 4237;
pub const SENDMMSG: usize = 4343;
pub const SENDMSG: usize = 4179;
pub const SENDTO: usize = 4180;
pub const SET_MEMPOLICY: usize = 4270;
pub const SET_ROBUST_LIST: usize = 4309;
pub const SET_TID_ADDRESS: usize = 4252;
pub const SETDOMAINNAME: usize = 4121;
pub const SETFSGID: usize = 4139;
// pub const SETFSGID32: usize = __NR_setfsgid32;
pub const SETFSUID: usize = 4138;
// pub const SETFSUID32: usize = __NR_setfsuid32;
pub const SETGID: usize = 4046;
// pub const SETGID32: usize = __NR_setgid32;
pub const SETGROUPS: usize = 4081;
// pub const SETGROUPS32: usize = __NR_setgroups32;
pub const SETHOSTNAME: usize = 4074;
pub const SETITIMER: usize = 4104;
pub const SETNS: usize = 4344;
pub const SETPGID: usize = 4057;
pub const SETPRIORITY: usize = 4097;
pub const SETREGID: usize = 4071;
// pub const SETREGID32: usize = __NR_setregid32;
pub const SETRESGID: usize = 4190;
// pub const SETRESGID32: usize = __NR_setresgid32;
pub const SETRESUID: usize = 4185;
// pub const SETRESUID32: usize = __NR_setresuid32;
pub const SETREUID: usize = 4070;
// pub const SETREUID32: usize = __NR_setreuid32;
pub const SETRLIMIT: usize = 4075;
pub const SETSID: usize = 4066;
pub const SETSOCKOPT: usize = 4181;
pub const SETTIMEOFDAY: usize = 4079;
pub const SETUID: usize = 4023;
// pub const SETUID32: usize = __NR_setuid32;
pub const SETXATTR: usize = 4224;
// pub const SHMAT: usize = __NR_shmat;
// pub const SHMCTL: usize = __NR_shmctl;
// pub const SHMDT: usize = __NR_shmdt;
// pub const SHMGET: usize = __NR_shmget;
pub const SHUTDOWN: usize = 4182;
pub const SIGACTION: usize = 4067;
pub const SIGALTSTACK: usize = 4206;
pub const SIGNALFD: usize = 4317;
pub const SIGNALFD4: usize = 4324;
pub const SIGPENDING: usize = 4073;
pub const SIGPROCMASK: usize = 4126;
pub const SIGRETURN: usize = 4119;
pub const SIGSUSPEND: usize = 4072;
pub const SOCKET: usize = 4183;
pub const SOCKETCALL: usize = 4102;
pub const SOCKETPAIR: usize = 4184;
pub const SPLICE: usize = 4304;
pub const STAT: usize = 4106;
pub const STAT64: usize = 4213;
pub const STATFS: usize = 4099;
pub const STATFS64: usize = 4255;
pub const STIME: usize = 4025;
pub const SWAPOFF: usize = 4115;
pub const SWAPON: usize = 4087;
pub const SYMLINK: usize = 4083;
pub const SYMLINKAT: usize = 4297;
pub const SYNC: usize = 4036;
// pub const SYNC_FILE_RANGE2: usize = __NR_sync_file_range2;
pub const SYNCFS: usize = 4342;
pub const SYSCALL: usize = 4000;
pub const SYSFS: usize = 4135;
pub const SYSINFO: usize = 4116;
pub const SYSLOG: usize = 4103;
pub const TEE: usize = 4306;
pub const TGKILL: usize = 4266;
pub const TIME: usize = 4013;
pub const TIMER_CREATE: usize = 4257;
pub const TIMER_DELETE: usize = 4261;
pub const TIMER_GETOVERRUN: usize = 4260;
pub const TIMER_GETTIME: usize = 4259;
pub const TIMER_SETTIME: usize = 4258;
pub const TIMERFD_CREATE: usize = 4321;
pub const TIMERFD_GETTIME: usize = 4322;
pub const TIMERFD_SETTIME: usize = 4323;
pub const TIMES: usize = 4043;
pub const TKILL: usize = 4236;
pub const TRUNCATE: usize = 4092;
pub const TRUNCATE64: usize = 4211;
// pub const UGETRLIMIT: usize = __NR_ugetrlimit;
pub const UMASK: usize = 4060;
pub const UMOUNT: usize = 4022;
pub const UMOUNT2: usize = 4052;
pub const UNAME: usize = 4122;
pub const UNLINK: usize = 4010;
pub const UNLINKAT: usize = 4294;
pub const UNSHARE: usize = 4303;
pub const USELIB: usize = 4086;
pub const USTAT: usize = 4062;
pub const UTIME: usize = 4030;
pub const UTIMENSAT: usize = 4316;
pub const UTIMES: usize = 4267;
// pub const VFORK: usize = __NR_vfork;
pub const VHANGUP: usize = 4111;
pub const VMSPLICE: usize = 4307;
pub const VSERVER: usize = 4277;
pub const WAIT4: usize = 4114;
pub const WAITID: usize = 4278;
pub const WRITE: usize = 4004;
pub const WRITEV: usize = 4146;
