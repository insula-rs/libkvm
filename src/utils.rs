// Copyright (C) 2018, Cloudbase Solutions Srl
//
// Licensed under LGPL version 2 or any later version.

use linux::kvm_bindings::{kvm_cpuid2, kvm_cpuid_entry2};
use std;

pub struct KVMCpuid2Wrapper {
    buf: Vec<u8>,
    kvm_cpuid: *mut kvm_cpuid2,
}

impl KVMCpuid2Wrapper {
    pub fn new(num_entries: u32) -> KVMCpuid2Wrapper {
        let size = std::mem::size_of::<kvm_cpuid2>()
            + std::mem::size_of::<kvm_cpuid_entry2>() * num_entries as usize;
        let buf: Vec<u8> = vec![0; size];
        let kvm_cpuid: &mut kvm_cpuid2 = unsafe { &mut *(buf.as_ptr() as *mut kvm_cpuid2) };
        kvm_cpuid.nent = num_entries;

        KVMCpuid2Wrapper {
            buf: buf,
            kvm_cpuid: kvm_cpuid,
        }
    }

    pub fn from_cpuid_entries(cpuid_entries: &[kvm_cpuid_entry2]) -> KVMCpuid2Wrapper {
        let mut kvm_cpuid = KVMCpuid2Wrapper::new(cpuid_entries.len() as u32);
        kvm_cpuid.copy_cpuid_entries(cpuid_entries);
        kvm_cpuid
    }

    fn copy_cpuid_entries(&mut self, cpuid_entries: &[kvm_cpuid_entry2]) {
        unsafe {
            (*self.kvm_cpuid)
                .entries
                .as_mut_slice((*self.kvm_cpuid).nent as usize)
        }.clone_from_slice(cpuid_entries);
    }

    pub fn to_entries_vec(&self) -> Vec<kvm_cpuid_entry2> {
        unsafe {
            (*self.kvm_cpuid)
                .entries
                .as_slice((*self.kvm_cpuid).nent as usize)
        }.to_vec()
    }

    pub fn as_mut_ptr(&mut self) -> *mut kvm_cpuid2 {
        self.buf.as_mut_ptr() as *mut kvm_cpuid2
    }

    pub fn as_ptr(&self) -> *const kvm_cpuid2 {
        self.buf.as_ptr() as *const kvm_cpuid2
    }
}
