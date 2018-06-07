use std::io::Error;
use std::ptr::null_mut;
use libc;

use linux::kvm_structs::{kvm_userspace_memory_region};

pub struct Slot {
    slot_id: u32,
    flags: u32,
    memory_size: usize,
    guest_addr: u64,
    host_addr: *mut i32,
}

impl Slot {
    pub fn new(memory_size: usize) -> Result<Slot, Error> {
        let host_addr = unsafe {
            libc::mmap( // request a memory mapping
                null_mut(), // kernel chooses where to start memory
                memory_size, // requested size in bytes
                libc::PROT_READ | libc::PROT_WRITE, // readable and writable
                libc::MAP_SHARED | libc::MAP_ANONYMOUS, // shared and anonymous
                -1, 0 // fd and offset ignored for anonymous
            )
        };
        if host_addr == libc::MAP_FAILED {
            return Err(Error::last_os_error());
        }

        Ok(Slot {
            memory_size,
            slot_id: 0,
            flags: 0,
            guest_addr: 0,
            host_addr: host_addr as *mut i32,
        })
    }

    pub fn as_kvm_memory_region(&self) -> kvm_userspace_memory_region {
        kvm_userspace_memory_region {
            slot: self.slot_id,
            flags: self.flags,
            guest_phys_addr: self.guest_addr,
            memory_size: self.memory_size as u64,
            userspace_addr: self.host_addr as u64,
        }
    }
}

impl Drop for Slot {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.host_addr as *mut libc::c_void, self.memory_size);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_kvm_memory_region() {
        let slot = Slot::new(500).unwrap();
        let region = slot.as_kvm_memory_region();
        assert_eq!(region.memory_size, 500);
        assert_eq!(region.slot, 0);
        assert_eq!(region.flags, 0);
        assert_eq!(region.guest_phys_addr, 0);
    }
}
