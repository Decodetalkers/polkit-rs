// Generated by gir (https://github.com/gtk-rs/gir @ b2a1c6f9b362)
// from ../misc (@ ???)
// DO NOT EDIT

use crate::ffi::polkit_unix_process_set_gids;
use crate::{Subject, ffi};
use glib::{
    ffi::{GArray, g_array_append_vals, g_array_new},
    prelude::*,
    signal::{SignalHandlerId, connect_raw},
    translate::*,
};
use std::{boxed::Box as Box_, slice};

glib::wrapper! {
    #[doc(alias = "PolkitUnixProcess")]
    pub struct UnixProcess(Object<ffi::PolkitUnixProcess, ffi::PolkitUnixProcessClass>) @implements Subject;

    match fn {
        type_ => || ffi::polkit_unix_process_get_type(),
    }
}

impl UnixProcess {
    #[doc(alias = "polkit_unix_process_get_gids")]
    #[doc(alias = "get_gids")]
    pub fn gids(&self) -> Vec<i32> {
        unsafe {
            let arrays_g = ffi::polkit_unix_process_get_gids(self.to_glib_none().0);
            let arrays = (*arrays_g).data as *mut i32;
            let len = (*arrays_g).len as usize;
            slice::from_raw_parts(arrays, len).to_vec()
        }
    }

