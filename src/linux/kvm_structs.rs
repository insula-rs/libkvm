// Copyright (C) 2018, Allison Randal
//
// Licensed under LGPL version 2 or any later version.

//! Structs for KVM ioctl operations.
//!
//! This is a minimal subset of the structs defined in the C header
//! file `linux/kvm.h`.

/// for KVM_SET_USER_MEMORY_REGION
#[repr(C)]
pub struct kvm_userspace_memory_region {
    pub slot: u32,
    pub flags: u32,
    pub guest_phys_addr: u64,
    pub memory_size: u64,    /* bytes */
    pub userspace_addr: u64, /* start of the userspace allocated memory */
}

/// for KVM_GET_SREGS and KVM_SET_SREGS (arch: x86)
#[repr(C)]
pub struct kvm_sregs {
    cs: kvm_segment,
    ds: kvm_segment,
    es: kvm_segment,
    fs: kvm_segment,
    gs: kvm_segment,
    ss: kvm_segment,
    tr: kvm_segment,
    ldt: kvm_segment,
    gdt: kvm_dtable,
    idt: kvm_dtable,
    cr0: u64,
    cr2: u64,
    cr3: u64,
    cr4: u64,
    cr8: u64,
    efer: u64,
    apic_base: u64,
    interrupt_bitmap: [u64; 4usize],
}

/// used within kvm_sregs
#[repr(C)]
pub struct kvm_segment {
    base: u64,
    limit: u32,
    selector: u16,
    type_: u8,
    present: u8,
    dpl: u8,
    db: u8,
    s: u8,
    l: u8,
    g: u8,
    avl: u8,
    unusable: u8,
    padding: u8,
}

/// used within kvm_sregs
#[repr(C)]
pub struct kvm_dtable {
    base: u64,
    limit: u16,
    padding: [u16; 3usize],
}
