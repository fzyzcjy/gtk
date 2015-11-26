// This file was generated by gir (c6a4ae6) from gir-files (11e0e6d)
// DO NOT EDIT

use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct AccelGroup(Object<ffi::GtkAccelGroup>);

    match fn {
        get_type => || ffi::gtk_accel_group_get_type(),
    }
}

impl AccelGroup {
    pub fn new() -> AccelGroup {
        unsafe {
            from_glib_full(ffi::gtk_accel_group_new())
        }
    }

    //pub fn activate<T: Upcast</*Ignored*/gobject::Object>>(&self, accel_quark: /*Unknown conversion*/Unknown rust type: "Quark", acceleratable: &T, accel_key: u32, accel_mods: gdk::ModifierType) -> bool {
    //    unsafe { TODO: call ffi::gtk_accel_group_activate() }
    //}

    //pub fn connect(&self, accel_key: u32, accel_mods: gdk::ModifierType, accel_flags: AccelFlags, closure: /*Ignored*/&gobject::Closure) {
    //    unsafe { TODO: call ffi::gtk_accel_group_connect() }
    //}

    //pub fn connect_by_path(&self, accel_path: &str, closure: /*Ignored*/&gobject::Closure) {
    //    unsafe { TODO: call ffi::gtk_accel_group_connect_by_path() }
    //}

    //pub fn disconnect(&self, closure: /*Ignored*/Option<&gobject::Closure>) -> bool {
    //    unsafe { TODO: call ffi::gtk_accel_group_disconnect() }
    //}

    //pub fn disconnect_key(&self, accel_key: u32, accel_mods: gdk::ModifierType) -> bool {
    //    unsafe { TODO: call ffi::gtk_accel_group_disconnect_key() }
    //}

    //pub fn find(&self, find_func: /*Unknown conversion*/Unknown rust type: "AccelGroupFindFunc", data: Fundamental: Pointer) -> /*Ignored*/AccelKey {
    //    unsafe { TODO: call ffi::gtk_accel_group_find() }
    //}

    pub fn get_is_locked(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_accel_group_get_is_locked(self.to_glib_none().0))
        }
    }

    //pub fn get_modifier_mask(&self) -> gdk::ModifierType {
    //    unsafe { TODO: call ffi::gtk_accel_group_get_modifier_mask() }
    //}

    pub fn lock(&self) {
        unsafe {
            ffi::gtk_accel_group_lock(self.to_glib_none().0);
        }
    }

    //pub fn query(&self, accel_key: u32, accel_mods: gdk::ModifierType, n_entries: &mut u32) -> /*Unknown conversion*/Unknown rust type: "CArray TypeId { ns_id: 1, id: 14 }" {
    //    unsafe { TODO: call ffi::gtk_accel_group_query() }
    //}

    pub fn unlock(&self) {
        unsafe {
            ffi::gtk_accel_group_unlock(self.to_glib_none().0);
        }
    }

    //pub fn from_accel_closure(closure: /*Ignored*/&gobject::Closure) -> Option<AccelGroup> {
    //    unsafe { TODO: call ffi::gtk_accel_group_from_accel_closure() }
    //}

}
