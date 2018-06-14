use std::fs::File;
use std::io::Error;
use std::os::raw::c_void;
use std::os::unix::io::{AsRawFd, FromRawFd};
use libc;

use linux::kvm_consts::*;
use linux::kvm_structs::{kvm_userspace_memory_region};
use mem::{MemorySlot};
use vcpu::*;

pub struct VirtualMachine {
    ioctl: File,
}

impl VirtualMachine {
    pub fn new(handle: File) -> Self {
        VirtualMachine { ioctl: handle }
    }

    pub fn create_vcpu(&self) -> Result<VirtualCPU, Error> {
        let raw_fd = unsafe { libc::ioctl(self.ioctl.as_raw_fd(), KVM_CREATE_VCPU, 0) };
        if raw_fd < 0 {
            return Err(Error::last_os_error());
        }
        let safe_handle = unsafe { File::from_raw_fd(raw_fd) };

        // Return value is safe because raw file descriptor result is checked
        // and ownership of File struct is consumed by VirtualCPU struct.
        Ok(VirtualCPU::new(safe_handle))
    }

    pub fn set_user_memory_region(&self, slot: &MemorySlot) -> Result<bool, Error> {
        let region = kvm_userspace_memory_region {
            slot: slot.slot_id(),
            flags: slot.flags(),
            guest_phys_addr: slot.guest_address(),
            memory_size: slot.memory_size() as u64,
            userspace_addr: slot.host_address(),
        };

        let result = unsafe { libc::ioctl(self.ioctl.as_raw_fd(), KVM_SET_USER_MEMORY_REGION, &region as *const kvm_userspace_memory_region as *const c_void) };
        if result == 0 {
            return Ok(true);
        } else {
            return Err(Error::last_os_error());
        }
    }

}

