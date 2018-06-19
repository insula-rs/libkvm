// Copyright (C) 2018, Allison Randal
//
// Licensed under LGPL version 2 or any later version.

//! KVM virtual machine operations.

extern crate libc;

use self::libc::ioctl;
use std::fs::File;
use std::io::Error;
use std::os::unix::io::AsRawFd;

use linux::kvm_consts::KVM_RUN;

/// The VirtualCPU module handles KVM virtual CPU operations.
/// It owns the filehandle for these operations.
pub struct VirtualCPU {
    ioctl: File,
}

impl VirtualCPU {
    /// Creates a new `VirtualCPU` from an existing filehandle for
    /// virtual CPU operations.
    pub fn from_file(handle: File) -> Self {
        VirtualCPU { ioctl: handle }
    }

    /// Runs the guest virtual CPU, and returns a `Result`. If the run
    /// operation fails, the `Result` unwraps as an `Error`. If it succeeds,
    /// the `Result` unwraps as a boolean true value.
    ///
    /// ```ignore
    /// let result = vcpu.run();
    /// ```
    pub fn run(&self) -> Result<bool, Error> {
        let result = unsafe { ioctl(self.ioctl.as_raw_fd(), KVM_RUN, 0) };
        if result == 0 {
            return Ok(true);
        } else {
            return Err(Error::last_os_error());
        }
    }
}
