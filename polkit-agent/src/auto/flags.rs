// Generated by gir (https://github.com/gtk-rs/gir @ b2a1c6f9b362)
// from ../misc (@ ???)
// DO NOT EDIT

use crate::ffi;
use glib::{bitflags::bitflags, prelude::*, translate::*};

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "PolkitAgentRegisterFlags")]
    pub struct RegisterFlags: u32 {
        #[doc(alias = "POLKIT_AGENT_REGISTER_FLAGS_NONE")]
        const NONE = ffi::POLKIT_AGENT_REGISTER_FLAGS_NONE as _;
        #[doc(alias = "POLKIT_AGENT_REGISTER_FLAGS_RUN_IN_THREAD")]
        const RUN_IN_THREAD = ffi::POLKIT_AGENT_REGISTER_FLAGS_RUN_IN_THREAD as _;
    }
}

#[doc(hidden)]
impl IntoGlib for RegisterFlags {
    type GlibType = ffi::PolkitAgentRegisterFlags;

    #[inline]
    fn into_glib(self) -> ffi::PolkitAgentRegisterFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::PolkitAgentRegisterFlags> for RegisterFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::PolkitAgentRegisterFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl StaticType for RegisterFlags {
    #[inline]
    #[doc(alias = "polkit_agent_register_flags_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::polkit_agent_register_flags_get_type()) }
    }
}

impl glib::HasParamSpec for RegisterFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

impl glib::value::ValueType for RegisterFlags {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for RegisterFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        unsafe { from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0)) }
    }
}

impl ToValue for RegisterFlags {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<RegisterFlags> for glib::Value {
    #[inline]
    fn from(v: RegisterFlags) -> Self {
        ToValue::to_value(&v)
    }
}
