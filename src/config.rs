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

use std::{fs, io};

use oauth2::{
    basic::BasicClient, reqwest::http_client, AuthUrl, ClientId, ClientSecret,
    ResourceOwnerPassword, ResourceOwnerUsername, Scope, TokenUrl,
};
use serde::Deserialize;

use crate::bindings::{pam_ext::PAM_ABORT, pam_modules::PAM_SUCCESS};

#[derive(Deserialize)]
pub struct PamOidcConfig {
    client_id: String,
    client_secret: String,
    auth_url: String,
    token_url: String,
}

pub const PAM_OIDC_CONFIG: &str = "/etc/pam_oidc/config.toml";

impl PamOidcConfig {
    pub fn new() -> io::Result<PamOidcConfig> {
        let data = fs::read_to_string(PAM_OIDC_CONFIG)?;
        match serde_yaml::from_str::<PamOidcConfig>(&data) {
            Ok(conf) => return Ok(conf),
            Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e)),
        }
    }

    pub fn authorize_user(&self, user: &str, pass: &str) -> Result<i32, url::ParseError> {
        let client = BasicClient::new(
            ClientId::new(self.client_id.to_string()),
            Some(ClientSecret::new(self.client_secret)),
            AuthUrl::new(self.auth_url)?,
            Some(TokenUrl::new(self.token_url)?),
        );

        let resp = client
            .exchange_password(
                &ResourceOwnerUsername::new(user.to_string()),
                &ResourceOwnerPassword::new(pass.to_string()),
            )
            .add_scope(Scope::new("openid".to_string()))
            .request(http_client);

        match resp {
            Ok(resp) => Ok(PAM_SUCCESS as i32),
            Err(e) => Err(_),
        }
    }
}
