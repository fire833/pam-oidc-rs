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

use bindings::pam_modules::{pam_get_user, pam_handle_t};

mod bindings;
mod config;
mod oidc;

#[no_mangle]
pub extern "C" fn pam_sm_authenticate(
    pamh: *mut pam_handle_t,
    flags: ::std::os::raw::c_int,
    argc: ::std::os::raw::c_int,
    argv: *mut *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    0
}

#[no_mangle]
pub extern "C" fn pam_sm_open_session(
    pamh: *mut pam_handle_t,
    flags: ::std::os::raw::c_int,
    argc: ::std::os::raw::c_int,
    argv: *mut *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
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
