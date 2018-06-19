extern crate libc;
extern crate libkvm;

use libkvm::system::*;

use std::io::Error;
use std::ptr::null_mut;

use libkvm::mem::MemorySlot;

pub struct MockSlot {
    id: u32,
    flags: u32,
    size: usize,
    guest_addr: u64,
    host_addr: *mut i32,
}

impl MockSlot {
    pub fn new(memory_size: usize) -> Result<MockSlot, Error> {
        let host_addr = unsafe {
            libc::mmap(
                // request a memory mapping
                null_mut(),                         // kernel chooses where to start memory
                memory_size,                        // requested size in bytes
                libc::PROT_READ | libc::PROT_WRITE, // readable and writable
                libc::MAP_SHARED | libc::MAP_ANONYMOUS, // shared and anonymous
                -1,
                0, // fd and offset ignored for anonymous
            )
        };
        if host_addr == libc::MAP_FAILED {
            return Err(Error::last_os_error());
        }

        Ok(MockSlot {
            size: memory_size,
            id: 0,
            flags: 0,
            guest_addr: 0,
            host_addr: host_addr as *mut i32,
        })
    }
}

impl MemorySlot for MockSlot {
    fn slot_id(&self) -> u32 {
        self.id
    }
    fn flags(&self) -> u32 {
        self.flags
    }
    fn memory_size(&self) -> usize {
        self.size
    }
    fn guest_address(&self) -> u64 {
        self.guest_addr
    }
    fn host_address(&self) -> u64 {
        self.host_addr as u64
    }
}

impl Drop for MockSlot {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.host_addr as *mut libc::c_void, self.size);
        }
    }
}

#[test]
fn create_vm() {
    let sys = KVMSystem::new().expect("failed to create KVM system ioctl");
    let version = sys.api_version().expect("failed to read API version");
    assert_eq!(version, 12);
    let irqchip_cap = sys.check_cap_irqchip()
        .expect("failed to check IRQCHIP capability");
    assert!(irqchip_cap > 0);
    let user_memory_cap = sys.check_cap_user_memory()
        .expect("failed to check user memory capability");
    assert!(user_memory_cap > 0);
    let vm = sys.create_vm().expect("failed to create VM");
    let vcpu = vm.create_vcpu().expect("failed to create VCPU");
    let slot = MockSlot::new(0x20000000).expect("failed to create memory region");
    let set_mem_region = vm.set_user_memory_region(&slot)
        .expect("failed to set user memory region");
    assert!(set_mem_region);
    let running = vcpu.run().expect("failed to run VCPU");
    assert!(running)
}
