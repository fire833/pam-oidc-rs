/*
*	Copyright (C) 2023 Kendall Tauser
*
*	This program is free software; you can redistribute it and/or modify
*	it under the terms of the GNU General Public License as published by
*	the Free Software Foundation; either version 2 of the License, or
*	(at your option) any later version.
*
*	This program is distributed in the hope that it will be useful,
*	but WITHOUT ANY WARRANTY; without even the implied warranty of
*	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
*	GNU General Public License for more details.
*
*	You should have received a copy of the GNU General Public License along
*	with this program; if not, write to the Free Software Foundation, Inc.,
*	51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 */

use std::{ffi::CStr, ptr::null};

use bindings::{
    pam_ext::pam_get_authtok,
    pam_modules::{
        pam_get_user, pam_handle_t, PAM_ABORT, PAM_NO_MODULE_DATA, PAM_SUCCESS, PAM_TRY_AGAIN,
    },
    pam_modutil::PAM_AUTHTOK,
};

mod bindings;
mod config;
mod oauth;

#[no_mangle]
pub extern "C" fn pam_sm_authenticate(
    pamh: *mut pam_handle_t,
    flags: std::os::raw::c_int,
    argc: std::os::raw::c_int,
    argv: *mut *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    let user: &str;
    let pass: &str;
    let ucstr: &CStr;
    let pcstr: &CStr;

    unsafe {
        let ubuf: *const i8;
        let pbuf: *const i8;
        let err = pam_get_user(pamh, ubuf, null());
        if err != PAM_SUCCESS as i32 {
            return PAM_TRY_AGAIN as i32;
        }

        let err = pam_get_authtok(pamh, PAM_AUTHTOK as i32, pbuf, null());
        if err != PAM_SUCCESS as i32 {
            return PAM_TRY_AGAIN as i32;
        }

        ucstr = CStr::from_ptr(ubuf);
        pcstr = CStr::from_ptr(pbuf);
    }

    match ucstr.to_str() {
        Ok(s) => user = s,
        Err(e) => return PAM_ABORT as i32,
    }

    match pcstr.to_str() {
        Ok(s) => pass = s,
        Err(e) => return PAM_ABORT as i32,
    }

    if let Ok(conf) = config::PamOidcConfig::new() {
        let resp = conf.authorize_user(user, pass);
        match resp {
            Ok(v) => v,
            Err(_) => PAM_ABORT as i32,
        }
    } else {
        PAM_NO_MODULE_DATA as i32
    }
}

#[no_mangle]
pub extern "C" fn pam_sm_open_session(
    pamh: *mut pam_handle_t,
    flags: std::os::raw::c_int,
    argc: std::os::raw::c_int,
    argv: *mut *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    unsafe {
        // let user: *mut *const ::std::os::raw::c_char;
        // let prompt: *const ::std::os::raw::c_char;
        // let i = pam_get_user(pamh, user, prompt);
    };

    0
}

#[no_mangle]
pub extern "C" fn pam_sm_close_session(
    pamh: *mut pam_handle_t,
    flags: ::std::os::raw::c_int,
    argc: ::std::os::raw::c_int,
    argv: *mut *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    0
}
