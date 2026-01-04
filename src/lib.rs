//! # embeddenator-interop
//!
//! Kernel interop and system integration for Embeddenator.
//!
//! Extracted from embeddenator core as part of Phase 2A component decomposition.

pub mod interop;
pub use interop::*;

#[cfg(test)]
mod tests {
    #[test]
    fn component_loads() {
        assert!(true);
    }
}
