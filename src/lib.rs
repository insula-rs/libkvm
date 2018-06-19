// Copyright (C) 2018, Allison Randal
//
// Licensed under LGPL version 2 or any later version.

//! # Rust library interface to KVM
//!
//! The libKVM library is a userspace interface to the hardware
//! virtualization features in the Linux Kernel provided by KVM. It is
//! a minimal interface, which can serve as the lowest userspace base
//! layer for any hypervisor to use KVM's hardware virtualization features. It
//! is essentially no more than a clean, safe wrapper around [KVM's ioctl
//! API](https://www.kernel.org/doc/Documentation/virtual/kvm/api.txt) (i.e
//! `/dev/kvm`).
//!
//! As part of being a minimal interface, libKVM avoids external
//! dependencies as much as possible, using only `libc` to access the
//! ioctl interface. The constants and structs required for the KVM
//! API are defined in Rust, rather than automatically generating
//! bindings to the C header files, which has benefits for usability
//! and maintainability, and simplifies reasoning from a security
//! perspective.

extern crate libc;

pub mod linux;
pub mod mem;
pub mod system;
pub mod vcpu;
pub mod vm;
