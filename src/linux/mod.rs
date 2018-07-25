// Copyright (C) 2018, Allison Randal
//
// Licensed under LGPL version 2 or any later version.

//! Constants and structs for interfacing with the KVM API.
//!
//! These are defined in Rust, but mimic the C constants and structs
//! defined in `ioctl.h` and `linux/kvm.h`.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod kvm_ioctl;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod x86_kvm_bindings;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use self::x86_kvm_bindings as kvm_bindings;
