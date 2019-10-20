//! This crate implements virtual machine instance life cycles and
//! sending commands to virtual machine instances.
//!
//! # Virtual machine instance management
//!
//! There are two popular approaches for managing virtual machine instances:
//! statelessly and statefully.
//! With the stateless approach,
//! you can boot up an instance on demand.
//! When you terminate the instance, its state is gone.
//! This approach is used by [QEMU].
//! With the stateful approach,
//! you must first create an instance configuration and give it a name.
//! Then you can start, pause, resume, and stop it as many times as you please.
//! This approach is used by [libvirt] and [VirtualBox].
//!
//! The stateful approach is mostly useful for
//! instances that must persist across reboots of the host.
//! On the other hand, the stateless approach is
//! easier to work with for ephemeral instances,
//! such as one-shot instances for CI builds.
//!
//! This crate assumes the stateless approach.
//! It does not manage global configuration.
//!
//! [QEMU]: https://www.qemu.org
//! [libvirt]: https://libvirt.org
//! [VirtualBox]: https://www.virtualbox.org

pub mod backend;
