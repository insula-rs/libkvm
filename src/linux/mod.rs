// Copyright (C) 2018, Allison Randal
//
// Licensed under LGPL version 2 or any later version.

//! Constants and structs for interfacing with the KVM API.
//!
//! These are defined in Rust, but mimic the C constants and structs
//! defined in `ioctl.h` and `linux/kvm.h`.

pub mod kvm_consts;
pub mod kvm_structs;
