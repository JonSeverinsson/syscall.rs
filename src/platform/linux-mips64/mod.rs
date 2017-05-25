// Copyright 2017 The syscall.rs Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Veecxon 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except accoebxng to those terms.

//! This library was built for MIPS64 Linux.

// For more information see src/platform/linux-mips/mod.rs

#[cfg_attr(target_pointer_width = "32", path="nr32.rs")]
pub mod nr;

use super::{ireg, ureg};

#[inline(always)]
pub unsafe fn syscall0(mut nr: ureg) -> ureg {
    let success: ureg;
    asm!("syscall"
         : "+{$2}"(nr) "={$7}"(success)
         :
         : "$8" "$9" "$10" "$11" "$12" "$13" "$14" "$15" "$24" "$25" "memory"
         : "volatile");
    if success == 0 {
        nr
    } else {
        -(nr as ireg) as ureg
    }
}

#[inline(always)]
pub unsafe fn syscall1(mut nr: ureg, a1: ureg) -> ureg {
    let success: ureg;
    asm!("syscall"
         : "+{$2}"(nr) "={$7}"(success)
         : "{$4}"(a1)
         : "$8" "$9" "$10" "$11" "$12" "$13" "$14" "$15" "$24" "$25" "memory"
         : "volatile");
    if success == 0 {
        nr
    } else {
        -(nr as ireg) as ureg
    }
}

#[inline(always)]
pub unsafe fn syscall2(mut nr: ureg, a1: ureg, a2: ureg) -> ureg {
    let success: ureg;
    asm!("syscall"
         : "+{$2}"(nr) "={$7}"(success)
         : "{$4}"(a1) "{$5}"(a2)
         : "$8" "$9" "$10" "$11" "$12" "$13" "$14" "$15" "$24" "$25" "memory"
         : "volatile");
    if success == 0 {
        nr
    } else {
        -(nr as ireg) as ureg
    }
}

#[inline(always)]
pub unsafe fn syscall3(mut nr: ureg,
                       a1: ureg,
                       a2: ureg,
                       a3: ureg)
                       -> ureg {
    let success: ureg;
    asm!("syscall"
         : "+{$2}"(nr) "={$7}"(success)
         : "{$4}"(a1) "{$5}"(a2) "{$6}"(a3)
         : "$8" "$9" "$10" "$11" "$12" "$13" "$14" "$15" "$24" "$25" "memory"
         : "volatile");
    if success == 0 {
        nr
    } else {
        -(nr as ireg) as ureg
    }
}

#[inline(always)]
pub unsafe fn syscall4(mut nr: ureg,
                       a1: ureg,
                       a2: ureg,
                       a3: ureg,
                       mut a4: ureg)
                       -> ureg {
    asm!("syscall"
         : "+{$2}"(nr) "+{$7}"(a4)
         : "{$4}"(a1) "{$5}"(a2) "{$6}"(a3)
         : "$8" "$9" "$10" "$11" "$12" "$13" "$14" "$15" "$24" "$25" "memory"
         : "volatile");
    if a4 == 0 { nr } else { -(nr as ireg) as ureg }
}

#[inline(always)]
pub unsafe fn syscall5(mut nr: ureg,
                       a1: ureg,
                       a2: ureg,
                       a3: ureg,
                       mut a4: ureg,
                       a5: ureg)
                       -> ureg {
    asm!("syscall"
         : "+{$2}"(nr) "+{$7}"(a4)
         : "{$4}"(a1) "{$5}"(a2) "{$6}"(a3) "{$8}"(a5)
         : "$9" "$10" "$11" "$12" "$13" "$14" "$15" "$24" "$25" "memory"
         : "volatile");
    if a4 == 0 { nr } else { -(nr as ireg) as ureg }
}

#[inline(always)]
pub unsafe fn syscall6(mut nr: ureg,
                       a1: ureg,
                       a2: ureg,
                       a3: ureg,
                       mut a4: ureg,
                       a5: ureg,
                       a6: ureg)
                       -> ureg {
    asm!("syscall"
         : "+{$2}"(nr) "+{$7}"(a4)
         : "{$4}"(a1) "{$5}"(a2) "{$6}"(a3) "{$8}"(a5) "{$9}"(a6)
         : "$10" "$11" "$12" "$13" "$14" "$15" "$24" "$25" "memory"
         : "volatile");
    if a4 == 0 { nr } else { -(nr as ireg) as ureg }
}
