// This file was generated by gir (c6a4ae6) from gir-files (11e0e6d)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::Upcast;
use glib::translate::*;

glib_wrapper! {
    pub struct Overlay(Object<ffi::GtkOverlay>): Widget, Container, Bin, Buildable;

    match fn {
        get_type => || ffi::gtk_overlay_get_type(),
    }
}

impl Overlay {
    pub fn new() -> Overlay {
        unsafe {
            Widget::from_glib_none(ffi::gtk_overlay_new()).downcast_unchecked()
        }
    }

    pub fn add_overlay<T: Upcast<Widget>>(&self, widget: &T) {
        unsafe {
            ffi::gtk_overlay_add_overlay(self.to_glib_none().0, widget.to_glib_none().0);
        }
    }

}
