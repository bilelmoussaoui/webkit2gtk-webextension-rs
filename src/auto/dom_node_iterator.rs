// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMNode;
use DOMObject;
use Error;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;
use webkit2_webextension_sys;

glib_wrapper! {
    pub struct DOMNodeIterator(Object<webkit2_webextension_sys::WebKitDOMNodeIterator, webkit2_webextension_sys::WebKitDOMNodeIteratorClass, DOMNodeIteratorClass>) @extends DOMObject;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_node_iterator_get_type(),
    }
}

pub const NONE_DOM_NODE_ITERATOR: Option<&DOMNodeIterator> = None;

pub trait DOMNodeIteratorExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn detach(&self);

    #[cfg_attr(feature = "v2_12", deprecated)]
    fn get_expand_entity_references(&self) -> bool;

    //#[cfg_attr(feature = "v2_22", deprecated)]
    //fn get_filter(&self) -> /*Ignored*/Option<DOMNodeFilter>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_pointer_before_reference_node(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_reference_node(&self) -> Option<DOMNode>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_root(&self) -> Option<DOMNode>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_what_to_show(&self) -> libc::c_ulong;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn next_node(&self) -> Result<DOMNode, Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn previous_node(&self) -> Result<DOMNode, Error>;

    fn connect_property_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pointer_before_reference_node_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_reference_node_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_root_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_what_to_show_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMNodeIterator>> DOMNodeIteratorExt for O {
    fn detach(&self) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_node_iterator_detach(self.as_ref().to_glib_none().0);
        }
    }

    fn get_expand_entity_references(&self) -> bool {
        unsafe {
            from_glib(webkit2_webextension_sys::webkit_dom_node_iterator_get_expand_entity_references(self.as_ref().to_glib_none().0))
        }
    }

    //fn get_filter(&self) -> /*Ignored*/Option<DOMNodeFilter> {
    //    unsafe { TODO: call webkit2_webextension_sys:webkit_dom_node_iterator_get_filter() }
    //}

    fn get_pointer_before_reference_node(&self) -> bool {
        unsafe {
            from_glib(webkit2_webextension_sys::webkit_dom_node_iterator_get_pointer_before_reference_node(self.as_ref().to_glib_none().0))
        }
    }

    fn get_reference_node(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(webkit2_webextension_sys::webkit_dom_node_iterator_get_reference_node(self.as_ref().to_glib_none().0))
        }
    }

    fn get_root(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(webkit2_webextension_sys::webkit_dom_node_iterator_get_root(self.as_ref().to_glib_none().0))
        }
    }

    fn get_what_to_show(&self) -> libc::c_ulong {
        unsafe {
            webkit2_webextension_sys::webkit_dom_node_iterator_get_what_to_show(self.as_ref().to_glib_none().0)
        }
    }

    fn next_node(&self) -> Result<DOMNode, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = webkit2_webextension_sys::webkit_dom_node_iterator_next_node(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn previous_node(&self) -> Result<DOMNode, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = webkit2_webextension_sys::webkit_dom_node_iterator_previous_node(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_property_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::filter\0".as_ptr() as *const _,
                Some(transmute(notify_filter_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_pointer_before_reference_node_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::pointer-before-reference-node\0".as_ptr() as *const _,
                Some(transmute(notify_pointer_before_reference_node_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_reference_node_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::reference-node\0".as_ptr() as *const _,
                Some(transmute(notify_reference_node_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_root_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::root\0".as_ptr() as *const _,
                Some(transmute(notify_root_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_what_to_show_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::what-to-show\0".as_ptr() as *const _,
                Some(transmute(notify_what_to_show_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_filter_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMNodeIterator, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMNodeIterator> {
    let f: &F = &*(f as *const F);
    f(&DOMNodeIterator::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_pointer_before_reference_node_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMNodeIterator, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMNodeIterator> {
    let f: &F = &*(f as *const F);
    f(&DOMNodeIterator::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_reference_node_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMNodeIterator, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMNodeIterator> {
    let f: &F = &*(f as *const F);
    f(&DOMNodeIterator::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_root_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMNodeIterator, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMNodeIterator> {
    let f: &F = &*(f as *const F);
    f(&DOMNodeIterator::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_what_to_show_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMNodeIterator, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMNodeIterator> {
    let f: &F = &*(f as *const F);
    f(&DOMNodeIterator::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for DOMNodeIterator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMNodeIterator")
    }
}
