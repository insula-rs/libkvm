// Copyright (C) 2018, Allison Randal
//
// Licensed under LGPL version 2 or any later version.

//! Constants for KVM ioctl operations.
//!
//! This is a minimal subset of the constants defined in the C header
//! files `ioctl.h` and `linux/kvm.h`.

use std::mem::size_of;

use linux::kvm_structs::*;

// Define constants from ioctl.h

// Left shift constants for ioctl op type (always KVMIO), the size of
// the struct used to read or write data, and the op direction (read,
// write, or none).
const _IOC_TYPESHIFT: u32 = 8;
const _IOC_SIZESHIFT: u32 = 16;
const _IOC_DIRSHIFT: u32 = 30;

// Flags for ioctl ops that read or write data, or neither.
const _IOC_NONE: u32 = 0;
const _IOC_WRITE: u32 = 1;
const _IOC_READ: u32 = 2;

const KVM_IOC_TYPE: u32 = KVMIO << _IOC_TYPESHIFT;

// Macro for defining all KVM ioctl operation constants.
macro_rules! define_ioctl_op {
    ($direction:expr, $ioctl_number:expr, $ioctl_size:expr) => {
        (($direction << _IOC_DIRSHIFT) | KVM_IOC_TYPE | $ioctl_number
            | ($ioctl_size << _IOC_SIZESHIFT)) as u64
    };
}

// Define constants from linux/kvm.h
// See: https://www.kernel.org/doc/Documentation/virtual/kvm/api.txt

// Identifies ioctl ops for KVM.
const KVMIO: u32 = 0xAE;

pub const KVM_GET_API_VERSION: u64 = define_ioctl_op!(_IOC_NONE, 0x00, 0);
pub const KVM_CREATE_VM: u64 = define_ioctl_op!(_IOC_NONE, 0x01, 0);
pub const KVM_CHECK_EXTENSION: u64 = define_ioctl_op!(_IOC_NONE, 0x03, 0);
pub const KVM_GET_VCPU_MMAP_SIZE: u64 = define_ioctl_op!(_IOC_NONE, 0x04, 0); /* in bytes */
pub const KVM_CREATE_VCPU: u64 = define_ioctl_op!(_IOC_NONE, 0x41, 0);
pub const KVM_SET_USER_MEMORY_REGION: u64 = define_ioctl_op!(
    _IOC_WRITE,
    0x46,
    size_of::<kvm_userspace_memory_region>() as u32
);
pub const KVM_RUN: u64 = define_ioctl_op!(_IOC_NONE, 0x80, 0);

// Extension capability list.
pub const KVM_CAP_IRQCHIP: u64 = 0;
//const KVM_CAP_HLT:u64 = 1;
//const KVM_CAP_MMU_SHADOW_CACHE_CONTROL:u64 = 2;
pub const KVM_CAP_USER_MEMORY: u64 = 3;
//const KVM_CAP_SET_TSS_ADDR:u64 = 4;
//const KVM_CAP_VAPIC:u64 = 6;
//const KVM_CAP_EXT_CPUID:u64 = 7;
//const KVM_CAP_CLOCKSOURCE:u64 = 8;
//const KVM_CAP_NR_VCPUS:u64 = 9;       /* returns recommended max vcpus per vm */
//const KVM_CAP_NR_MEMSLOTS:u64 = 10;   /* returns max memory slots per vm */
//const KVM_CAP_PIT:u64 = 11;
//const KVM_CAP_NOP_IO_DELAY:u64 = 12;
//const KVM_CAP_PV_MMU:u64 = 13;
//const KVM_CAP_MP_STATE:u64 = 14;
//const KVM_CAP_COALESCED_MMIO:u64 = 15;
//const KVM_CAP_SYNC_MMU:u64 = 16;  /* Changes to host mmap are reflected in guest */
//const KVM_CAP_IOMMU:u64 = 18;
/* Bug in KVM_SET_USER_MEMORY_REGION fixed: */
//const KVM_CAP_DESTROY_MEMORY_REGION_WORKS:u64 =  21;
//const KVM_CAP_USER_NMI:u64 = 22;
