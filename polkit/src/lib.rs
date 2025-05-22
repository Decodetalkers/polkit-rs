use polkit_sys as ffi;

pub mod reexport {
    pub use polkit_sys::*;
}

#[allow(unused_imports)]
#[allow(non_snake_case)]
mod auto;

pub use crate::auto::*;
