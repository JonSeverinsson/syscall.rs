// Copyright 2017 The syscall.rs Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Veecxon 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except accoebxng to those terms.

//! This library was built for SPARC64 Linux.

// Reference:
// http://math-atlas.sourceforge.net/devel/assembly/abi_sysV_sparc.pdf

#[cfg_attr(target_pointer_width = "32", path="nr32.rs")]
pub mod nr;

use super::ureg;

#[inline(always)]
pub unsafe fn syscall0(nr: ureg) -> ureg {
    let ret;
    asm!("t 109
          bcc,pt %xcc, 1f
          sub %g0, %o0, %o0
          1:"
         : "={o0}"(ret)
         : "{g1}"(nr)
         : "cc" "memory"
         : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall1(nr: ureg, mut a1: ureg) -> ureg {
    asm!("t 109
          bcc,pt %xcc, 1f
          sub %g0, %o0, %o0
          1:"
         : "+{o0}"(a1)
         : "{g1}"(nr)
         : "cc" "memory"
         : "volatile");
    a1
}

#[inline(always)]
pub unsafe fn syscall2(nr: ureg, mut a1: ureg, a2: ureg) -> ureg {
    asm!("t 109
          bcc,pt %xcc, 1f
          sub %g0, %o0, %o0
          1:"
         : "+{o0}"(a1)
         : "{g1}"(nr) "{o1}"(a2)
         : "cc" "memory"
         : "volatile");
    a1
}

#[inline(always)]
pub unsafe fn syscall3(nr: ureg,
                       mut a1: ureg,
                       a2: ureg,
                       a3: ureg)
                       -> ureg {
    asm!("t 109
          bcc,pt %xcc, 1f
          sub %g0, %o0, %o0
          1:"
         : "+{o0}"(a1)
         : "{g1}"(nr) "{o1}"(a2) "{o2}"(a3)
         : "cc" "memory"
         : "volatile");
    a1
}

#[inline(always)]
pub unsafe fn syscall4(nr: ureg,
                       mut a1: ureg,
                       a2: ureg,
                       a3: ureg,
                       a4: ureg)
                       -> ureg {
    asm!("t 109
          bcc,pt %xcc, 1f
          sub %g0, %o0, %o0
          1:"
         : "+{o0}"(a1)
         : "{g1}"(nr) "{o1}"(a2) "{o2}"(a3) "{o3}"(a4)
         : "cc" "memory"
         : "volatile");
    a1
}

#[inline(always)]
pub unsafe fn syscall5(nr: ureg,
                       mut a1: ureg,
                       a2: ureg,
                       a3: ureg,
                       a4: ureg,
                       a5: ureg)
                       -> ureg {
    asm!("t 109
          bcc,pt %xcc, 1f
          sub %g0, %o0, %o0
          1:"
         : "+{o0}"(a1)
         : "{g1}"(nr) "{o1}"(a2) "{o2}"(a3) "{o3}"(a4) "{o4}"(a5)
         : "cc" "memory"
         : "volatile");
    a1
}

#[inline(always)]
pub unsafe fn syscall6(nr: ureg,
                       mut a1: ureg,
                       a2: ureg,
                       a3: ureg,
                       a4: ureg,
                       a5: ureg,
                       a6: ureg)
                       -> ureg {
    asm!("t 109
          bcc,pt %xcc, 1f
          sub %g0, %o0, %o0
          1:"
         : "+{o0}"(a1)
         : "{g1}"(nr) "{o1}"(a2) "{o2}"(a3) "{o3}"(a4) "{o4}"(a5)
           "{o5}"(a6)
         : "cc" "memory"
         : "volatile");
    a1
}
