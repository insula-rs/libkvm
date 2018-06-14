pub trait MemorySlot {
    fn slot_id(&self) -> u32;
    fn flags(&self) -> u32;
    fn memory_size(&self) -> usize;
    fn guest_address(&self) -> u64;
    fn host_address(&self) -> u64;
}

