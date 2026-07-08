use crate::command::Command;
use crate::Result;
use super::types::{DomainName, DomainState};

#[derive(Debug, Clone)]
pub struct Virsh {
    domain: DomainName,
}

impl Virsh {
    pub fn new(domain: impl Into<DomainName>) -> Self {
        Self {
            domain: domain.into(),
        }
    }

    /// Executes a virsh command. Encapsulates CLI syntax rules.
    fn call(&self, args: &[&str]) -> Result<String> {
        Command::new("virsh")
            .args(args)
            .wait_with_output()
    }

    /// Returns successfully if the configured domain exists.
    pub fn exists(&self) -> Result<()> {
        self.call(&["dominfo", self.domain.as_str()])?;
        Ok(())
    }

    /// Returns the current runtime state of the domain.
    pub fn state(&self) -> Result<DomainState> {
        let stdout = self.call(&["domstate", self.domain.as_str()])?;
        Ok(DomainState::parse(&stdout))
    }

    pub fn is_running(&self) -> Result<bool> {
        Ok(self.state()? == DomainState::Running)
    }

    pub fn is_stopped(&self) -> Result<bool> {
        Ok(self.state()? == DomainState::Stopped)
    }
}
