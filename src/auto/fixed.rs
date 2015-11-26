// This file was generated by gir (c6a4ae6) from gir-files (11e0e6d)
// DO NOT EDIT

use Buildable;
use Container;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::Upcast;
use glib::translate::*;

glib_wrapper! {
    pub struct Fixed(Object<ffi::GtkFixed>): Widget, Container, Buildable;

    match fn {
        get_type => || ffi::gtk_fixed_get_type(),
    }
}

impl Fixed {
    pub fn new() -> Fixed {
        unsafe {
            Widget::from_glib_none(ffi::gtk_fixed_new()).downcast_unchecked()
        }
    }

    pub fn move_<T: Upcast<Widget>>(&self, widget: &T, x: i32, y: i32) {
        unsafe {
            ffi::gtk_fixed_move(self.to_glib_none().0, widget.to_glib_none().0, x, y);
        }
    }

    pub fn put<T: Upcast<Widget>>(&self, widget: &T, x: i32, y: i32) {
        unsafe {
            ffi::gtk_fixed_put(self.to_glib_none().0, widget.to_glib_none().0, x, y);
        }
    }

}
