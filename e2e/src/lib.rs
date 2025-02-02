#![cfg_attr(test, deny(missing_docs))]
#![doc(html_no_source)]

//! Core library traits and utilities for E2E testing framework

pub mod common;
pub mod logging;
pub mod distribution;
pub mod ide;
pub mod startup;

// Re-exports
pub use logging::{Logger, ConsoleLogger, TestLogger};
pub use distribution::DistributionTest;
pub use ide::IdeTest;
pub use startup::StartupTest;
