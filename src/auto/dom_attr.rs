// This file was generated by gir (d50d839) from gir-files (469db10)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMNode;
use DOMObject;
use Error;
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
    pub struct DOMAttr(Object<ffi::WebKitDOMAttr, ffi::WebKitDOMAttrClass>): DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_attr_get_type(),
    }
}

pub trait DOMAttrExt {
    fn get_name(&self) -> Option<String>;

    fn get_owner_element(&self) -> Option<DOMElement>;

    fn get_specified(&self) -> bool;

    fn get_value(&self) -> Option<String>;

    fn set_value(&self, value: &str) -> Result<(), Error>;

    fn connect_property_local_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_namespace_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_owner_element_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_prefix_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_specified_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMAttr> + IsA<glib::object::Object>> DOMAttrExt for O {
    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_attr_get_name(self.to_glib_none().0))
        }
    }

    fn get_owner_element(&self) -> Option<DOMElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_attr_get_owner_element(self.to_glib_none().0))
        }
    }

    fn get_specified(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_attr_get_specified(self.to_glib_none().0))
        }
    }

    fn get_value(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_attr_get_value(self.to_glib_none().0))
        }
    }

    fn set_value(&self, value: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_attr_set_value(self.to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_property_local_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::local-name",
                transmute(notify_local_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::name",
                transmute(notify_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_namespace_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::namespace-uri",
                transmute(notify_namespace_uri_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_owner_element_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::owner-element",
                transmute(notify_owner_element_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_prefix_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::prefix",
                transmute(notify_prefix_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_specified_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::specified",
                transmute(notify_specified_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::value",
                transmute(notify_value_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_local_name_trampoline<P>(this: *mut ffi::WebKitDOMAttr, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMAttr> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMAttr::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_name_trampoline<P>(this: *mut ffi::WebKitDOMAttr, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMAttr> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMAttr::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_namespace_uri_trampoline<P>(this: *mut ffi::WebKitDOMAttr, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMAttr> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMAttr::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_owner_element_trampoline<P>(this: *mut ffi::WebKitDOMAttr, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMAttr> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMAttr::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_prefix_trampoline<P>(this: *mut ffi::WebKitDOMAttr, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMAttr> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMAttr::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_specified_trampoline<P>(this: *mut ffi::WebKitDOMAttr, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMAttr> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMAttr::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_value_trampoline<P>(this: *mut ffi::WebKitDOMAttr, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMAttr> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMAttr::from_glib_borrow(this).downcast_unchecked())
}
