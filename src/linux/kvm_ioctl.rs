// Copyright (C) 2018, Allison Randal
//
// Licensed under LGPL version 2 or any later version.

//! See: https://www.kernel.org/doc/Documentation/virtual/kvm/api.txt

use std::mem::size_of;

use linux::kvm_bindings::*;

const KVM_IOC_TYPE: u32 = KVMIO << _IOC_TYPESHIFT;

// Macro for defining all KVM ioctl operation constants.
macro_rules! define_ioctl_op {
    ($direction:expr, $ioctl_number:expr, $ioctl_size:expr) => {
        (($direction << _IOC_DIRSHIFT)
            | KVM_IOC_TYPE
            | $ioctl_number
            | ($ioctl_size << _IOC_SIZESHIFT)) as u64
    };
}

// Define constants from linux/kvm.h
//

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
pub const KVM_SET_TSS_ADDR: u64 = define_ioctl_op!(_IOC_NONE, 0x47, 0);
pub const KVM_RUN: u64 = define_ioctl_op!(_IOC_NONE, 0x80, 0);
pub const KVM_GET_REGS: u64 = define_ioctl_op!(_IOC_READ, 0x81, size_of::<kvm_regs>() as u32);
pub const KVM_SET_REGS: u64 = define_ioctl_op!(_IOC_WRITE, 0x82, size_of::<kvm_regs>() as u32);
pub const KVM_GET_SREGS: u64 = define_ioctl_op!(_IOC_READ, 0x83, size_of::<kvm_sregs>() as u32);
pub const KVM_SET_SREGS: u64 = define_ioctl_op!(_IOC_WRITE, 0x84, size_of::<kvm_sregs>() as u32);
pub const KVM_SET_MSRS: u64 = define_ioctl_op!(_IOC_WRITE, 0x89, size_of::<kvm_msrs>() as u32);
pub const KVM_SET_CPUID2: u64 = define_ioctl_op!(_IOC_WRITE, 0x90, size_of::<kvm_cpuid2>() as u32);
