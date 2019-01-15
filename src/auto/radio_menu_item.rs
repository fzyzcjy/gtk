// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use CheckMenuItem;
use Container;
use MenuItem;
use Widget;
use ffi;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct RadioMenuItem(Object<ffi::GtkRadioMenuItem, ffi::GtkRadioMenuItemClass, RadioMenuItemClass>) @extends CheckMenuItem, MenuItem, Bin, Container, Widget, @implements Buildable, Actionable;

    match fn {
        get_type => || ffi::gtk_radio_menu_item_get_type(),
    }
}

impl RadioMenuItem {
    pub fn new_from_widget<P: IsA<RadioMenuItem>>(group: &P) -> RadioMenuItem {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_menu_item_new_from_widget(group.as_ref().to_glib_none().0)).unsafe_cast()
        }
    }

    pub fn new_with_label_from_widget<'a, P: IsA<RadioMenuItem>, Q: Into<Option<&'a str>>>(group: &P, label: Q) -> RadioMenuItem {
        skip_assert_initialized!();
        let label = label.into();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_menu_item_new_with_label_from_widget(group.as_ref().to_glib_none().0, label.to_glib_none().0)).unsafe_cast()
        }
    }

    pub fn new_with_mnemonic_from_widget<'a, P: IsA<RadioMenuItem>, Q: Into<Option<&'a str>>>(group: &P, label: Q) -> RadioMenuItem {
        skip_assert_initialized!();
        let label = label.into();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_menu_item_new_with_mnemonic_from_widget(group.as_ref().to_glib_none().0, label.to_glib_none().0)).unsafe_cast()
        }
    }
}

pub const NONE_RADIO_MENU_ITEM: Option<&RadioMenuItem> = None;

pub trait RadioMenuItemExt: 'static {
    fn get_group(&self) -> Vec<RadioMenuItem>;

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn join_group<'a, P: IsA<RadioMenuItem> + 'a, Q: Into<Option<&'a P>>>(&self, group_source: Q);

    fn connect_group_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<RadioMenuItem>> RadioMenuItemExt for O {
    fn get_group(&self) -> Vec<RadioMenuItem> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_radio_menu_item_get_group(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn join_group<'a, P: IsA<RadioMenuItem> + 'a, Q: Into<Option<&'a P>>>(&self, group_source: Q) {
        let group_source = group_source.into();
        unsafe {
            ffi::gtk_radio_menu_item_join_group(self.as_ref().to_glib_none().0, group_source.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn connect_group_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"group-changed\0".as_ptr() as *const _,
                transmute(group_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn group_changed_trampoline<P>(this: *mut ffi::GtkRadioMenuItem, f: glib_ffi::gpointer)
where P: IsA<RadioMenuItem> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&RadioMenuItem::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for RadioMenuItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RadioMenuItem")
    }
}
