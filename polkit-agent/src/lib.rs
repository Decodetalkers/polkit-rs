use polkit_agent_rs_sys as ffi;
mod auto;
pub use auto::*;
pub mod subclass;

pub use gio;
pub use polkit_rs as polkit;
