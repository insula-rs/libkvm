// Copyright (C) 2018, Allison Randal
//
// Licensed under LGPL version 2 or any later version.

//! KVM system operations.
//!
//! The [KVM API](https://www.kernel.org/doc/Documentation/virtual/kvm/api.txt)
//! divides operations into three classes: system operations, virtual
//! machine operations, and virtual CPU operations. Each class of
//! operation must be made on an ioctl filehandle created specifically
//! for that class of operations. So in Rust, we divide the operations
//! into separate modules, and each module owns the filehandle for the
//! class of operations it performs.

extern crate libc;

use self::libc::{ioctl, open, O_CLOEXEC, O_RDWR};
use std;
use std::fs::File;
use std::io::Error;
use std::os::raw::c_char;
use std::os::unix::io::{AsRawFd, FromRawFd};
use utils::KVMCpuid2Wrapper;

use linux::kvm_bindings::{
    kvm_cpuid_entry2, kvm_msr_list, KVM_CAP_IRQCHIP, KVM_CAP_SET_TSS_ADDR, KVM_CAP_USER_MEMORY,
};

use linux::kvm_ioctl::{
    KVM_CHECK_EXTENSION, KVM_CREATE_VM, KVM_GET_API_VERSION, KVM_GET_MSR_FEATURE_INDEX_LIST,
    KVM_GET_MSR_INDEX_LIST, KVM_GET_SUPPORTED_CPUID, KVM_GET_VCPU_MMAP_SIZE,
};
use vm::*;

/// The KVMSystem module handles KVM system operations. It creates and
/// owns the initial filehandle on `/dev/kvm`.
pub struct KVMSystem {
    ioctl: File,
}

impl KVMSystem {
    /// Opens a filehandle to `/dev/kvm`, and returns a `Result`. If the open
    /// operation fails, the `Result` unwraps as an `Error`. If it succeeds, the
    /// `Result` unwraps as an instance of `KVMSystem` for performing KVM system
    /// operations.
    ///
    ///     # use libkvm::system::*;
    ///     let system = KVMSystem::new().expect("failed to connect to KVM");

    pub fn new() -> Result<KVMSystem, Error> {
        let raw_fd = unsafe { open("/dev/kvm\0".as_ptr() as *const c_char, O_RDWR | O_CLOEXEC) };
        if raw_fd < 0 {
            return Err(Error::last_os_error());
        }
        let safe_handle = unsafe { File::from_raw_fd(raw_fd) };

        // Return value is safe because raw file descriptor result is checked
        // and ownership of File struct is consumed by KVMSystem struct.
        Ok(KVMSystem { ioctl: safe_handle })
    }

    /// Fetches the API version from KVM. At this time, the stable KVM API version
    /// is 12, and this is not expected to change, so any other value is considered
    /// an error. Some earlier versions of the Linux Kernel (2.6.20 and 2.6.21)
    /// report earlier API versions, however these are not documented and not
    /// supported. Returns a `Result`, which unwraps as the integer value 12 if
    /// successful, and an `Error` value otherwise. Applications should refuse to
    /// run if `api_version` does not return 12. At version 12, all operations
    /// tagged as 'basic' will be available in KVM.
    ///
    ///     # use libkvm::system::*;
    ///     # let system = KVMSystem::new().expect("failed to connect to KVM");
    ///     let version = system.api_version().expect("version number is not 12");

    pub fn api_version(&self) -> Result<i32, Error> {
        let api_version = unsafe { ioctl(self.ioctl.as_raw_fd(), KVM_GET_API_VERSION, 0) };
        if api_version == 12 {
            return Ok(api_version);
        } else {
            return Err(Error::last_os_error());
        }
    }

    pub fn get_msr_index_list(&self) -> Result<Vec<u32>, Error> {
        self.get_msr_list(KVM_GET_MSR_INDEX_LIST)
    }

    pub fn get_msr_feature_index_list(&self) -> Result<Vec<u32>, Error> {
        self.get_msr_list(KVM_GET_MSR_FEATURE_INDEX_LIST)
    }

