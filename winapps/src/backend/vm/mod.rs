use crate::Result;

pub trait VirtualMachine {
    fn check_depends(&self) -> Result<()>;
}

mod libvirt;

// Expose the adapter internally to the backend layer
pub(crate) use libvirt::adapter::LibvirtVm;
