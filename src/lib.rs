// Copyright 2014 The syscall.rs Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Raw system calls for Rust.

// Reference http://man7.org/linux/man-pages/man2/syscall.2.html

#![deny(warnings)]
#![allow(non_camel_case_types)]
#![feature(asm)]
#![no_std]

#[cfg(test)]
extern crate std;

#[macro_use]
extern crate cfg_if;

pub use platform::*;

cfg_if! {
    if #[cfg(all(
        target_pointer_width = "32",
        target_os = "linux",
        any(target_arch = "aarch64",
            target_arch = "mips64",
            target_arch = "powerpc64",
            target_arch = "sparc64",
            target_arch = "x86_64")))] {
        pub type ireg = i64;
        pub type ureg = u64;
    } else {
        pub type ireg = isize;
        pub type ureg = usize;
    }
}

cfg_if! {
    if #[cfg(all(
        target_pointer_width = "32",
        target_os = "linux",
        any(target_arch = "aarch64",
            target_arch = "mips64",
            target_arch = "powerpc64",
            target_arch = "sparc64",
            target_arch = "x86_64")))] {
        #[path="macros/64.rs"]
        pub mod macros;
    } else if #[cfg(target_pointer_width = "64")] {
        #[path="macros/64.rs"]
        pub mod macros;
    } else if #[cfg(all(
        target_pointer_width = "32",
        target_endian = "big",
        target_os = "linux",
        any(target_arch = "arm",
            target_arch = "mips",
            target_arch = "powerpc",
            target_arch = "xtensa")))] {
        #[path="macros/32be-align.rs"]
        pub mod macros;
    } else if #[cfg(all(
        target_pointer_width = "32",
        target_endian = "big"))] {
        #[path="macros/32be.rs"]
        pub mod macros;
    } else if #[cfg(all(
        target_pointer_width = "32",
        target_endian = "little",
        target_os = "linux",
        any(target_arch = "arm",
            target_arch = "mips",
            target_arch = "powerpc",
            target_arch = "xtensa")))] {
        #[path="macros/32le-align.rs"]
        pub mod macros;
    } else if #[cfg(all(
        target_pointer_width = "32",
        target_endian = "little"))] {
        #[path="macros/32le.rs"]
        pub mod macros;
    } else {}
}

#[cfg(all(target_os = "linux",
          target_arch = "aarch64"))]
#[path="platform/linux-aarch64/mod.rs"]
pub mod platform;

#[cfg(all(target_os = "linux",
          target_arch = "arm"))]
#[path="platform/linux-armeabi/mod.rs"]
pub mod platform;

#[cfg(all(target_os = "linux",
          target_arch = "mips"))]
#[path="platform/linux-mips/mod.rs"]
pub mod platform;

#[cfg(all(target_os = "linux",
          target_arch = "mips64"))]
#[path="platform/linux-mips64/mod.rs"]
pub mod platform;

#[cfg(all(target_os = "linux",
          target_arch = "powerpc"))]
#[path="platform/linux-powerpc/mod.rs"]
pub mod platform;

#[cfg(all(target_os = "linux",
          target_arch = "powerpc64"))]
#[path="platform/linux-powerpc64/mod.rs"]
pub mod platform;

#[cfg(all(target_os = "linux",
          target_arch = "sparc64"))]
#[path="platform/linux-sparc64/mod.rs"]
pub mod platform;

#[cfg(all(target_os = "linux",
          target_arch = "x86"))]
#[path="platform/linux-x86/mod.rs"]
pub mod platform;

#[cfg(all(target_os = "linux",
          target_arch = "x86_64"))]
#[path="platform/linux-x86_64/mod.rs"]
pub mod platform;

#[cfg(all(target_os = "freebsd",
          target_arch = "x86_64"))]
#[path="platform/freebsd-x86_64/mod.rs"]
pub mod platform;

#[cfg(all(target_os = "macos",
          target_arch = "x86_64"))]
#[path="platform/macos-x86_64/mod.rs"]
pub mod platform;