    fn get_msr_list(&self, ioctl_request: u64) -> Result<Vec<u32>, Error> {
        const MAX_KVM_MSR_ENTRIES: usize = 256;

        let size =
            std::mem::size_of::<kvm_msr_list>() + std::mem::size_of::<u32>() * MAX_KVM_MSR_ENTRIES;
        let mut buf: Vec<u8> = vec![0; size];
        let kvm_msr_list: &mut kvm_msr_list = unsafe { &mut *(buf.as_ptr() as *mut kvm_msr_list) };
        kvm_msr_list.nmsrs = MAX_KVM_MSR_ENTRIES as u32;

        let result = unsafe { ioctl(self.ioctl.as_raw_fd(), ioctl_request, buf.as_mut_ptr()) };
        if result == 0 {
            return Ok(
                unsafe { kvm_msr_list.indices.as_slice(kvm_msr_list.nmsrs as usize) }.to_vec(),
            );
        } else {
            return Err(Error::last_os_error());
        }
    }

    pub fn get_supported_cpuid(&self) -> Result<Vec<kvm_cpuid_entry2>, Error> {
        const MAX_KVM_CPUID_ENTRIES: u32 = 256;
        let mut kvm_cpuid = KVMCpuid2Wrapper::new(MAX_KVM_CPUID_ENTRIES);

        let result = unsafe {
            ioctl(
                self.ioctl.as_raw_fd(),
                KVM_GET_SUPPORTED_CPUID,
                kvm_cpuid.as_mut_ptr(),
            )
        };
        if result == 0 {
            return Ok(kvm_cpuid.to_entries_vec());
        } else {
            return Err(Error::last_os_error());
        }
    }

    fn check_extension(&self, capability: u32) -> Result<i32, Error> {
        let result = unsafe { ioctl(self.ioctl.as_raw_fd(), KVM_CHECK_EXTENSION, capability) };
        if result > -1 {
            return Ok(result);
        } else {
            return Err(Error::last_os_error());
        }
    }

    /// Check whether this KVM API supports the capability to create
    /// interrupt controller models in the Kernel.
    ///
    ///     # use libkvm::system::*;
    ///     # let system = KVMSystem::new().expect("failed to connect to KVM");
    ///     let result = system.check_cap_irqchip();

    pub fn check_cap_irqchip(&self) -> Result<i32, Error> {
        self.check_extension(KVM_CAP_IRQCHIP)
    }

    /// Check whether this KVM API supports the capability for fine
    /// grained control over memory allocation for guests.
    ///
    ///     # use libkvm::system::*;
    ///     # let system = KVMSystem::new().expect("failed to connect to KVM");
    ///     let result = system.check_cap_user_memory();

    pub fn check_cap_user_memory(&self) -> Result<i32, Error> {
        self.check_extension(KVM_CAP_USER_MEMORY)
    }

    pub fn check_cap_set_tss_address(&self) -> Result<i32, Error> {
        self.check_extension(KVM_CAP_SET_TSS_ADDR)
    }

    /// Fetch the size of the shared memory region that KVM uses to
    /// communicate with userspace for the `run` operation.
    ///
    ///     # use libkvm::system::*;
    ///     # let system = KVMSystem::new().expect("failed to connect to KVM");
    ///     let result = system.get_vcpu_mmap_size();

    pub fn get_vcpu_mmap_size(&self) -> Result<usize, Error> {
        let vcpu_mmap_size = unsafe { ioctl(self.ioctl.as_raw_fd(), KVM_GET_VCPU_MMAP_SIZE, 0) };
        if vcpu_mmap_size > 0 {
            return Ok(vcpu_mmap_size as usize);
        } else {
            return Err(Error::last_os_error());
        }
    }

    /// Opens a filehandle for virtual machine operations, and returns a
    /// `Result`. If the open operation fails, the `Result` unwraps as an
    /// `Error`. If it succeeds, the `Result` unwraps as an instance of
    /// `VM` for performing virtual machine operations.
    ///
    ///     # use libkvm::system::*;
    ///     # use libkvm::vm::*;
    ///     # let system = KVMSystem::new().expect("failed to connect to KVM");
    ///     let vm = system.create_vm().expect("failed to create VM");

    pub fn create_vm(&self) -> Result<VirtualMachine, Error> {
        let raw_fd = unsafe { ioctl(self.ioctl.as_raw_fd(), KVM_CREATE_VM, 0) };
        if raw_fd < 0 {
            return Err(Error::last_os_error());
        }
        let safe_handle = unsafe { File::from_raw_fd(raw_fd) };

        // Return value is safe because raw file descriptor result is checked
        // and ownership of File struct is consumed by VirtualMachine struct.
        Ok(VirtualMachine::from_file(safe_handle))
    }
}
