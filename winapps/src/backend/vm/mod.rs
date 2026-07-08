mod libvirt;

pub(crate) use libvirt::virsh::Virsh;
pub(crate) use libvirt::LibvirtVm; // Placeholder for the adapter
pub use libvirt::virsh::Virsh as LibvirtDriver; // Example pattern
