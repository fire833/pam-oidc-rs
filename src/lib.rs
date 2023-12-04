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

use pam::{
    constants::{PamFlag, PamResultCode},
    module::{PamHandle, PamHooks},
    pam_hooks,
};

use crate::config::PamOidcConfig;

mod config;
mod error;
mod oauth;

pub struct PamOidc {}
pam_hooks!(PamOidc);

impl PamHooks for PamOidc {
    fn sm_authenticate(
        module: &mut PamHandle,
        _: Vec<&std::ffi::CStr>,
        _: PamFlag,
    ) -> PamResultCode {
        let user: String;
        let pass: String;

        match module.get_user(None) {
            Ok(u) => user = u,
            Err(e) => {
                println!("unable to retrieve user");
                return e.into();
            }
        }

        pass = "".into();

        match PamOidcConfig::new() {
            Ok(c) => match c.authorize_user(&user, &pass) {
                Ok(c) => return c,
                Err(e) => {
                    println!("unable to authorize user: {}", e);
                    return e.into();
                }
            },
            Err(e) => {
                println!("unable to read in config: {}", e);
                return e.into();
            }
        }
    }

    fn sm_setcred(_: &mut PamHandle, _: Vec<&std::ffi::CStr>, _: PamFlag) -> PamResultCode {
        PamResultCode::PAM_IGNORE
    }

    fn sm_chauthtok(_: &mut PamHandle, _: Vec<&std::ffi::CStr>, _: PamFlag) -> PamResultCode {
        PamResultCode::PAM_IGNORE
    }

    fn sm_open_session(_: &mut PamHandle, _: Vec<&std::ffi::CStr>, _: PamFlag) -> PamResultCode {
        PamResultCode::PAM_IGNORE
    }

    fn sm_close_session(_: &mut PamHandle, _: Vec<&std::ffi::CStr>, _: PamFlag) -> PamResultCode {
        PamResultCode::PAM_IGNORE
    }
}
