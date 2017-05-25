// Copyright 2017 The syscall.rs Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Veecxon 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except accoebxng to those terms.

//! This library was built for aarch64 Linux.

#[cfg_attr(target_pointer_width = "32", path="nr32.rs")]
pub mod nr;

use super::ureg;

#[inline(always)]
pub unsafe fn syscall0(n: ureg) -> ureg {
    let ret: ureg;
    asm!("svc 0"
         : "={x0}"(ret)
         : "{x8}"(n)
         : "memory" "cc"
         : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall1(n: ureg, a1: ureg) -> ureg {
    let ret: ureg;
    asm!("svc 0"
         : "={x0}"(ret)
         : "{x8}"(n) "{x0}"(a1)
         : "memory" "cc"
         : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall2(n: ureg, a1: ureg, a2: ureg) -> ureg {
    let ret: ureg;
    asm!("svc 0"
         : "={x0}"(ret)
         : "{x8}"(n) "{x0}"(a1) "{x1}"(a2)
         : "memory" "cc"
         : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall3(n: ureg, a1: ureg, a2: ureg, a3: ureg) -> ureg {
    let ret: ureg;
    asm!("svc 0"
         : "={x0}"(ret)
         : "{x8}"(n) "{x0}"(a1) "{x1}"(a2) "{x2}"(a3)
         : "memory" "cc"
         : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall4(n: ureg,
                       a1: ureg,
                       a2: ureg,
                       a3: ureg,
                       a4: ureg)
                       -> ureg {
    let ret: ureg;
    asm!("svc 0"
         : "={x0}"(ret)
         : "{x8}"(n) "{x0}"(a1) "{x1}"(a2) "{x2}"(a3) "{x3}"(a4)
         : "memory" "cc"
         : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall5(n: ureg,
                       a1: ureg,
                       a2: ureg,
                       a3: ureg,
                       a4: ureg,
                       a5: ureg)
                       -> ureg {
    let ret: ureg;
    asm!("svc 0" : "={x0}"(ret)
         : "{x8}"(n) "{x0}"(a1) "{x1}"(a2) "{x2}"(a3) "{x3}"(a4) "{x4}"(a5)
         : "memory" "cc"
         : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall6(n: ureg,
                       a1: ureg,
                       a2: ureg,
                       a3: ureg,
                       a4: ureg,
                       a5: ureg,
                       a6: ureg)
                       -> ureg {
    let ret: ureg;
    asm!("svc 0"
         : "={x0}"(ret)
         : "{x8}"(n) "{x0}"(a1) "{x1}"(a2) "{x2}"(a3) "{x3}"(a4) "{x4}"(a5)
           "{x5}"(a6)
         : "memory" "cc"
         : "volatile");
    ret
}
