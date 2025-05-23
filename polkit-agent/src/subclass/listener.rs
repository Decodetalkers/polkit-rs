use std::ffi::c_char;

use crate::Listener;
use crate::ffi;
use glib::GString;
use glib::translate::FromGlibPtrBorrow;
use glib::translate::FromGlibPtrNone;
use glib::translate::IntoGlib;
use glib::translate::from_glib_none;
use glib::{object::IsA, subclass::prelude::*};

unsafe impl<T: ListenerImpl> IsSubclassable<T> for Listener {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.initiate_authentication = Some(initiate_authentication::<T>);
        klass.initiate_authentication_finish = Some(initiate_authentication_finish::<T>);
    }
}

unsafe fn glist_to_vec<T, F>(glist: *mut glib::ffi::GList) -> Vec<T>
where
    T: IsA<glib::Object> + FromGlibPtrNone<*mut F>,
    F: 'static,
{
    let mut list = vec![];

    let mut l = glist;
    while !l.is_null() {
        let data = (unsafe { *l }).data as *mut F;
        if !data.is_null() {
            let obj: T = unsafe { from_glib_none(data) };
            list.push(obj);
        }
        l = unsafe { (*l).next };
    }
    list
}

unsafe extern "C" fn initiate_authentication<T: ListenerImpl>(
    ptr: *mut ffi::PolkitAgentListener,
    action_id: *const c_char,
    message: *const c_char,
    icon_name: *const c_char,
    details: *mut polkit::ffi::PolkitDetails,
    cookie: *const c_char,
    identities: *mut glib::ffi::GList,
    cancellable: *mut gio::ffi::GCancellable,
    callback: gio::ffi::GAsyncReadyCallback,
    user_data: glib::ffi::gpointer,
) {
    let instance = unsafe { &*(ptr as *mut T::Instance) };
    let imp = instance.imp();

    let action_id = unsafe { GString::from_glib_borrow(action_id) };
    let message = unsafe { GString::from_glib_borrow(message) };
    let icon_name = unsafe { GString::from_glib_borrow(icon_name) };
    let details: polkit::Details = unsafe { from_glib_none(details) };
    let cookie = unsafe { GString::from_glib_borrow(cookie) };
    let identities: Vec<polkit::Identity> =
        unsafe { glist_to_vec::<polkit::Identity, polkit::ffi::PolkitIdentity>(identities) };
    let cancelable: gio::Cancellable = unsafe { from_glib_none(cancellable) };
    imp.initilate_authentication(
        action_id.as_str(),
        &message,
        &icon_name,
        &details,
        &cookie,
        &identities,
        cancelable,
        callback,
        user_data,
    );
}
unsafe extern "C" fn initiate_authentication_finish<T: ListenerImpl>(
    ptr: *mut ffi::PolkitAgentListener,
    gio_result: *mut gio::ffi::GAsyncResult,
    error: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    unsafe {
        let gio_result: gio::AsyncResult = from_glib_none(gio_result);
        let error: Option<glib::Error> = from_glib_none(*error);
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        imp.initiate_authentication_finish(gio_result, error)
            .into_glib()
    }
}

impl<T: ListenerImpl> ListenerImplExt for T {}

pub trait ListenerImpl: ObjectImpl + ObjectSubclass<Type: IsA<Listener>> {
    fn initilate_authentication(
        &self,
        _action_id: &str,
        _message: &str,
        _icon_name: &str,
        _details: &polkit::Details,
        _cookie: &str,
        _identities: &[polkit::Identity],
        _cancelable: gio::Cancellable,
        _callback: gio::ffi::GAsyncReadyCallback,
        _user_data: glib::ffi::gpointer,
    ) {
    }
    fn initiate_authentication_finish(
        &self,
        _gio_result: gio::AsyncResult,
        _error: Option<glib::Error>,
    ) -> bool {
        false
    }
}

pub trait ListenerImplExt: ListenerImpl {}
