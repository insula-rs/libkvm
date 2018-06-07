extern crate libc;

use std::fs::File;
use std::io::Error;
use std::os::unix::io::{AsRawFd};
use self::libc::{ioctl};

use linux::kvm_consts::{KVM_RUN};

pub struct VirtualCPU {
    ioctl: File,
}

impl VirtualCPU {
    pub fn new(handle: File) -> Self {
        VirtualCPU { ioctl: handle }
    }

    pub fn run(&self) -> Result<bool, Error> {
        let result = unsafe { ioctl(self.ioctl.as_raw_fd(), KVM_RUN, 0) };
        if result == 0 {
            return Ok(true);
        } else {
            return Err(Error::last_os_error());
        }
    }
}

