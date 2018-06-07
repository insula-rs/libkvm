extern crate libc;

use std::fs::File;
use std::io::Error;
use std::os::raw::{c_char};
use std::os::unix::io::{AsRawFd, FromRawFd};
use self::libc::{ioctl, open, O_RDWR, O_CLOEXEC};

use linux::kvm_consts::{KVM_GET_API_VERSION, KVM_CHECK_EXTENSION, KVM_CAP_IRQCHIP, KVM_CAP_USER_MEMORY, KVM_GET_VCPU_MMAP_SIZE, KVM_CREATE_VM};
use vm::*;


pub struct KVMSystem {
    ioctl: File,
}

impl KVMSystem {
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

    pub fn api_version(&self) -> Result<i32, Error> {
        let api_version = unsafe { ioctl(self.ioctl.as_raw_fd(), KVM_GET_API_VERSION, 0) };
        if api_version == 12 {
            return Ok(api_version);
        } else {
            return Err(Error::last_os_error());
        }
    }

    fn check_extension(&self, capability: u64) -> Result<i32, Error> {
        let result = unsafe { ioctl(self.ioctl.as_raw_fd(), KVM_CHECK_EXTENSION, capability) };
        if result > -1 {
            return Ok(result);
        } else {
            return Err(Error::last_os_error());
        }
    }

    pub fn check_cap_irqchip(&self) -> Result<i32, Error> {
        self.check_extension(KVM_CAP_IRQCHIP)
    }

    pub fn check_cap_user_memory(&self) -> Result<i32, Error> {
        self.check_extension(KVM_CAP_USER_MEMORY)
    }

    pub fn get_vcpu_mmap_size(&self) -> Result<usize, Error> {
        let vcpu_mmap_size = unsafe { ioctl(self.ioctl.as_raw_fd(), KVM_GET_VCPU_MMAP_SIZE, 0) };
        if vcpu_mmap_size > 0 {
            return Ok(vcpu_mmap_size as usize);
        } else {
            return Err(Error::last_os_error());
        }
    }

    pub fn create_vm(&self) -> Result<VirtualMachine, Error> {
        let raw_fd = unsafe { ioctl(self.ioctl.as_raw_fd(), KVM_CREATE_VM, 0) };
        if raw_fd < 0 {
            return Err(Error::last_os_error());
        }
        let safe_handle = unsafe { File::from_raw_fd(raw_fd) };

        // Return value is safe because raw file descriptor result is checked
        // and ownership of File struct is consumed by VirtualMachine struct.
        Ok(VirtualMachine::new(safe_handle))
    }
}

