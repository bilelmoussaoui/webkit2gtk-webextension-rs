// This file was generated by gir (d50d839) from gir-files (469db10)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMHTMLFormElement;
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
    pub struct DOMHTMLFieldSetElement(Object<ffi::WebKitDOMHTMLFieldSetElement, ffi::WebKitDOMHTMLFieldSetElementClass>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_field_set_element_get_type(),
    }
}

pub trait DOMHTMLFieldSetElementExt {
    fn get_form(&self) -> Option<DOMHTMLFormElement>;

    fn connect_property_form_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLFieldSetElement> + IsA<glib::object::Object>> DOMHTMLFieldSetElementExt for O {
    fn get_form(&self) -> Option<DOMHTMLFormElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_field_set_element_get_form(self.to_glib_none().0))
        }
    }

    fn connect_property_form_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::form",
                transmute(notify_form_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_form_trampoline<P>(this: *mut ffi::WebKitDOMHTMLFieldSetElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLFieldSetElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLFieldSetElement::from_glib_borrow(this).downcast_unchecked())
}
