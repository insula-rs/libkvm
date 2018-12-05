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

use linux::kvm_bindings::{
    kvm_cpuid_entry2, kvm_fpu, kvm_lapic_state, kvm_msr_entry, kvm_regs, kvm_run, kvm_sregs,
};
use linux::kvm_ioctl::{
    KVM_GET_CPUID2, KVM_GET_FPU, KVM_GET_LAPIC, KVM_GET_MSRS, KVM_GET_REGS, KVM_GET_SREGS, KVM_RUN,
    KVM_SET_CPUID2, KVM_SET_FPU, KVM_SET_LAPIC, KVM_SET_MSRS, KVM_SET_REGS, KVM_SET_SREGS,
};
use system::KVMSystem;
use utils::{KVMCpuid2Wrapper, KVMMSRSWrapper};

/// The VirtualCPU module handles KVM virtual CPU operations.
/// It owns the filehandle for these operations.
pub struct VirtualCPU {
    ioctl: File,
    kvm_run: *mut kvm_run,
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

    fn map_kvm_run(handle: &File, vcpu_map_size: usize) -> Result<*mut kvm_run, Error> {
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
            Ok(address as *mut kvm_run)
        }
    }

    pub fn kvm_run(&self) -> &kvm_run {
        unsafe { &*self.kvm_run }
    }

    pub fn kvm_run_mut(&mut self) -> &mut kvm_run {
        unsafe { &mut *self.kvm_run }
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

    pub fn get_kvm_regs(&self) -> Result<kvm_regs, Error> {
        let mut regs: kvm_regs = Default::default();
        let result = unsafe { libc::ioctl(self.ioctl.as_raw_fd(), KVM_GET_REGS, &mut regs) };
        if result == 0 {
            return Ok(regs);
        } else {
            return Err(Error::last_os_error());
        }
    }

    pub fn set_kvm_regs(&self, regs: &kvm_regs) -> Result<(), Error> {
        let result = unsafe { libc::ioctl(self.ioctl.as_raw_fd(), KVM_SET_REGS, regs) };
        if result == 0 {
            return Ok(());
        } else {
            return Err(Error::last_os_error());
        }
    }

    pub fn get_kvm_sregs(&self) -> Result<kvm_sregs, Error> {
        let mut sregs: kvm_sregs = Default::default();
        let result = unsafe { libc::ioctl(self.ioctl.as_raw_fd(), KVM_GET_SREGS, &mut sregs) };
        if result == 0 {
            return Ok(sregs);
        } else {
            return Err(Error::last_os_error());
        }
    }

    pub fn set_kvm_sregs(&self, sregs: &kvm_sregs) -> Result<(), Error> {
        let result = unsafe { libc::ioctl(self.ioctl.as_raw_fd(), KVM_SET_SREGS, sregs) };
        if result == 0 {
            return Ok(());
        } else {
            return Err(Error::last_os_error());
        }
    }

    pub fn get_fpu(&self) -> Result<kvm_fpu, Error> {
        let mut fpu: kvm_fpu = Default::default();
        let result = unsafe { libc::ioctl(self.ioctl.as_raw_fd(), KVM_GET_FPU, &mut fpu) };
        if result == 0 {
            return Ok(fpu);
        } else {
            return Err(Error::last_os_error());
        }
    }

    pub fn set_fpu(&self, fpu: &kvm_fpu) -> Result<(), Error> {
        let result = unsafe { libc::ioctl(self.ioctl.as_raw_fd(), KVM_SET_FPU, fpu) };
        if result == 0 {
            return Ok(());
        } else {
            return Err(Error::last_os_error());
        }
    }

    pub fn get_cpuid(&self) -> Result<Vec<kvm_cpuid_entry2>, Error> {
        const MAX_KVM_CPUID_ENTRIES: u32 = 256;
        let mut kvm_cpuid = KVMCpuid2Wrapper::new(MAX_KVM_CPUID_ENTRIES);

        let result = unsafe {
            ioctl(
                self.ioctl.as_raw_fd(),
                KVM_GET_CPUID2,
                kvm_cpuid.as_mut_ptr(),
            )
        };
        if result == 0 {
            return Ok(kvm_cpuid.to_entries_vec());
        } else {
            return Err(Error::last_os_error());
        }
    }

    pub fn set_cpuid(&self, cpuid_entries: &[kvm_cpuid_entry2]) -> Result<(), Error> {
        let kvm_cpuid = KVMCpuid2Wrapper::from_cpuid_entries(cpuid_entries);
        let result =
            unsafe { libc::ioctl(self.ioctl.as_raw_fd(), KVM_SET_CPUID2, kvm_cpuid.as_ptr()) };
        if result == 0 {
            return Ok(());
        } else {
            return Err(Error::last_os_error());
        }
    }

    pub fn get_msrs(&self, msr_indices: &[u32]) -> Result<Vec<kvm_msr_entry>, Error> {
        let mut kvm_msrs = KVMMSRSWrapper::from_msr_indices(msr_indices);

        let result = unsafe { ioctl(self.ioctl.as_raw_fd(), KVM_GET_MSRS, kvm_msrs.as_mut_ptr()) };
        if result >= 0 {
            return Ok(kvm_msrs.to_entries_vec());
        } else {
            return Err(Error::last_os_error());
        }
    }

    pub fn set_msrs(&self, msr_entries: &[kvm_msr_entry]) -> Result<(), Error> {
        let kvm_msrs = KVMMSRSWrapper::from_msr_entries(msr_entries);

        let result =
            unsafe { libc::ioctl(self.ioctl.as_raw_fd(), KVM_SET_MSRS, kvm_msrs.as_ptr()) };
        // Returns the number of msr entries written
        if result >= 0 {
            return Ok(());
        } else {
            return Err(Error::last_os_error());
        }
    }

    pub fn get_lapic(&self) -> Result<kvm_lapic_state, Error> {
        let mut klapic: kvm_lapic_state = Default::default();

        let result = unsafe { ioctl(self.ioctl.as_raw_fd(), KVM_GET_LAPIC, &mut klapic) };
        if result >= 0 {
            return Ok(klapic);
        } else {
            return Err(Error::last_os_error());
        }
    }

    pub fn set_lapic(&self, klapic: &kvm_lapic_state) -> Result<(), Error> {
        let result = unsafe { libc::ioctl(self.ioctl.as_raw_fd(), KVM_SET_LAPIC, klapic) };
        if result >= 0 {
            return Ok(());
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
