// Copyright (C) 2018, Allison Randal
//
// Licensed under LGPL version 2 or any later version.

//! Traits for working with guest memory.
//!
//! Rather than taking an opinionated stance on how to implement memory
//! allocation for the guest running in a VM, libKVM uses a trait to define the
//! interface it requires to register an allocated memory slot with the VM. KVM
//! is quite flexible in terms of what it will accept as allocated memory,
//! including anonymous memory, Huge Pages, and ordinary files.

/// A single slot of allocated memory for a VM guest.
pub trait MemorySlot {
    /// Returns a unique integer identifier for the memory slot.
    fn slot_id(&self) -> u32;

    /// Returns binary flags for read-only memory or logging dirty pages.
    fn flags(&self) -> u32;

    /// Returns the size of the memory slot, in bytes.
    fn memory_size(&self) -> usize;

    /// Returns the address of the start of the memory block in the guest, as an integer value.
    fn guest_address(&self) -> u64;

    /// Returns the address of the start of the memory slot in the host, as an integer value.
    fn host_address(&self) -> u64;
}
