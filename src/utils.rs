// Copyright (C) 2018, Cloudbase Solutions Srl
//
// Licensed under LGPL version 2 or any later version.

use linux::kvm_bindings::{kvm_cpuid2, kvm_cpuid_entry2, kvm_msr_entry, kvm_msr_list, kvm_msrs};
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
        kvm_cpuid.copy_entries(cpuid_entries);
        kvm_cpuid
    }

    fn copy_entries(&mut self, cpuid_entries: &[kvm_cpuid_entry2]) {
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

pub struct KVMMSRSWrapper {
    buf: Vec<u8>,
    kvm_msrs: *mut kvm_msrs,
}

impl KVMMSRSWrapper {
    pub fn new(num_entries: u32) -> KVMMSRSWrapper {
        let size = std::mem::size_of::<kvm_msrs>()
            + std::mem::size_of::<kvm_msr_entry>() * num_entries as usize;
        let buf: Vec<u8> = vec![0; size];
        let kvm_msrs: &mut kvm_msrs = unsafe { &mut *(buf.as_ptr() as *mut kvm_msrs) };
        kvm_msrs.nmsrs = num_entries;

        KVMMSRSWrapper {
            buf: buf,
            kvm_msrs: kvm_msrs,
        }
    }

    pub fn from_msr_indices(msr_indices: &[u32]) -> KVMMSRSWrapper {
        let mut kvm_msrs = KVMMSRSWrapper::new(msr_indices.len() as u32);
        kvm_msrs.copy_indices(msr_indices);
        kvm_msrs
    }

    pub fn from_msr_entries(msr_entries: &[kvm_msr_entry]) -> KVMMSRSWrapper {
        let mut kvm_msrs = KVMMSRSWrapper::new(msr_entries.len() as u32);
        kvm_msrs.copy_entries(msr_entries);
        kvm_msrs
    }

    fn copy_entries(&mut self, msr_entries: &[kvm_msr_entry]) {
        unsafe {
            (*self.kvm_msrs)
                .entries
                .as_mut_slice((*self.kvm_msrs).nmsrs as usize)
        }.clone_from_slice(msr_entries);
    }

    fn copy_indices(&mut self, msr_indices: &[u32]) {
        let entries = unsafe {
            (*self.kvm_msrs)
                .entries
                .as_mut_slice((*self.kvm_msrs).nmsrs as usize)
        };
        for (i, index) in msr_indices.iter().enumerate() {
            entries[i].index = *index;
        }
    }

    pub fn to_entries_vec(&self) -> Vec<kvm_msr_entry> {
        unsafe {
            (*self.kvm_msrs)
                .entries
                .as_slice((*self.kvm_msrs).nmsrs as usize)
        }.to_vec()
    }

    pub fn as_mut_ptr(&mut self) -> *mut kvm_msrs {
        self.buf.as_mut_ptr() as *mut kvm_msrs
    }

    pub fn as_ptr(&self) -> *const kvm_msrs {
        self.buf.as_ptr() as *const kvm_msrs
    }
}

pub struct KVMMSRListWrapper {
    buf: Vec<u8>,
    kvm_msr_list: *mut kvm_msr_list,
}

impl KVMMSRListWrapper {
    pub fn new(num_entries: u32) -> KVMMSRListWrapper {
        let size =
            std::mem::size_of::<kvm_msrs>() + std::mem::size_of::<u32>() * num_entries as usize;
        let buf: Vec<u8> = vec![0; size];
        let kvm_msr_list: &mut kvm_msr_list = unsafe { &mut *(buf.as_ptr() as *mut kvm_msr_list) };
        kvm_msr_list.nmsrs = num_entries;

        KVMMSRListWrapper {
            buf: buf,
            kvm_msr_list: kvm_msr_list,
        }
    }

    pub fn to_indices_vec(&self) -> Vec<u32> {
        unsafe {
            (*self.kvm_msr_list)
                .indices
                .as_slice((*self.kvm_msr_list).nmsrs as usize)
        }.to_vec()
    }

    pub fn as_mut_ptr(&mut self) -> *mut kvm_msr_list {
        self.buf.as_mut_ptr() as *mut kvm_msr_list
    }
}
