#![cfg_attr(docsrs, feature(doc_cfg))]
pub use polkit_rs_sys as ffi;

#[allow(unused_imports)]
#[allow(non_snake_case)]
mod auto;

pub use crate::auto::*;
