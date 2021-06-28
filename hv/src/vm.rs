use std::ffi::c_void;

use crate::{call, sys, Addr, Error, GPAddr, Size, Vcpu};

bitflags::bitflags! {
    /// Guest physical memory region permissions.
    pub struct Memory: u32 {
        const READ = sys::HV_MEMORY_READ;
        const WRITE = sys::HV_MEMORY_WRITE;
        const EXEC = sys::HV_MEMORY_EXEC;
    }
}

/// Vm is an entry point to Hypervisor Framework.
#[derive(Debug)]
pub struct Vm;

impl Vm {
    /// Creates a vCPU instance for the current thread.
    pub fn create_cpu() -> Result<Vcpu, Error> {
        Vcpu::new()
    }

    /// Maps a region in the virtual address space of the current task into the guest physical
    /// address space of the VM.
    ///
    /// The host memory must encompass a single VM region, typically allocated with `mmap` or
    /// `mach_vm_allocate` instead of `malloc`. [1]
    ///
    /// # Arguments
    /// * `uva` - Page aligned virtual address in the current task.
    /// * `gpa` - Page aligned address in the guest physical address space.
    /// * `size` - Size in bytes of the region to be mapped.
    /// * `flags` - READ, WRITE and EXECUTE permissions of the region
    ///
    /// [1]: https://developer.apple.com/documentation/hypervisor/1441187-hv_vm_map
    ///
    pub fn map(uva: Addr, gpa: GPAddr, size: Size, flags: Memory) -> Result<(), Error> {
        call!(sys::hv_vm_map(
            uva as *mut c_void,
            gpa,
            size,
            flags.bits().into()
        ))
    }

    /// Unmaps a region in the guest physical address space of the VM
    ///
    /// # Arguments
    /// * `gpa` - Page aligned address in the guest physical address space.
    /// * `size` - Size in bytes of the region to be unmapped.
    pub fn unmap(gpa: GPAddr, size: Size) -> Result<(), Error> {
        call!(sys::hv_vm_unmap(gpa, size))
    }

    /// Modifies the permissions of a region in the guest physical address space of the VM.
    ///
    /// # Arguments
    /// * `gpa` - Page aligned address in the guest physical address space.
    /// * `size` - Size in bytes of the region to be modified.
    /// * `flags` - New READ, WRITE and EXECUTE permissions of the region.
    pub fn protect(gpa: GPAddr, size: Size, flags: Memory) -> Result<(), Error> {
        call!(sys::hv_vm_protect(gpa, size, flags.bits().into()))
    }

    /// Destroys the VM instance associated with the current process.
    pub fn destroy() -> Result<(), Error> {
        call!(sys::hv_vm_destroy())
    }
}