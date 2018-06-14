extern crate libkvm;

use libkvm::mem::{MemorySlot};

pub struct MockSlot {
    size: usize,
}

impl MockSlot {
    pub fn new() -> Self {
        MockSlot { size: 55 }
    }
}

impl MemorySlot for MockSlot {
    fn slot_id(&self) -> u32 { 11 }
    fn flags(&self) -> u32 { 22 }
    fn memory_size(&self) -> usize { self.size }
    fn guest_address(&self) -> u64 { 33 }
    fn host_address(&self) -> u64 { 44 }
}

#[test]
fn create_kvm_memory_slot() {
    let slot = MockSlot::new();
    assert_eq!(slot.memory_size(), 55);
    assert_eq!(slot.slot_id(), 11);
    assert_eq!(slot.flags(), 22);
    assert_eq!(slot.guest_address(), 33);
    assert_eq!(slot.host_address(), 44);
}
