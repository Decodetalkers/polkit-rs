use std::ffi::c_char;

use crate::polkit;
use crate::Listener;
use crate::ffi;
use gio::ffi::g_task_new;
use glib::GString;
use glib::object::Cast;
use glib::translate::FromGlibPtrBorrow;
use glib::translate::FromGlibPtrNone;
use glib::translate::IntoGlib;
use glib::translate::from_glib_none;
use glib::value::ValueType;
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
    let task = unsafe { g_task_new(ptr as *mut _, cancellable, callback, user_data) };
    let task: gio::Task<T::Message> = unsafe { from_glib_none(task) };
    let instance = unsafe { &*(ptr as *mut T::Instance) };
    let imp = instance.imp();

    let action_id = unsafe { GString::from_glib_borrow(action_id) };
    let message = unsafe { GString::from_glib_borrow(message) };
    let icon_name = unsafe { GString::from_glib_borrow(icon_name) };
    let details: polkit::Details = unsafe { from_glib_none(details) };
    let cookie = unsafe { GString::from_glib_borrow(cookie) };
    let identities: Vec<polkit::Identity> =
        unsafe { glist_to_vec::<polkit::Identity, polkit::ffi::PolkitIdentity>(identities) };
    let cancellable: gio::Cancellable = unsafe { from_glib_none(cancellable) };
    imp.initiate_authentication(
        action_id.as_str(),
        &message,
        &icon_name,
        &details,
        &cookie,
        identities,
        cancellable,
        task,
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
        let finish_res_pre = error.map(|err| Err(err));
        let finish_res = finish_res_pre.unwrap_or(Ok(gio_result
            .downcast::<gio::Task<T::Message>>()
            .expect("Should can be downcasted")));

        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        imp.initiate_authentication_finish(finish_res).into_glib()
    }
}

impl<T: ListenerImpl> ListenerImplExt for T {}

pub trait ListenerImpl: ObjectImpl + ObjectSubclass<Type: IsA<Listener>> {
    type Message: ValueType + Send;
    fn initiate_authentication(
        &self,
        action_id: &str,
        message: &str,
        icon_name: &str,
        details: &polkit::Details,
        cookie: &str,
        identities: Vec<polkit::Identity>,
        cancellable: gio::Cancellable,
        task: gio::Task<Self::Message>,
    );

    fn initiate_authentication_finish(
        &self,
        gio_result: Result<gio::Task<Self::Message>, glib::Error>,
    ) -> bool;
}

pub trait ListenerImplExt: ListenerImpl {}
