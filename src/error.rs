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

use oauth2::{
    basic::BasicErrorResponseType, reqwest::Error, RequestTokenError, StandardErrorResponse,
};
use pam::constants::PamResultCode;
use std::fmt::{Debug, Display};
use std::io;

#[derive(Debug)]
pub enum PamOidcError {
    UrlParseError(url::ParseError),
    RequestTokenError(
        RequestTokenError<Error<reqwest::Error>, StandardErrorResponse<BasicErrorResponseType>>,
    ),
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
            Self::UrlParseError(e) => write!(f, "{}", e),
            Self::RequestTokenError(e) => write!(f, "{}", e),
            Self::Internal(e) => write!(f, "{}", e),
            Self::ConfigRetrievalError(e) => write!(f, "{}", e),
            Self::ConfigUnmarshalError(e) => write!(f, "{}", e),
        }
    }
}

impl Into<PamResultCode> for PamOidcError {
    fn into(self) -> PamResultCode {
        match self {
            PamOidcError::Internal(_) => PamResultCode::PAM_ABORT,
            PamOidcError::UrlParseError(_) => PamResultCode::PAM_ABORT,
            PamOidcError::ConfigRetrievalError(_) => PamResultCode::PAM_OPEN_ERR,
            PamOidcError::RequestTokenError(_) => PamResultCode::PAM_ABORT,
            PamOidcError::ConfigUnmarshalError(_) => PamResultCode::PAM_ABORT,
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
