// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use glib::GString;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use webkit2_webextension_sys;

glib_wrapper! {
    pub struct DOMHTMLTitleElement(Object<webkit2_webextension_sys::WebKitDOMHTMLTitleElement, webkit2_webextension_sys::WebKitDOMHTMLTitleElementClass, DOMHTMLTitleElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_html_title_element_get_type(),
    }
}

pub const NONE_DOMHTML_TITLE_ELEMENT: Option<&DOMHTMLTitleElement> = None;

pub trait DOMHTMLTitleElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_text(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_text(&self, value: &str);

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLTitleElement>> DOMHTMLTitleElementExt for O {
    fn get_text(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_title_element_get_text(self.as_ref().to_glib_none().0))
        }
    }

    fn set_text(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_title_element_set_text(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::text\0".as_ptr() as *const _,
                Some(transmute(notify_text_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_text_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLTitleElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLTitleElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLTitleElement::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for DOMHTMLTitleElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMHTMLTitleElement")
    }
}
