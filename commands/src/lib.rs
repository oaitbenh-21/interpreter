// Declare the submodule
pub mod builtin;

// Optional: re-export things you want external crates to access
// For example, if builtin::mod.rs has `Registry` and `Cmd`
pub use builtin::{Registry, Cmd};
