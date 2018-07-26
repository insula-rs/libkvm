// Copyright (C) 2018, Cloudbase Solutions Srl
//
// Licensed under LGPL version 2 or any later version.

extern crate libc;
extern crate libkvm;

use libkvm::linux::kvm_bindings::*;
use libkvm::mem::MemorySlot;
use libkvm::system::*;
use libkvm::vcpu::VirtualCPU;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Write};
use std::path::PathBuf;

const CPUID_EXT_HYPERVISOR: u32 = 1 << 31;

const PDE64_PRESENT: u64 = 1;
const PDE64_RW: u64 = 1 << 1;
const PDE64_USER: u64 = 1 << 2;
const PDE64_PS: u64 = 1 << 7;
const CR4_PAE: u64 = 1 << 5;

const CR0_PE: u64 = 1;
const CR0_MP: u64 = 1 << 1;
const CR0_ET: u64 = 1 << 4;
const CR0_NE: u64 = 1 << 5;
const CR0_WP: u64 = 1 << 16;
const CR0_AM: u64 = 1 << 18;
const CR0_PG: u64 = 1 << 31;
const EFER_LME: u64 = 1 << 8;
const EFER_LMA: u64 = 1 << 10;

fn main() {
    let kvm = KVMSystem::new().unwrap();
    let api = kvm.api_version().unwrap();
    println!("KVM API version: {}", api);

    let vm = kvm.create_vm().unwrap();
    if kvm.check_cap_set_tss_address().unwrap() > 0 {
        println!("Setting TSS address");
        vm.set_tss_address(0xfffbd000).unwrap();
    }

    let mem_size = 0x100000;

    let mut mem = MmapMemorySlot::new(mem_size, 0);
    vm.set_user_memory_region(&mem).unwrap();

    read_payload(&mut mem);

    let mut vcpu = vm.create_vcpu().unwrap();

    setup_long_mode(&vcpu, &mem);
    setup_cpuid(&kvm, &vcpu);
    setup_msrs(&kvm, &vcpu);

    loop {
        vcpu.run().unwrap();
        let mut kvm_run = vcpu.kvm_run_mut();
        match kvm_run.exit_reason {
            KVM_EXIT_HLT => {
                println!("Halt");
                break;
            }
            KVM_EXIT_MMIO => {
                handle_mmio(&mut kvm_run);
            }
            KVM_EXIT_IO => {
                handle_io_port(&kvm_run);
            }
            _ => {
                panic!("Not supported exit reason: {}", kvm_run.exit_reason);
            }
        }
    }
}

fn handle_io_port(kvm_run: &kvm_run) {
    let io = unsafe { &kvm_run.__bindgen_anon_1.io };

    if io.direction == KVM_EXIT_IO_OUT as u8 && io.port == 42 {
        let data_addr = kvm_run as *const _ as u64 + io.data_offset;
        let data = unsafe { std::slice::from_raw_parts(data_addr as *const u8, io.size as usize) };
        io::stdout().write(data).unwrap();
    }
}

fn handle_mmio(kvm_run: &mut kvm_run) {
    let mmio = unsafe { kvm_run.__bindgen_anon_1.mmio };

    if mmio.len == 8 {
        if mmio.is_write == 0 {
            let data = &mmio.data as *const _ as *mut u64;
            unsafe {
                *data = 0x1000;
                println!("MMIO read: 0x{:x}", *data);
            }
        } else {
            let value = unsafe { *(&mmio.data as *const _ as *const u64) };
            println!("MMIO write: 0x{:x}", value);
        }
    }
}

fn setup_cpuid(kvm: &KVMSystem, vcpu: &VirtualCPU) {
    let mut kvm_cpuid_entries = kvm.get_supported_cpuid().unwrap();

    let i = kvm_cpuid_entries
        .iter()
        .position(|&r| r.function == 0x40000000)
        .unwrap();

    let mut id_reg_values: [u32; 3] = [0; 3];
    let id = "libwhp\0";
    unsafe {
        std::ptr::copy_nonoverlapping(id.as_ptr(), id_reg_values.as_mut_ptr() as *mut u8, id.len());
    }
    kvm_cpuid_entries[i].ebx = id_reg_values[0];
    kvm_cpuid_entries[i].ecx = id_reg_values[1];
    kvm_cpuid_entries[i].edx = id_reg_values[2];

    let i = kvm_cpuid_entries
        .iter()
        .position(|&r| r.function == 1)
        .unwrap();

    kvm_cpuid_entries[i].ecx |= CPUID_EXT_HYPERVISOR;

    vcpu.set_cpuid(&kvm_cpuid_entries).unwrap();
}

