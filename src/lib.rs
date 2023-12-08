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

use error::PamOidcError;
use pam::{
    constants::{PamFlag, PamResultCode},
    items::AuthTok,
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
                println!("unable to retrieve user");
                return e.into();
            }
        }

        match module.get_item::<AuthTok>() {
            Ok(u) => {
                if let Some(s) = u {
                    match s.0.to_str() {
                        Ok(s) => pass = s.to_string(),
                        Err(e) => {
                            let err: PamOidcError = e.into();
                            return err.into();
                        }
                    }
                } else {
                    pass = String::from("");
                }
            }
            Err(e) => return e.into(),
        }

        // rpassword::prompt_password("Password: ").unwrap();
        // match rpassword::read_password() {
        //     Ok(p) => {
        //         println!("passwd: {}\n", p);
        //         pass = p;
        //     }
        //     Err(_) => return PamResultCode::PAM_ABORT,
        // }

        match PamOidcConfig::new() {
            Ok(c) => match c.authenticate_user(&user, &pass) {
                Ok(code) => return code,
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
