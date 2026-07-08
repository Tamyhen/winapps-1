use crate::Result;
use crate::backend::vm::VirtualMachine;
use super::virsh::Virsh;
use super::types::DomainName;

#[derive(Debug, Clone)]
pub struct LibvirtVm {
    virsh: Virsh,
}

impl LibvirtVm {
    pub fn new(domain: impl Into<DomainName>) -> Self {
        Self { virsh: Virsh::new(domain) }
    }
}

impl VirtualMachine for LibvirtVm {
    fn check_depends(&self) -> Result<()> {
        self.virsh.exists()
    }
}
