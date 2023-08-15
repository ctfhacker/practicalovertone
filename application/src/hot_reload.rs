//! Utility functions to enable hot reload

use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::os::raw::{c_char, c_void};

#[link(name = "dl")]
extern "C" {
    pub(crate) fn dlopen(filename: *const c_char, flags: u32) -> Handle;
    pub(crate) fn dlclose(handle: Handle);
    pub(crate) fn dlsym(handle: Handle, symbol: *const c_char) -> *mut c_void;
    pub(crate) fn dlerror() -> *const c_char;
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Handle(pub usize);

impl Handle {
    pub fn drop(&mut self) {
        unsafe {
            dlclose(*self);
        }
    }
}

pub struct Symbol<T> {
    /// Handle to the opened symbo
    pub handle: *mut c_void,

    /// Type of function for this handle
    phantom: PhantomData<T>,
}

impl<T> std::ops::Deref for Symbol<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*(&self.handle as *const *mut _ as *const T) }
    }
}

pub fn load_library(name: &str) -> Handle {
    /// The path to copy the hot-reloadable library
    const TMP_FILE: &str = "/tmp/.libpracticalovertone.so";

    /// Lazy function call binding
    const RTLD_LAZY: u32 = 1;

    // Copy the current library into a temp file for hot reload.
    let _discard = std::fs::copy(name, TMP_FILE);

    unsafe {
        let name = CString::new(TMP_FILE).unwrap();
        let handle = dlopen(name.as_ptr(), RTLD_LAZY);
        assert!(handle.0 != 0, "libpracticalovertone.so not found");
        Handle(handle.0)
    }
}

impl Handle {
    pub fn get_symbol<T>(&self, symbol_name: &str) -> Result<Symbol<T>, CString> {
        unsafe {
            let symbol_name =
                CString::new(symbol_name).expect("{symbol_name} cannot be made into a CString");

            // Get the handle to this symbol
            let handle = dlsym(*self, symbol_name.as_ptr().cast::<i8>());

            // Return the `dlerror` if the symbol was not found
            if handle.is_null() {
                return Err(CStr::from_ptr(dlerror()).into());
            }

            // Return the handle to the symbol
            Ok(Symbol {
                handle,
                phantom: PhantomData,
            })
        }
    }
}
