// Copyright (C) 2018, Allison Randal
//
// Licensed under LGPL version 2 or any later version.

//! KVM virtual machine operations.

use libc;
use std::fs::File;
use std::io::Error;
use std::os::raw::c_void;
use std::os::unix::io::{AsRawFd, FromRawFd};

use linux::kvm_bindings::*;
use linux::kvm_ioctl::*;
use mem::MemorySlot;
use vcpu::*;

/// The VirtualMachine module handles KVM virtual machine operations.
/// It owns the filehandle for these operations.
pub struct VirtualMachine {
    ioctl: File,
}

impl VirtualMachine {
    /// Creates a new `VirtualMachine` from an existing filehandle for
    /// virtual machine operations.
    pub fn from_file(handle: File) -> Self {
        VirtualMachine { ioctl: handle }
    }

    /// Opens a filehandle for virtual CPU operations, and returns a
    /// `Result`. If the open operation fails, the `Result` unwraps as an
    /// `Error`. If it succeeds, the `Result` unwraps as an instance of
    /// `VCPU` for performing virtual CPU operations.
    ///
    ///     # use libkvm::system::*;
    ///     # use libkvm::vm::*;
    ///     # use libkvm::vcpu::*;
    ///     # let system = KVMSystem::new().expect("failed to connect to KVM");
    ///     # let vm = system.create_vm().expect("failed to create VM");
    ///     let vcpu = vm.create_vcpu().expect("failed to create VCPU");

    pub fn create_vcpu(&self) -> Result<VirtualCPU, Error> {
        let raw_fd = unsafe { libc::ioctl(self.ioctl.as_raw_fd(), KVM_CREATE_VCPU, 0) };
        if raw_fd < 0 {
            return Err(Error::last_os_error());
        }
        let safe_handle = unsafe { File::from_raw_fd(raw_fd) };

        // Return value is safe because raw file descriptor result is checked
        // and ownership of File struct is consumed by VirtualCPU struct.
        let vcpu = VirtualCPU::from_file(safe_handle)?;
        Ok(vcpu)
    }

    /// Register an allocated memory slot as guest memory. The allocated
    /// memory is passed in the `slot` argument, which can be any
    /// instance that implements the `MemorySlot` trait.
    ///
    /// ```ignore
    /// let slot = CustomMemorySlot::new();
    /// let result = vm.set_user_memory_region(&slot);
    /// ```

    pub fn set_user_memory_region(&self, slot: &MemorySlot) -> Result<bool, Error> {
        let region = kvm_userspace_memory_region {
            slot: slot.slot_id(),
            flags: slot.flags(),
            guest_phys_addr: slot.guest_address(),
            memory_size: slot.memory_size() as u64,
            userspace_addr: slot.host_address(),
        };

        let result = unsafe {
            libc::ioctl(
                self.ioctl.as_raw_fd(),
                KVM_SET_USER_MEMORY_REGION,
                &region as *const kvm_userspace_memory_region as *const c_void,
            )
        };
        if result == 0 {
            return Ok(true);
        } else {
            return Err(Error::last_os_error());
        }
    }

    pub fn set_tss_address(&self, tss_address: u32) -> Result<(), Error> {
        let result = unsafe { libc::ioctl(self.ioctl.as_raw_fd(), KVM_SET_TSS_ADDR, tss_address) };
        if result == 0 {
            return Ok(());
        } else {
            return Err(Error::last_os_error());
        }
    }
}
