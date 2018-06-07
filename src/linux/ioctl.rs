// Define contstants and macros from ioctl.h
pub const _IOC_NRSHIFT: u32 = 0;
pub const _IOC_TYPESHIFT: u32 = 8;
pub const _IOC_SIZESHIFT: u32 = 16;
pub const _IOC_DIRSHIFT: u32 = 30;
pub const _IOC_NONE: u32 = 0;
pub const _IOC_WRITE: u32 = 1;
pub const _IOC_READ: u32 = 2;

macro_rules! define_ioctl_op {
    ($direction:expr, $ioctl_type:expr, $ioctl_number:expr, $ioctl_size:expr) => (
        (($direction << _IOC_DIRSHIFT) |
        ($ioctl_type << _IOC_TYPESHIFT) |
        ($ioctl_number<< _IOC_NRSHIFT) |
        ($ioctl_size << _IOC_SIZESHIFT)) as u64
    )
}

