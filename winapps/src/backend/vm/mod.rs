//! Virtual machine abstractions.
//!
//! This module provides shared abstractions for hypervisor-backed Windows
//! environments. Concrete implementations are introduced incrementally.

/// Marker trait for virtual machine implementations.
///
/// Methods will be introduced alongside the first concrete implementation
/// to avoid committing to an API before real usage exists.
pub trait VirtualMachine {}
