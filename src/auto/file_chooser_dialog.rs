// This file was generated by gir (6a48033) from gir-files (db49619)
// DO NOT EDIT

use Bin;
use Container;
use Dialog;
use FileChooser;
use Widget;
use Window;
use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct FileChooserDialog(Object<ffi::GtkFileChooserDialog>): Dialog, Window, Bin, Container, Widget, FileChooser;

    match fn {
        get_type => || ffi::gtk_file_chooser_dialog_get_type(),
    }
}

impl FileChooserDialog {
    //pub fn new<'a, 'b, 'c, P: Into<Option<&'a str>>, Q: IsA<Window> + 'b, R: Into<Option<&'b Q>>, S: Into<Option<&'c str>>>(title: P, parent: R, action: FileChooserAction, first_button_text: S, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> FileChooserDialog {
    //    unsafe { TODO: call ffi::gtk_file_chooser_dialog_new() }
    //}
}
