// Copyright (c) 2022 Sungbae Jeong
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::ffi::{CStr, CString};
use std::fs;
use std::io::prelude::*;

#[no_mangle]
pub extern "C" fn read_file(filename: *const i8) -> *const i8 {
    if filename.is_null() {
        eprintln!("filename pointer is null");
        return std::ptr::null();
    }

    let filename = match unsafe { CStr::from_ptr(filename) }.to_str() {
        Ok(filename) => filename,
        Err(err) => {
            eprintln!("{err}");
            return std::ptr::null();
        }
    };
    let mut file = match fs::File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("{err}");
            return std::ptr::null();
        }
    };
    let mut buf = Vec::with_capacity(100);
    if file.read_to_end(&mut buf).is_err() {
        return std::ptr::null();
    }
    buf.push(0);

    // SAFETY: Since we add a null byte to the end, converting into a c_string is file
    let cstr = unsafe { CString::from_vec_unchecked(buf) };
    cstr.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn free_file_contents(file_content: *const i8) {
    let _ = CString::from_raw(file_content.cast_mut());
}
