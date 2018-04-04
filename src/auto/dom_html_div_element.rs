// This file was generated by gir (https://github.com/gtk-rs/gir @ 6855214)
// from gir-files (https://github.com/gtk-rs/gir-files @ 3fde76b)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
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
    pub struct DOMHTMLDivElement(Object<ffi::WebKitDOMHTMLDivElement, ffi::WebKitDOMHTMLDivElementClass>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_div_element_get_type(),
    }
}

pub trait DOMHTMLDivElementExt {
    fn get_align(&self) -> Option<String>;

    fn set_align(&self, value: &str);

    fn connect_property_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLDivElement> + IsA<glib::object::Object>> DOMHTMLDivElementExt for O {
    fn get_align(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_div_element_get_align(self.to_glib_none().0))
        }
    }

    fn set_align(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_div_element_set_align(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn connect_property_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::align",
                transmute(notify_align_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_align_trampoline<P>(this: *mut ffi::WebKitDOMHTMLDivElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLDivElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLDivElement::from_glib_borrow(this).downcast_unchecked())
}
