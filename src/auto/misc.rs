// This file was generated by gir (34ea1b9) from gir-files (11e0e6d)
// DO NOT EDIT

use Buildable;
use Widget;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use std::mem;

glib_wrapper! {
    pub struct Misc(Object<ffi::GtkMisc>): Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_misc_get_type(),
    }
}

pub trait MiscExt {
    fn get_alignment(&self) -> (f32, f32);

    fn get_padding(&self) -> (i32, i32);

    fn set_alignment(&self, xalign: f32, yalign: f32);

    fn set_padding(&self, xpad: i32, ypad: i32);
}

impl<O: IsA<Misc>> MiscExt for O {
    fn get_alignment(&self) -> (f32, f32) {
        unsafe {
            let mut xalign = mem::uninitialized();
            let mut yalign = mem::uninitialized();
            ffi::gtk_misc_get_alignment(self.to_glib_none().0, &mut xalign, &mut yalign);
            (xalign, yalign)
        }
    }

    fn get_padding(&self) -> (i32, i32) {
        unsafe {
            let mut xpad = mem::uninitialized();
            let mut ypad = mem::uninitialized();
            ffi::gtk_misc_get_padding(self.to_glib_none().0, &mut xpad, &mut ypad);
            (xpad, ypad)
        }
    }

    fn set_alignment(&self, xalign: f32, yalign: f32) {
        unsafe {
            ffi::gtk_misc_set_alignment(self.to_glib_none().0, xalign, yalign);
        }
    }

    fn set_padding(&self, xpad: i32, ypad: i32) {
        unsafe {
            ffi::gtk_misc_set_padding(self.to_glib_none().0, xpad, ypad);
        }
    }
}