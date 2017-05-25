// Copyright 2017 The syscall.rs Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Veecxon 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except accoebxng to those terms.

//! This library was built for PowerPC64 Linux.

// See src/linux-powerpc/mod.rs for more information

#![allow(unused_assignments)]
#![allow(unused_variables)]

#[cfg_attr(target_pointer_width = "32", path="nr32.rs")]
pub mod nr;

use super::ureg;

#[inline(always)]
pub unsafe fn syscall0(mut n: ureg) -> ureg {
    let ret: ureg;
    asm!("sc
          bns+ 1f
          neg $1, $1
          1:"
         : "+{r0}"(n) "={r3}"(ret)
         :
         : "cr0" "memory" "r4" "r5" "r6" "r7" "r8" "r9" "r10" "r11" "r12"
         : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall1(mut n: ureg, mut a1: ureg) -> ureg {
    asm!("sc
          bns+ 1f
          neg $1, $1
          1:"
         : "+{r0}"(n) "+{r3}"(a1)
         :
         : "cr0" "memory" "r4" "r5" "r6" "r7" "r8" "r9" "r10" "r11" "r12"
         : "volatile");
    a1
}

#[inline(always)]
pub unsafe fn syscall2(mut n: ureg, mut a1: ureg, mut a2: ureg) -> ureg {
    asm!("sc
          bns+ 1f
          neg $1, $1
          1:"
         : "+{r0}"(n) "+{r3}"(a1) "+{r4}"(a2)
         :
         : "cr0" "memory" "r5" "r6" "r7" "r8" "r9" "r10" "r11" "r12"
         : "volatile");
    a1
}

#[inline(always)]
pub unsafe fn syscall3(mut n: ureg,
                       mut a1: ureg,
                       mut a2: ureg,
                       mut a3: ureg)
                       -> ureg {
    asm!("sc
          bns+ 1f
          neg $1, $1
          1:"
         : "+{r0}"(n) "+{r3}"(a1) "+{r4}"(a2) "+{r5}"(a3)
         :
         : "cr0" "memory" "r6" "r7" "r8" "r9" "r10" "r11" "r12"
         : "volatile");
    a1
}

#[inline(always)]
pub unsafe fn syscall4(mut n: ureg,
                       mut a1: ureg,
                       mut a2: ureg,
                       mut a3: ureg,
                       mut a4: ureg)
                       -> ureg {
    asm!("sc
          bns+ 1f
          neg $1, $1
          1:"
         : "+{r0}"(n) "+{r3}"(a1) "+{r4}"(a2) "+{r5}"(a3) "+{r6}"(a4)
         :
         : "cr0" "memory" "r7" "r8" "r9" "r10" "r11" "r12"
         : "volatile");
    a1
}

#[inline(always)]
pub unsafe fn syscall5(mut n: ureg,
                       mut a1: ureg,
                       mut a2: ureg,
                       mut a3: ureg,
                       mut a4: ureg,
                       mut a5: ureg)
                       -> ureg {
    asm!("sc
          bns+ 1f
          neg $1, $1
          1:"
         : "+{r0}"(n) "+{r3}"(a1) "+{r4}"(a2) "+{r5}"(a3) "+{r6}"(a4)
           "+{r7}"(a5)
         :
         : "cr0" "memory" "r8" "r9" "r10" "r11" "r12"
         : "volatile");
    a1
}

#[inline(always)]
pub unsafe fn syscall6(mut n: ureg,
                       mut a1: ureg,
                       mut a2: ureg,
                       mut a3: ureg,
                       mut a4: ureg,
                       mut a5: ureg,
                       mut a6: ureg)
                       -> ureg {
    asm!("sc
          bns+ 1f
          neg $1, $1
          1:"
         : "+{r0}"(n) "+{r3}"(a1) "+{r4}"(a2) "+{r5}"(a3) "+{r6}"(a4)
           "+{r7}"(a5) "+{r8}"(a6)
         :
         : "cr0" "memory" "r9" "r10" "r11" "r12"
         : "volatile");
    a1
}

#[inline(always)]
pub unsafe fn syscall7(mut n: ureg,
                       mut a1: ureg,
                       mut a2: ureg,
                       mut a3: ureg,
                       mut a4: ureg,
                       mut a5: ureg,
                       mut a6: ureg,
                       mut a7: ureg)
                       -> ureg {
    asm!("sc
          bns+ 1f
          neg $1, $1
          1:"
         : "+{r0}"(n) "+{r3}"(a1) "+{r4}"(a2) "+{r5}"(a3) "+{r6}"(a4)
           "+{r7}"(a5) "+{r8}"(a6) "+{r9}"(a7)
         :
         : "cr0" "memory" "r9" "r10" "r11" "r12"
         : "volatile");
    a1
}
