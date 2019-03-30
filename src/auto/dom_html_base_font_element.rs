// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use libc;
use std::fmt;
use webkit2_webextension_sys;

glib_wrapper! {
    pub struct DOMHTMLBaseFontElement(Object<webkit2_webextension_sys::WebKitDOMHTMLBaseFontElement, webkit2_webextension_sys::WebKitDOMHTMLBaseFontElementClass, DOMHTMLBaseFontElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_html_base_font_element_get_type(),
    }
}

pub const NONE_DOMHTML_BASE_FONT_ELEMENT: Option<&DOMHTMLBaseFontElement> = None;

pub trait DOMHTMLBaseFontElementExt: 'static {
    #[cfg_attr(feature = "v2_12", deprecated)]
    fn get_color(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_12", deprecated)]
    fn get_face(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_12", deprecated)]
    fn get_size(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_12", deprecated)]
    fn set_color(&self, value: &str);

    #[cfg_attr(feature = "v2_12", deprecated)]
    fn set_face(&self, value: &str);

    #[cfg_attr(feature = "v2_12", deprecated)]
    fn set_size(&self, value: libc::c_long);
}

impl<O: IsA<DOMHTMLBaseFontElement>> DOMHTMLBaseFontElementExt for O {
    fn get_color(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_base_font_element_get_color(self.as_ref().to_glib_none().0))
        }
    }

    fn get_face(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_base_font_element_get_face(self.as_ref().to_glib_none().0))
        }
    }

    fn get_size(&self) -> libc::c_long {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_base_font_element_get_size(self.as_ref().to_glib_none().0)
        }
    }

    fn set_color(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_base_font_element_set_color(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_face(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_base_font_element_set_face(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_size(&self, value: libc::c_long) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_base_font_element_set_size(self.as_ref().to_glib_none().0, value);
        }
    }
}

impl fmt::Display for DOMHTMLBaseFontElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMHTMLBaseFontElement")
    }
}
