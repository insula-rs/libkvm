// Copyright (C) 2018, Allison Randal
//
// Licensed under LGPL version 2 or any later version.

//! Constants and macros for ioctl operations.
//!
//! This is a minimal subset of the constants defined in the C header
//! file `ioctl.h`. It also provides one macro used to calculate the
//! KVM ioctl operation numbers.

// Left shift constant for ioctl op number
pub const _IOC_NRSHIFT: u32 = 0;

// Left shift constant for ioctl op type (always KVMIO)
pub const _IOC_TYPESHIFT: u32 = 8;

// Left shift constant for the size of the struct used to read or
// write data
pub const _IOC_SIZESHIFT: u32 = 16;

// Left shift constant for ioctl op direction (read, write, or none)
pub const _IOC_DIRSHIFT: u32 = 30;

// Flag for ioctl ops that neither read or write data
pub const _IOC_NONE: u32 = 0;

// Flag for ioctl ops that write data
pub const _IOC_WRITE: u32 = 1;

// Flag for ioctl ops that read data
pub const _IOC_READ: u32 = 2;

macro_rules! define_ioctl_op {
    ($direction:expr, $ioctl_type:expr, $ioctl_number:expr, $ioctl_size:expr) => (
        (($direction << _IOC_DIRSHIFT) |
        ($ioctl_type << _IOC_TYPESHIFT) |
        ($ioctl_number<< _IOC_NRSHIFT) |
        ($ioctl_size << _IOC_SIZESHIFT)) as u64
    )
}

