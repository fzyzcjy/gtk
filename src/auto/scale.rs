// This file was generated by gir (c6a4ae6) from gir-files (11e0e6d)
// DO NOT EDIT

use Adjustment;
use Buildable;
use Orientable;
use Orientation;
use PositionType;
use Range;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;
use std::mem;

glib_wrapper! {
    pub struct Scale(Object<ffi::GtkScale>): Widget, Range, Buildable, Orientable;

    match fn {
        get_type => || ffi::gtk_scale_get_type(),
    }
}

impl Scale {
    pub fn new(orientation: Orientation, adjustment: Option<&Adjustment>) -> Scale {
        unsafe {
            Widget::from_glib_none(ffi::gtk_scale_new(orientation, adjustment.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_range(orientation: Orientation, min: f64, max: f64, step: f64) -> Scale {
        unsafe {
            Widget::from_glib_none(ffi::gtk_scale_new_with_range(orientation, min, max, step)).downcast_unchecked()
        }
    }

    pub fn add_mark(&self, value: f64, position: PositionType, markup: Option<&str>) {
        unsafe {
            ffi::gtk_scale_add_mark(self.to_glib_none().0, value, position, markup.to_glib_none().0);
        }
    }

    pub fn clear_marks(&self) {
        unsafe {
            ffi::gtk_scale_clear_marks(self.to_glib_none().0);
        }
    }

    pub fn get_digits(&self) -> i32 {
        unsafe {
            ffi::gtk_scale_get_digits(self.to_glib_none().0)
        }
    }

    pub fn get_draw_value(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_scale_get_draw_value(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_4)]
    pub fn get_has_origin(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_scale_get_has_origin(self.to_glib_none().0))
        }
    }

    //pub fn get_layout(&self) -> /*Ignored*/Option<pango::Layout> {
    //    unsafe { TODO: call ffi::gtk_scale_get_layout() }
    //}

    pub fn get_layout_offsets(&self) -> (i32, i32) {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            ffi::gtk_scale_get_layout_offsets(self.to_glib_none().0, &mut x, &mut y);
            (x, y)
        }
    }

    pub fn get_value_pos(&self) -> PositionType {
        unsafe {
            ffi::gtk_scale_get_value_pos(self.to_glib_none().0)
        }
    }

    pub fn set_digits(&self, digits: i32) {
        unsafe {
            ffi::gtk_scale_set_digits(self.to_glib_none().0, digits);
        }
    }

    pub fn set_draw_value(&self, draw_value: bool) {
        unsafe {
            ffi::gtk_scale_set_draw_value(self.to_glib_none().0, draw_value.to_glib());
        }
    }

    #[cfg(gtk_3_4)]
    pub fn set_has_origin(&self, has_origin: bool) {
        unsafe {
            ffi::gtk_scale_set_has_origin(self.to_glib_none().0, has_origin.to_glib());
        }
    }

    pub fn set_value_pos(&self, pos: PositionType) {
        unsafe {
            ffi::gtk_scale_set_value_pos(self.to_glib_none().0, pos);
        }
    }

}