    #[doc(alias = "polkit_unix_process_get_owner")]
    #[doc(alias = "get_owner")]
    pub fn owner(&self) -> Result<i32, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::polkit_unix_process_get_owner(self.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "polkit_unix_process_get_pid")]
    #[doc(alias = "get_pid")]
    pub fn pid(&self) -> i32 {
        unsafe { ffi::polkit_unix_process_get_pid(self.to_glib_none().0) }
    }

    #[doc(alias = "polkit_unix_process_get_pidfd")]
    #[doc(alias = "get_pidfd")]
    pub fn pidfd(&self) -> i32 {
        unsafe { ffi::polkit_unix_process_get_pidfd(self.to_glib_none().0) }
    }

    #[doc(alias = "polkit_unix_process_get_pidfd_is_safe")]
    #[doc(alias = "get_pidfd_is_safe")]
    #[doc(alias = "pidfd-is-safe")]
    pub fn is_pidfd_is_safe(&self) -> bool {
        unsafe {
            from_glib(ffi::polkit_unix_process_get_pidfd_is_safe(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "polkit_unix_process_get_start_time")]
    #[doc(alias = "get_start_time")]
    #[doc(alias = "start-time")]
    pub fn start_time(&self) -> u64 {
        unsafe { ffi::polkit_unix_process_get_start_time(self.to_glib_none().0) }
    }

    #[doc(alias = "polkit_unix_process_get_uid")]
    #[doc(alias = "get_uid")]
    pub fn uid(&self) -> i32 {
        unsafe { ffi::polkit_unix_process_get_uid(self.to_glib_none().0) }
    }

    #[doc(alias = "polkit_unix_process_set_gids")]
    #[doc(alias = "gids")]
    pub fn set_gids(&self, gids: Vec<usize>) {
        unsafe {
            let new_gids = g_array_new(0, 0, std::mem::size_of::<usize>() as u32);

            g_array_append_vals(new_gids, gids.as_ptr() as *const _, gids.len() as u32);
            ffi::polkit_unix_process_set_gids(self.to_glib_none().0, new_gids);
        }
    }

    #[doc(alias = "polkit_unix_process_set_pid")]
    #[doc(alias = "pid")]
    pub fn set_pid(&self, pid: i32) {
        unsafe {
            ffi::polkit_unix_process_set_pid(self.to_glib_none().0, pid);
        }
    }

    #[doc(alias = "polkit_unix_process_set_pidfd")]
    #[doc(alias = "pidfd")]
    pub fn set_pidfd(&self, pidfd: i32) {
        unsafe {
            ffi::polkit_unix_process_set_pidfd(self.to_glib_none().0, pidfd);
        }
    }

    #[doc(alias = "polkit_unix_process_set_start_time")]
    #[doc(alias = "start-time")]
    pub fn set_start_time(&self, start_time: u64) {
        unsafe {
            ffi::polkit_unix_process_set_start_time(self.to_glib_none().0, start_time);
        }
    }

    #[doc(alias = "polkit_unix_process_set_uid")]
    #[doc(alias = "uid")]
    pub fn set_uid(&self, uid: i32) {
        unsafe {
            ffi::polkit_unix_process_set_uid(self.to_glib_none().0, uid);
        }
    }

    #[doc(alias = "polkit_unix_process_new")]
    pub fn new(pid: i32) -> Self {
        unsafe {
            let subject: Subject = from_glib_full(ffi::polkit_unix_process_new(pid));
            subject.unsafe_cast()
        }
    }

    #[doc(alias = "polkit_unix_process_new_for_owner")]
    pub fn new_for_owner(pid: i32, start_time: u64, uid: i32) -> Self {
        unsafe {
            let subject: Subject =
                from_glib_full(ffi::polkit_unix_process_new_for_owner(pid, start_time, uid));
            subject.unsafe_cast()
        }
    }

    #[doc(alias = "polkit_unix_process_new_full")]
    pub fn new_full(pid: i32, start_time: u64) -> Self {
        unsafe {
            let subject: Subject =
                from_glib_full(ffi::polkit_unix_process_new_full(pid, start_time));
            subject.unsafe_cast()
        }
    }

    #[doc(alias = "polkit_unix_process_new_pidfd")]
    pub fn new_pidfd(pidfd: i32, uid: i32, gids: Vec<usize>) -> Self {
        unsafe {
            let subject: Subject = {
                let new_gids = g_array_new(0, 0, std::mem::size_of::<usize>() as u32);

                g_array_append_vals(new_gids, gids.as_ptr() as *const _, gids.len() as u32);
                from_glib_full(ffi::polkit_unix_process_new_pidfd(pidfd, uid, new_gids))
            };
            subject.unsafe_cast()
        }
    }

    #[doc(alias = "gids")]
    pub fn connect_gids_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_gids_trampoline<F: Fn(&UnixProcess) + 'static>(
            this: *mut ffi::PolkitUnixProcess,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            unsafe {
                let f: &F = &*(f as *const F);
                f(&from_glib_borrow(this))
            }
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::gids".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_gids_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pid")]
    pub fn connect_pid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pid_trampoline<F: Fn(&UnixProcess) + 'static>(
            this: *mut ffi::PolkitUnixProcess,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            unsafe {
                let f: &F = &*(f as *const F);
                f(&from_glib_borrow(this))
            }
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::pid".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_pid_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pidfd")]
    pub fn connect_pidfd_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pidfd_trampoline<F: Fn(&UnixProcess) + 'static>(
            this: *mut ffi::PolkitUnixProcess,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            unsafe {
                let f: &F = &*(f as *const F);
                f(&from_glib_borrow(this))
            }
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::pidfd".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_pidfd_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pidfd-is-safe")]
    pub fn connect_pidfd_is_safe_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pidfd_is_safe_trampoline<F: Fn(&UnixProcess) + 'static>(
            this: *mut ffi::PolkitUnixProcess,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            unsafe {
                let f: &F = &*(f as *const F);
                f(&from_glib_borrow(this))
            }
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::pidfd-is-safe".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_pidfd_is_safe_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "start-time")]
    pub fn connect_start_time_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_start_time_trampoline<F: Fn(&UnixProcess) + 'static>(
            this: *mut ffi::PolkitUnixProcess,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            unsafe {
                let f: &F = &*(f as *const F);
                f(&from_glib_borrow(this))
            }
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::start-time".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_start_time_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "uid")]
    pub fn connect_uid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_uid_trampoline<F: Fn(&UnixProcess) + 'static>(
            this: *mut ffi::PolkitUnixProcess,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            unsafe {
                let f: &F = &*(f as *const F);
                f(&from_glib_borrow(this))
            }
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::uid".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_uid_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
