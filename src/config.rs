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

use std::fs;

use oauth2::{
    reqwest::http_client, ClientId, ClientSecret, ResourceOwnerPassword, ResourceOwnerUsername,
};
use openidconnect::{
    core::{CoreClient, CoreProviderMetadata},
    IssuerUrl,
};
use pam::constants::PamResultCode;
use serde::Deserialize;

use crate::error::PamOidcError;

#[derive(Deserialize)]
pub struct PamOidcConfig {
    client_id: String,
    client_secret: String,
    issuer_url: String,
}

pub const PAM_OIDC_CONFIG: &str = "/etc/pam_oidc/config.yaml";

impl PamOidcConfig {
    pub fn new() -> Result<PamOidcConfig, PamOidcError> {
        let data = fs::read_to_string(PAM_OIDC_CONFIG)?;
        match serde_yaml::from_str::<PamOidcConfig>(&data) {
            Ok(conf) => return Ok(conf),
            Err(e) => return Err(e.into()),
        }
    }

    // pub fn new_from_args(args: &[String]) -> io::Result<PamOidcConfig> {
    //     let mut client_id: String;
    //     let mut client_secret: String;
    //     let mut auth_url: String;
    //     let mut token_url: String;

    //     let cid_name = String::from("client_id");
    //     let cs_name = String::from("client_secret");
    //     let aurl_name = String::from("auth_url");
    //     let turl_name = String::from("token_url");

    //     let mut found = false;

    //     for (i, arg) in args.iter().enumerate() {
    //         match arg.to_owned() {
    //             cid_name if !found => {
    //                 client_id = args[i + 1];
    //                 found = true;
    //             }
    //             cs_name if !found => {
    //                 client_secret = args[i + 1];
    //                 found = true;
    //             }
    //             aurl_name => {
    //                 auth_url = args[i + 1];
    //                 found = true;
    //             }
    //             turl_name => {
    //                 token_url = args[i + 1];
    //                 found = true;
    //             }
    //             _ if found => {
    //                 found = false;
    //             }
    //             _ => {}
    //         }
    //     }

    //     Ok(Self {
    //         client_id,
    //         client_secret,
    //         auth_url,
    //         token_url,
    //     })
    // }

    pub fn authenticate_user(&self, user: &str, pass: &str) -> Result<PamResultCode, PamOidcError> {
        let provider_meta = CoreProviderMetadata::discover(
            &IssuerUrl::new(self.issuer_url.to_owned())?,
            http_client,
        );

        match provider_meta {
            Ok(meta) => {
                let client = CoreClient::from_provider_metadata(
                    meta,
                    ClientId::new(self.client_id.to_owned()),
                    Some(ClientSecret::new(self.client_secret.to_owned())),
                );

                let username = ResourceOwnerUsername::new(user.to_string());
                let password = ResourceOwnerPassword::new(pass.to_string());

                let req = client.exchange_password(&username, &password);

                match req.request(http_client) {
                    Ok(_) => return Ok(PamResultCode::PAM_SUCCESS),
                    Err(e) => {
                        return Err(e.into());
                    }
                }
            }
            Err(e) => return Err(e.into()),
        }
    }
}
