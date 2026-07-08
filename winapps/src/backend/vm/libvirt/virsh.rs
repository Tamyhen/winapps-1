use crate::command::Command;
use crate::Result;
use super::types::{DomainName, DomainState};

#[derive(Debug, Clone)]
pub struct Virsh {
    domain: DomainName,
}

impl Virsh {
    pub fn new(domain: impl Into<DomainName>) -> Self {
        Self { domain: domain.into() }
    }

    fn call(&self, args: &[&str]) -> Result<String> {
        Command::new("virsh").args(args).wait_with_output()
    }

    pub fn exists(&self) -> Result<()> {
        self.call(&["dominfo", self.domain.as_str()])?;
        Ok(())
    }

    pub fn state(&self) -> Result<DomainState> {
        let stdout = self.call(&["domstate", self.domain.as_str()])?;
        Ok(DomainState::parse(stdout.trim()))
    }
}
