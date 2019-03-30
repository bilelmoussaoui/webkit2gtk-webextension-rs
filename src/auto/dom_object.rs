// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use std::fmt;
use webkit2_webextension_sys;

glib_wrapper! {
    pub struct DOMObject(Object<webkit2_webextension_sys::WebKitDOMObject, webkit2_webextension_sys::WebKitDOMObjectClass, DOMObjectClass>);

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_object_get_type(),
    }
}

impl DOMObject {}

pub const NONE_DOM_OBJECT: Option<&DOMObject> = None;

impl fmt::Display for DOMObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMObject")
    }
}
