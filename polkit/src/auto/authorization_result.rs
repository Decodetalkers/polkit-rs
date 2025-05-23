// Generated by gir (https://github.com/gtk-rs/gir @ b2a1c6f9b362)
// from ../misc (@ ???)
// DO NOT EDIT

use crate::{Details, ffi};
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "PolkitAuthorizationResult")]
    pub struct AuthorizationResult(Object<ffi::PolkitAuthorizationResult, ffi::PolkitAuthorizationResultClass>);

    match fn {
        type_ => || ffi::polkit_authorization_result_get_type(),
    }
}

impl AuthorizationResult {
    #[doc(alias = "polkit_authorization_result_new")]
    pub fn new(
        is_authorized: bool,
        is_challenge: bool,
        details: Option<&Details>,
    ) -> AuthorizationResult {
        unsafe {
            from_glib_full(ffi::polkit_authorization_result_new(
                is_authorized.into_glib(),
                is_challenge.into_glib(),
                details.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "polkit_authorization_result_get_details")]
    #[doc(alias = "get_details")]
    pub fn details(&self) -> Option<Details> {
        unsafe {
            from_glib_none(ffi::polkit_authorization_result_get_details(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v0_101")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_101")))]
    #[doc(alias = "polkit_authorization_result_get_dismissed")]
    #[doc(alias = "get_dismissed")]
    pub fn is_dismissed(&self) -> bool {
        unsafe {
            from_glib(ffi::polkit_authorization_result_get_dismissed(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "polkit_authorization_result_get_is_authorized")]
    #[doc(alias = "get_is_authorized")]
    pub fn is_authorized(&self) -> bool {
        unsafe {
            from_glib(ffi::polkit_authorization_result_get_is_authorized(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "polkit_authorization_result_get_is_challenge")]
    #[doc(alias = "get_is_challenge")]
    pub fn is_challenge(&self) -> bool {
        unsafe {
            from_glib(ffi::polkit_authorization_result_get_is_challenge(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "polkit_authorization_result_get_retains_authorization")]
    #[doc(alias = "get_retains_authorization")]
    pub fn is_retains_authorization(&self) -> bool {
        unsafe {
            from_glib(ffi::polkit_authorization_result_get_retains_authorization(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "polkit_authorization_result_get_temporary_authorization_id")]
    #[doc(alias = "get_temporary_authorization_id")]
    pub fn temporary_authorization_id(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(
                ffi::polkit_authorization_result_get_temporary_authorization_id(
                    self.to_glib_none().0,
                ),
            )
        }
    }
}
