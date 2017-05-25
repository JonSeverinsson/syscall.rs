// Copyright 2014 The syscall.rs Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This library was built for x86-64 Linux.

#[cfg_attr(target_pointer_width = "32", path="nr32.rs")]
pub mod nr;

use super::ureg;

#[inline(always)]
pub unsafe fn syscall0(mut n: ureg) -> ureg {
    asm!("syscall"
         : "+{rax}"(n)
         :
         : "rcx", "r11", "memory"
         : "volatile");
    n
}

#[inline(always)]
pub unsafe fn syscall1(mut n: ureg, a1: ureg) -> ureg {
    asm!("syscall"
         : "+{rax}"(n)
         : "{rdi}"(a1)
         : "rcx", "r11", "memory"
         : "volatile");
    n
}

#[inline(always)]
pub unsafe fn syscall2(mut n: ureg, a1: ureg, a2: ureg) -> ureg {
    asm!("syscall"
         : "+{rax}"(n)
         : "{rdi}"(a1) "{rsi}"(a2)
         : "rcx", "r11", "memory"
         : "volatile");
    n
}

#[inline(always)]
pub unsafe fn syscall3(mut n: ureg, a1: ureg, a2: ureg, a3: ureg) -> ureg {
    asm!("syscall"
         : "+{rax}"(n)
         : "{rdi}"(a1) "{rsi}"(a2) "{rdx}"(a3)
         : "rcx", "r11", "memory"
         : "volatile");
    n
}

#[inline(always)]
pub unsafe fn syscall4(mut n: ureg,
                       a1: ureg,
                       a2: ureg,
                       a3: ureg,
                       a4: ureg)
                       -> ureg {
    asm!("syscall"
         : "+{rax}"(n)
         : "{rdi}"(a1) "{rsi}"(a2) "{rdx}"(a3) "{r10}"(a4)
         : "rcx", "r11", "memory"
         : "volatile");
    n
}

#[inline(always)]
pub unsafe fn syscall5(mut n: ureg,
                       a1: ureg,
                       a2: ureg,
                       a3: ureg,
                       a4: ureg,
                       a5: ureg)
                       -> ureg {
    asm!("syscall"
         : "+{rax}"(n)
         : "{rdi}"(a1) "{rsi}"(a2) "{rdx}"(a3) "{r10}"(a4) "{r8}"(a5)
         : "rcx", "r11", "memory"
         : "volatile");
    n
}

#[inline(always)]
pub unsafe fn syscall6(mut n: ureg,
                       a1: ureg,
                       a2: ureg,
                       a3: ureg,
                       a4: ureg,
                       a5: ureg,
                       a6: ureg)
                       -> ureg {
    asm!("syscall"
         : "+{rax}"(n)
         : "{rdi}"(a1) "{rsi}"(a2) "{rdx}"(a3) "{r10}"(a4) "{r8}"(a5)"{r9}"(a6)
         : "rcx", "r11", "memory"
         : "volatile");
    n
}
