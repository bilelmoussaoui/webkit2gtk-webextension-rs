// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMNode;
use DOMObject;
use Error;
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
use std::ptr;
use webkit2_webextension_sys;

glib_wrapper! {
    pub struct DOMAttr(Object<webkit2_webextension_sys::WebKitDOMAttr, webkit2_webextension_sys::WebKitDOMAttrClass, DOMAttrClass>) @extends DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_attr_get_type(),
    }
}

pub const NONE_DOM_ATTR: Option<&DOMAttr> = None;

pub trait DOMAttrExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_name(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_owner_element(&self) -> Option<DOMElement>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_specified(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_value(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_value(&self, value: &str) -> Result<(), Error>;

    fn connect_property_local_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_namespace_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_owner_element_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_prefix_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_specified_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMAttr>> DOMAttrExt for O {
    fn get_name(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_attr_get_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_owner_element(&self) -> Option<DOMElement> {
        unsafe {
            from_glib_none(webkit2_webextension_sys::webkit_dom_attr_get_owner_element(self.as_ref().to_glib_none().0))
        }
    }

    fn get_specified(&self) -> bool {
        unsafe {
            from_glib(webkit2_webextension_sys::webkit_dom_attr_get_specified(self.as_ref().to_glib_none().0))
        }
    }

    fn get_value(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_attr_get_value(self.as_ref().to_glib_none().0))
        }
    }

    fn set_value(&self, value: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = webkit2_webextension_sys::webkit_dom_attr_set_value(self.as_ref().to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_property_local_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::local-name\0".as_ptr() as *const _,
                Some(transmute(notify_local_name_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::name\0".as_ptr() as *const _,
                Some(transmute(notify_name_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_namespace_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::namespace-uri\0".as_ptr() as *const _,
                Some(transmute(notify_namespace_uri_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_owner_element_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::owner-element\0".as_ptr() as *const _,
                Some(transmute(notify_owner_element_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_prefix_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::prefix\0".as_ptr() as *const _,
                Some(transmute(notify_prefix_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_specified_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::specified\0".as_ptr() as *const _,
                Some(transmute(notify_specified_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::value\0".as_ptr() as *const _,
                Some(transmute(notify_value_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_local_name_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMAttr, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMAttr> {
    let f: &F = &*(f as *const F);
    f(&DOMAttr::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_name_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMAttr, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMAttr> {
    let f: &F = &*(f as *const F);
    f(&DOMAttr::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_namespace_uri_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMAttr, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMAttr> {
    let f: &F = &*(f as *const F);
    f(&DOMAttr::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_owner_element_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMAttr, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMAttr> {
    let f: &F = &*(f as *const F);
    f(&DOMAttr::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_prefix_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMAttr, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMAttr> {
    let f: &F = &*(f as *const F);
    f(&DOMAttr::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_specified_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMAttr, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMAttr> {
    let f: &F = &*(f as *const F);
    f(&DOMAttr::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_value_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMAttr, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMAttr> {
    let f: &F = &*(f as *const F);
    f(&DOMAttr::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for DOMAttr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMAttr")
    }
}
