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

use libc::{c_char, c_int};
use pam::{
    constants::{PamFlag, PamResultCode},
    module::PamHandle,
};

use crate::config::PamOidcConfig;

mod config;
mod error;
mod oauth;

#[no_mangle]
pub extern "C" fn pam_sm_authenticate(
    module: PamHandle,
    _: PamFlag,
    _: c_int,
    _: *const *const c_char,
) -> PamResultCode {
    let user: String;
    let pass: String;

    match module.get_user(Some("Provide your username: ")) {
        Ok(u) => user = u,
        Err(e) => return e,
    }

    pass = "".to_owned();

    match PamOidcConfig::new() {
        Ok(c) => match c.authorize_user(&user, &pass) {
            Ok(c) => return c,
            Err(e) => {
                println!("unable to authorize user: {}", e);
                return PamResultCode::PAM_CRED_INSUFFICIENT;
            }
        },
        Err(e) => {
            println!("unable to read in config: {}", e);
            return PamResultCode::PAM_CRED_UNAVAIL;
        }
    }
}
