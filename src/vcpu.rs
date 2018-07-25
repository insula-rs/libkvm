// Copyright (C) 2018, Allison Randal
//
// Licensed under LGPL version 2 or any later version.

//! KVM virtual machine operations.

extern crate libc;

use self::libc::ioctl;
use std;
use std::fs::File;
use std::io::Error;
use std::os::unix::io::AsRawFd;

use linux::kvm_bindings::kvm_run;
use linux::kvm_ioctl::KVM_RUN;
use system::KVMSystem;

/// The VirtualCPU module handles KVM virtual CPU operations.
/// It owns the filehandle for these operations.
pub struct VirtualCPU {
    ioctl: File,
    kvm_run: *const kvm_run,
    vcpu_map_size: usize,
}

impl VirtualCPU {
    /// Creates a new `VirtualCPU` from an existing filehandle for
    /// virtual CPU operations.
    pub fn from_file(handle: File) -> Result<Self, Error> {
        let kvm = KVMSystem::new()?;
        let vcpu_map_size = kvm.get_vcpu_mmap_size()?;
        let kvm_run = VirtualCPU::map_kvm_run(&handle, vcpu_map_size)?;

        Ok(VirtualCPU {
            ioctl: handle,
            vcpu_map_size: vcpu_map_size,
            kvm_run: kvm_run,
        })
    }

    fn map_kvm_run(handle: &File, vcpu_map_size: usize) -> Result<*const kvm_run, Error> {
        let address = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                vcpu_map_size,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                handle.as_raw_fd(),
                0,
            )
        };

        if address == libc::MAP_FAILED {
            Err(Error::last_os_error())
        } else {
            Ok(address as *const kvm_run)
        }
    }

    pub fn kvm_run(&self) -> &kvm_run {
        unsafe { &*self.kvm_run }
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

impl Drop for VirtualCPU {
    fn drop(&mut self) {
        let result = unsafe { libc::munmap(self.kvm_run as *mut libc::c_void, self.vcpu_map_size) };
        if result != 0 {
            panic!("munmap failed with: {}", unsafe {
                *libc::__errno_location()
            });
        }
    }
}
