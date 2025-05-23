use polkit_agent_sys as ffi;
mod auto;
pub use auto::*;
pub mod subclass;

pub use gio;
pub use polkit;
