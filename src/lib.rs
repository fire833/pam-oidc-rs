/*
*	Copyright (C) 2025 Kendall Tauser
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
    items::ItemType,
    module::{PamHandle, PamHooks},
    pam_hooks,
};

use crate::config::PamOidcConfig;

mod config;
mod error;

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
                println!("unable to retrieve user: {:?}", e);
                return PamResultCode::PAM_ABORT;
            }
        }

        match module.get_authtok(ItemType::AuthTok, None) {
            Ok(p) => pass = p,
            Err(e) => {
                println!("unable to retrieve password: {:?}", e);
                return PamResultCode::PAM_ABORT;
            }
        }

        match PamOidcConfig::new() {
            Ok(c) => match c.authenticate_user(&user, &pass) {
                Ok(code) => {
                    if code != PamResultCode::PAM_SUCCESS {
                        println!("unsuccessful authentication: {:?}", code);
                    }
                    return code;
                }
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

    fn sm_open_session(
        pamh: &mut PamHandle,
        args: Vec<&std::ffi::CStr>,
        flags: PamFlag,
    ) -> PamResultCode {
        PamOidc::sm_authenticate(pamh, args, flags)
    }

    fn sm_close_session(
        pamh: &mut PamHandle,
        args: Vec<&std::ffi::CStr>,
        flags: PamFlag,
    ) -> PamResultCode {
        PamOidc::sm_authenticate(pamh, args, flags)
    }
}