fn setup_msrs(kvm: &KVMSystem, vcpu: &VirtualCPU) {
    let msr_list = kvm.get_msr_index_list().unwrap();

    let msr_entries = msr_list
        .iter()
        .map(|i| kvm_msr_entry {
            index: *i,
            data: 0,
            reserved: 0,
        })
        .collect::<Vec<_>>();

    vcpu.set_msrs(&msr_entries).unwrap();
}

fn setup_long_mode(vcpu: &VirtualCPU, mem: &MmapMemorySlot) {
    let mut sregs = vcpu.get_kvm_sregs().unwrap();
    let mem_addr = mem.host_address();

    let pml4_addr: u64 = 0x2000;
    let pdpt_addr: u64 = 0x3000;
    let pd_addr: u64 = 0x4000;
    let pml4: u64 = mem_addr + pml4_addr;
    let pdpt: u64 = mem_addr + pdpt_addr;
    let pd: u64 = mem_addr + pd_addr;

    unsafe {
        *(pml4 as *mut u64) = PDE64_PRESENT | PDE64_RW | PDE64_USER | pdpt_addr;
        *(pdpt as *mut u64) = PDE64_PRESENT | PDE64_RW | PDE64_USER | pd_addr;
        *(pd as *mut u64) = PDE64_PRESENT | PDE64_RW | PDE64_USER | PDE64_PS;
    }

    sregs.cr3 = pml4_addr;
    sregs.cr4 = CR4_PAE;
    sregs.cr0 = CR0_PE | CR0_MP | CR0_ET | CR0_NE | CR0_WP | CR0_AM | CR0_PG;
    sregs.efer = EFER_LME | EFER_LMA;

    let mut seg = kvm_segment {
        base: 0,
        limit: 0xffffffff,
        selector: 1 << 3,
        present: 1,
        type_: 11,
        dpl: 0,
        db: 0,
        s: 1,
        l: 1,
        g: 1,
        avl: 0,
        padding: 0,
        unusable: 0,
    };

    sregs.cs = seg;

    seg.type_ = 3;
    seg.selector = 2 << 3;
    sregs.ds = seg;
    sregs.es = seg;
    sregs.fs = seg;
    sregs.gs = seg;
    sregs.ss = seg;

    vcpu.set_kvm_sregs(&sregs).unwrap();

    let mut regs = vcpu.get_kvm_regs().unwrap();
    regs.rflags = 2;
    regs.rip = 0;
    regs.rsp = mem.memory_size() as u64;

    vcpu.set_kvm_regs(&regs).unwrap();
}

fn read_payload(mem: &mut MmapMemorySlot) {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("examples");
    p.push("payload");
    p.push("payload.img");

    let mut f = File::open(&p).expect(&format!(
        "Cannot find \"{}\". Run \"make\" in the same folder to build it",
        &p.to_str().unwrap()
    ));
    f.read(mem.as_slice_mut()).unwrap();
}

struct MmapMemorySlot {
    memory_size: usize,
    guest_address: u64,
    host_address: *mut libc::c_void,
}

impl MmapMemorySlot {
    pub fn new(memory_size: usize, guest_address: u64) -> MmapMemorySlot {
        let host_address = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                memory_size,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_NORESERVE,
                -1,
                0,
            )
        };

        if host_address == libc::MAP_FAILED {
            panic!("mmapp failed with: {}", unsafe {
                *libc::__errno_location()
            });
        }

        let result = unsafe { libc::madvise(host_address, memory_size, libc::MADV_MERGEABLE) };
        if result == -1 {
            panic!("madvise failed with: {}", unsafe {
                *libc::__errno_location()
            });
        }

        MmapMemorySlot {
            memory_size: memory_size,
            guest_address: guest_address,
            host_address,
        }
    }

    fn as_slice_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.host_address as *mut u8, self.memory_size) }
    }
}

impl MemorySlot for MmapMemorySlot {
    fn slot_id(&self) -> u32 {
        0
    }

    fn flags(&self) -> u32 {
        0
    }

    fn memory_size(&self) -> usize {
        self.memory_size
    }

    fn guest_address(&self) -> u64 {
        self.guest_address
    }

    fn host_address(&self) -> u64 {
        self.host_address as u64
    }
}

impl Drop for MmapMemorySlot {
    fn drop(&mut self) {
        let result = unsafe { libc::munmap(self.host_address, self.memory_size) };
        if result != 0 {
            panic!("munmap failed with: {}", unsafe {
                *libc::__errno_location()
            });
        }
    }
}
