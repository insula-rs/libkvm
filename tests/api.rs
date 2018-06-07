extern crate libkvm;

use libkvm::system::*;

#[test]
fn create_vm() {
    let system = KVMSystem::new().expect("failed to create KVM system ioctl");
    let version = system.api_version().expect("failed to read API version");
    assert_eq!(version, 12);
    let irqchip_cap = system.check_cap_irqchip().expect("failed to check IRQCHIP capability");
    assert!(irqchip_cap > 0);
    let user_memory_cap = system.check_cap_user_memory().expect("failed to check user memory capability");
    assert!(user_memory_cap > 0);
    let vm = system.create_vm().expect("failed to create VM");
    let vcpu = vm.create_vcpu().expect("failed to create VCPU");
    let set_mem_region = vm.set_user_memory_region().expect("failed to set user memory region");
    assert!(set_mem_region);
    let running = vcpu.run().expect("failed to run VCPU");
    assert!(running)
}
