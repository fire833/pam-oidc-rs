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

use oauth2::{
    basic::BasicErrorResponseType, reqwest::Error, RequestTokenError, StandardErrorResponse,
};
use openidconnect::DiscoveryError;
use pam::constants::PamResultCode;
use std::fmt::{Debug, Display};
use std::io;
use std::str;

#[derive(Debug)]
pub enum PamOidcError {
    UrlParseError(url::ParseError),
    Utf8Error(str::Utf8Error),
    RequestTokenError(
        RequestTokenError<Error<reqwest::Error>, StandardErrorResponse<BasicErrorResponseType>>,
    ),
    DiscoveryError(DiscoveryError<oauth2::reqwest::Error<reqwest::Error>>),
    ConfigRetrievalError(io::Error),
    ConfigUnmarshalError(serde_yaml::Error),
    Internal(String),
}

impl PamOidcError {
    #[allow(unused)]
    pub fn wrap_err<T, E>(result: Result<T, E>) -> Result<T, PamOidcError>
    where
        E: Debug + Into<PamOidcError>,
    {
        match result {
            Ok(res) => Ok(res),
            Err(e) => Err(e.into()),
        }
    }

    #[allow(unused)]
    pub fn new_internal(msg: String) -> Self {
        Self::Internal(msg)
    }
}

impl Display for PamOidcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UrlParseError(e) => write!(f, "url parse error: {}", e),
            Self::RequestTokenError(e) => match e {
                RequestTokenError::ServerResponse(resp) => {
                    write!(f, "request token error (server response): {}", resp)
                }
                RequestTokenError::Parse(err, data) => {
                    write!(f, "request token error (parse): {} - {:?}", err, data)
                }
                RequestTokenError::Request(req) => {
                    write!(f, "request token error (request): {}", req)
                }
                RequestTokenError::Other(o) => {
                    write!(f, "request token error (other): {}", o)
                }
            },
            Self::Internal(e) => write!(f, "{}", e),
            Self::ConfigRetrievalError(e) => write!(f, "config retrieve error: {}", e),
            Self::ConfigUnmarshalError(e) => write!(f, "config unmarshal error: {}", e),
            Self::DiscoveryError(e) => write!(f, "discovery error: {}", e),
            Self::Utf8Error(e) => write!(f, "utf8error: {}", e),
        }
    }
}

impl Into<PamResultCode> for PamOidcError {
    fn into(self) -> PamResultCode {
        match self {
            PamOidcError::Internal(_) => PamResultCode::PAM_ABORT,
            PamOidcError::UrlParseError(_) => PamResultCode::PAM_ABORT,
            PamOidcError::ConfigRetrievalError(_) => PamResultCode::PAM_OPEN_ERR,
            PamOidcError::RequestTokenError(_) => PamResultCode::PAM_AUTH_ERR,
            PamOidcError::ConfigUnmarshalError(_) => PamResultCode::PAM_ABORT,
            PamOidcError::DiscoveryError(_) => PamResultCode::PAM_AUTHINFO_UNAVAIL,
            PamOidcError::Utf8Error(_) => PamResultCode::PAM_AUTHTOK_ERR,
        }
    }
}

impl From<url::ParseError> for PamOidcError {
    fn from(value: url::ParseError) -> Self {
        PamOidcError::UrlParseError(value)
    }
}

impl From<RequestTokenError<Error<reqwest::Error>, StandardErrorResponse<BasicErrorResponseType>>>
    for PamOidcError
{
    fn from(
        value: RequestTokenError<
            Error<reqwest::Error>,
            StandardErrorResponse<BasicErrorResponseType>,
        >,
    ) -> Self {
        PamOidcError::RequestTokenError(value)
    }
}

impl From<io::Error> for PamOidcError {
    fn from(value: io::Error) -> Self {
        PamOidcError::ConfigRetrievalError(value)
    }
}

impl From<serde_yaml::Error> for PamOidcError {
    fn from(value: serde_yaml::Error) -> Self {
        PamOidcError::ConfigUnmarshalError(value)
    }
}

impl From<DiscoveryError<oauth2::reqwest::Error<reqwest::Error>>> for PamOidcError {
    fn from(value: DiscoveryError<oauth2::reqwest::Error<reqwest::Error>>) -> Self {
        PamOidcError::DiscoveryError(value)
    }
}

impl From<str::Utf8Error> for PamOidcError {
    fn from(value: str::Utf8Error) -> Self {
        PamOidcError::Utf8Error(value)
    }
}
