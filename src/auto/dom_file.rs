// This file was generated by gir (https://github.com/gtk-rs/gir @ 6855214)
// from gir-files (https://github.com/gtk-rs/gir-files @ 3fde76b)
// DO NOT EDIT

use DOMBlob;
use DOMObject;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DOMFile(Object<ffi::WebKitDOMFile, ffi::WebKitDOMFileClass>): DOMBlob, DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_file_get_type(),
    }
}

pub trait DOMFileExt {
    fn get_name(&self) -> Option<String>;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMFile> + IsA<glib::object::Object>> DOMFileExt for O {
    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_file_get_name(self.to_glib_none().0))
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::name",
                transmute(notify_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_name_trampoline<P>(this: *mut ffi::WebKitDOMFile, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMFile> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMFile::from_glib_borrow(this).downcast_unchecked())
}
