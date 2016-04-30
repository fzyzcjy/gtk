// This file was generated by gir (324239f) from gir-files (11e0e6d)
// DO NOT EDIT

use Actionable;
use Bin;
use Button;
use Container;
use Widget;
use ffi;
use ffi::GtkLinkButton;
use glib::object::Downcast;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi::gboolean;
use glib_ffi::gpointer;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct LinkButton(Object<ffi::GtkLinkButton>): Button, Bin, Container, Widget, Actionable;

    match fn {
        get_type => || ffi::gtk_link_button_get_type(),
    }
}

impl LinkButton {
    pub fn new(uri: &str) -> LinkButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_link_button_new(uri.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_label(uri: &str, label: Option<&str>) -> LinkButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_link_button_new_with_label(uri.to_glib_none().0, label.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn get_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_link_button_get_uri(self.to_glib_none().0))
        }
    }

    pub fn get_visited(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_link_button_get_visited(self.to_glib_none().0))
        }
    }

    pub fn set_uri(&self, uri: &str) {
        unsafe {
            ffi::gtk_link_button_set_uri(self.to_glib_none().0, uri.to_glib_none().0);
        }
    }

    pub fn set_visited(&self, visited: bool) {
        unsafe {
            ffi::gtk_link_button_set_visited(self.to_glib_none().0, visited.to_glib());
        }
    }

    pub fn connect_activate_link<F: Fn(&LinkButton) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&LinkButton) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate-link",
                transmute(activate_link_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn activate_link_trampoline(this: *mut GtkLinkButton, f: gpointer) -> gboolean {
    callback_guard!();
    let f: &Box_<Fn(&LinkButton) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this)).to_glib()
}
