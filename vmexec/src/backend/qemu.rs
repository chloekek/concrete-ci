//! This module provides routines
//! for working with [QEMU] virtual machine instances.
//!
//! Software emulation is not supported by this module;
//! QEMU is always started with hardware virtualization enabled.
//!
//! [QEMU]: https://www.qemu.org

use std::ffi::OsStr;
use std::io;
use std::process;

/// Instruction set architecture (ISA) to use for the guest.
///
/// Only architectures that can be hardware-virtualized
/// on the host are listed in this enumeration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QemuArch
{
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    X86,

    #[cfg(target_arch = "x86_64")]
    X86_64,
}

impl QemuArch
{
    /// There is a separate QEMU executable for each guest ISA.
    /// Return the name of the QEMU executable for this ISA.
    pub fn qemu_executable(&self) -> &'static str
    {
        match self {
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            Self::X86 => "qemu-system-i386",

            #[cfg(target_arch = "x86_64")]
            Self::X86_64 => "qemu-system-x86_64",
        }
    }
}

/// Spawn a QEMU virtual machine instance.
///
/// You must pass two image paths:
/// a base image path and a boot image path.
/// This routine will create a boot image at the given path
/// using *qemu-img create*, using the given base image.
///
/// Finally, this routine will spawn a virtual machine instance
/// and boot it using the boot image.
/// A handle to this QEMU process is returned.
pub fn start_qemu<P: AsRef<OsStr>,
                  Q: AsRef<OsStr>>
                 (arch: QemuArch,
                  base_image: P,
                  boot_image: Q)
                  -> io::Result<process::Child>
{
    process::Command

        ::new("bash")
        .arg("-c")
        .arg(include_str!("qemu.bash"))
        .arg("qemu.bash")

        .arg(arch.qemu_executable())
        .arg(base_image)
        .arg(boot_image)

        .spawn()
}
