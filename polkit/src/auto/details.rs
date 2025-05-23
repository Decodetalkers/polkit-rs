// Generated by gir (https://github.com/gtk-rs/gir @ b2a1c6f9b362)
// from ../misc (@ ???)
// DO NOT EDIT

use crate::ffi;
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "PolkitDetails")]
    pub struct Details(Object<ffi::PolkitDetails, ffi::PolkitDetailsClass>);

    match fn {
        type_ => || ffi::polkit_details_get_type(),
    }
}

impl Details {
    #[doc(alias = "polkit_details_new")]
    pub fn new() -> Details {
        unsafe { from_glib_full(ffi::polkit_details_new()) }
    }

    #[doc(alias = "polkit_details_get_keys")]
    #[doc(alias = "get_keys")]
    pub fn keys(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::polkit_details_get_keys(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "polkit_details_insert")]
    pub fn insert(&self, key: &str, value: Option<&str>) {
        unsafe {
            ffi::polkit_details_insert(
                self.to_glib_none().0,
                key.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "polkit_details_lookup")]
    pub fn lookup(&self, key: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::polkit_details_lookup(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }
}

impl Default for Details {
    fn default() -> Self {
        Self::new()
    }
}
