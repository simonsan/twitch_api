// This file was ((taken|adapted)|contains (data|code)) from twitch_api,
// Copyright 2017 Matt Shanker
// It's licensed under the Apache License, Version 2.0.
// You may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// (Modifications|Other (data|code)|Everything else) Copyright 2019 the
// libtwitch-rs authors.  See copying.md for further legal info.

use std::{
    fmt,
    io,
};

use serde::Deserialize;
use thiserror::Error;

pub type TwitchResult<T> = Result<T, ApiError>;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("HTTP error")]
    ReqwestErr(reqwest::Error),
    #[error("I/O error")]
    IoError(io::Error),
    #[error("Serde error while parsing")]
    ParseError(serde_json::error::Error),
    #[error("Twitch API error")]
    TwitchError(ErrorResponse),
    #[error("Empty response")]
    EmptyResponse(EmptyResponse),
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> ApiError {
        ApiError::ReqwestErr(err)
    }
}

impl From<io::Error> for ApiError {
    fn from(err: io::Error) -> ApiError {
        ApiError::IoError(err)
    }
}

impl From<serde_json::error::Error> for ApiError {
    fn from(err: serde_json::error::Error) -> ApiError {
        ApiError::ParseError(err)
    }
}

impl From<ErrorResponse> for ApiError {
    fn from(err: ErrorResponse) -> ApiError {
        ApiError::TwitchError(err)
    }
}

impl ApiError {
    pub fn empty_response() -> ApiError {
        ApiError::EmptyResponse(EmptyResponse {})
    }
}

///////////////////////////////////////
// ErrorResponse
///////////////////////////////////////
#[derive(Deserialize, Debug)]
pub struct ErrorResponse {
    pub error: String,
    pub status: i32,
    pub message: String,
    #[serde(skip_deserializing)]
    pub cause: Option<Box<dyn std::error::Error>>,
}

impl fmt::Display for ErrorResponse {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result
    {
        write!(
            f,
            "TwitchError: (Status: {}, Error: {}, Message: {})",
            &self.status, self.error, &self.message
        )
    }
}

///////////////////////////////////////
// EmptyResponse
///////////////////////////////////////
#[derive(Debug)]
pub struct EmptyResponse {}

impl fmt::Display for EmptyResponse {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result
    {
        write!(f, "EmptyResponse")
    }
}

macro_rules! next_result {
    ($obj:ident, $url:expr, $serde:ty, $lst:ident) => {{
        let mut values_exist = false;
        if $obj.cur.is_none() {
            match $obj.client.get::<$serde>($url) {
                Ok(r) => {
                    $obj.offset += r.$lst.len() as i32;
                    $obj.cur = Some(r);
                    values_exist = true;
                }
                Err(r) => writeln!(
                    &mut std::io::stderr(),
                    "{} Error: {}",
                    stringify!($serde),
                    r
                )
                .unwrap(),
            };
        }
        else {
            values_exist = true;
        }

        match values_exist {
            true => {
                let mut x = None;
                let mut cnt = 0;
                if let Some(ref mut cur) = $obj.cur {
                    cnt = cur.$lst.len();
                    if cnt > 0 {
                        x = Some(cur.$lst.remove(0));
                        cnt -= 1;
                    }
                }
                if cnt == 0 {
                    $obj.cur = None;
                }
                x
            }
            false => None,
        }
    }};
}

macro_rules! next_result_cursor {
    ($obj:ident, $url:expr, $serde:ty, $lst:ident) => {{
        let mut values_exist = false;
        if $obj.cur.is_none() {
            let mut new_url = $url.clone();
            {
                if let Some(ref cursor) = $obj.cursor {
                    if cursor.len() == 0 {
                        return None;
                    }
                    else {
                        new_url.push_str("&cursor=");
                        new_url.push_str(cursor.clone().as_str());
                    }
                }
            }
            match $obj.client.get::<$serde>(&new_url) {
                Ok(r) => {
                    if r.$lst.len() > 0 {
                        values_exist = true;
                    }
                    else {
                        values_exist = false;
                    }
                    $obj.cursor = r._cursor.clone();
                    $obj.cur = Some(r);
                }
                Err(r) => writeln!(
                    &mut std::io::stderr(),
                    "{} Error: {}",
                    stringify!($serde),
                    r
                )
                .unwrap(),
            };
        }
        else {
            values_exist = true;
        }

        match values_exist {
            true => {
                let mut x = None;
                let mut cnt = 0;
                if let Some(ref mut cur) = $obj.cur {
                    cnt = cur.$lst.len();
                    if cnt > 0 {
                        x = Some(cur.$lst.remove(0));
                        cnt -= 1;
                    }
                }
                if cnt == 0 {
                    $obj.cur = None;
                }
                x
            }
            false => None,
        }
    }};
}
