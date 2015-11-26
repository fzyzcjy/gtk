// This file was generated by gir (b85b267+) from gir-files (11e0e6d)
// DO NOT EDIT

use Unit;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct PaperSize(Boxed<ffi::GtkPaperSize>);

    match fn {
        copy => |ptr| ffi::gtk_paper_size_copy(ptr as *mut ffi::GtkPaperSize),
        free => |ptr| ffi::gtk_paper_size_free(ptr),
    }
}

impl PaperSize {
    pub fn new(name: Option<&str>) -> PaperSize {
        unsafe {
            from_glib_full(ffi::gtk_paper_size_new(name.to_glib_none().0))
        }
    }

    pub fn new_custom(name: &str, display_name: &str, width: f64, height: f64, unit: Unit) -> PaperSize {
        unsafe {
            from_glib_full(ffi::gtk_paper_size_new_custom(name.to_glib_none().0, display_name.to_glib_none().0, width, height, unit))
        }
    }

    #[cfg(gtk_3_16)]
    pub fn new_from_ipp(ipp_name: &str, width: f64, height: f64) -> PaperSize {
        unsafe {
            from_glib_full(ffi::gtk_paper_size_new_from_ipp(ipp_name.to_glib_none().0, width, height))
        }
    }

    //pub fn new_from_key_file(key_file: /*Ignored*/&glib::KeyFile, group_name: &str, error: /*Ignored*/Option<glib::Error>) -> PaperSize {
    //    unsafe { TODO: call ffi::gtk_paper_size_new_from_key_file() }
    //}

    pub fn new_from_ppd(ppd_name: &str, ppd_display_name: &str, width: f64, height: f64) -> PaperSize {
        unsafe {
            from_glib_full(ffi::gtk_paper_size_new_from_ppd(ppd_name.to_glib_none().0, ppd_display_name.to_glib_none().0, width, height))
        }
    }

    pub fn get_default_bottom_margin(&self, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_paper_size_get_default_bottom_margin(self.to_glib_none().0, unit)
        }
    }

    pub fn get_default_left_margin(&self, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_paper_size_get_default_left_margin(self.to_glib_none().0, unit)
        }
    }

    pub fn get_default_right_margin(&self, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_paper_size_get_default_right_margin(self.to_glib_none().0, unit)
        }
    }

    pub fn get_default_top_margin(&self, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_paper_size_get_default_top_margin(self.to_glib_none().0, unit)
        }
    }

    pub fn get_display_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_paper_size_get_display_name(self.to_glib_none().0))
        }
    }

    pub fn get_height(&self, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_paper_size_get_height(self.to_glib_none().0, unit)
        }
    }

    pub fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_paper_size_get_name(self.to_glib_none().0))
        }
    }

    pub fn get_ppd_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_paper_size_get_ppd_name(self.to_glib_none().0))
        }
    }

    pub fn get_width(&self, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_paper_size_get_width(self.to_glib_none().0, unit)
        }
    }

    pub fn is_custom(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_paper_size_is_custom(self.to_glib_none().0))
        }
    }

    pub fn is_equal(&self, size2: &PaperSize) -> bool {
        unsafe {
            from_glib(ffi::gtk_paper_size_is_equal(self.to_glib_none().0, size2.to_glib_none().0))
        }
    }

    pub fn is_ipp(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_paper_size_is_ipp(self.to_glib_none().0))
        }
    }

    pub fn set_size(&self, width: f64, height: f64, unit: Unit) {
        unsafe {
            ffi::gtk_paper_size_set_size(self.to_glib_none().0, width, height, unit);
        }
    }

    //pub fn to_key_file(&self, key_file: /*Ignored*/&glib::KeyFile, group_name: &str) {
    //    unsafe { TODO: call ffi::gtk_paper_size_to_key_file() }
    //}

    pub fn get_default() -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_paper_size_get_default())
        }
    }

    pub fn get_paper_sizes(include_custom: bool) -> Vec<PaperSize> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_paper_size_get_paper_sizes(include_custom.to_glib()))
        }
    }

}
