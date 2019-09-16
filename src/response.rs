// Copyright 2017 Matt Shanker
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate hyper;
extern crate serde_json;

use std::error::Error;
use std::fmt;
use std::io;

pub type TwitchResult<T> = Result<T, ApiError>;

#[derive(Debug)]
pub enum ApiError {
    HyperErr(hyper::error::Error),
    IoError(io::Error),
    ParseError(serde_json::error::Error),
    TwitchError(ErrorResponse),
    EmptyResponse(EmptyResponse),
}

impl From<hyper::error::Error> for ApiError {
    fn from(err: hyper::error::Error) -> ApiError {
        ApiError::HyperErr(err)
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

impl Error for ApiError {
    fn description(&self) -> &str {
        match *self {
            ApiError::HyperErr(ref err) => err.description(),
            ApiError::IoError(ref err) => err.description(),
            ApiError::ParseError(ref err) => err.description(),
            ApiError::TwitchError(ref err) => &err.error,
            ApiError::EmptyResponse(_) => "EmptyResponse",
        }
    }

    fn cause(&self) -> Option<&Error> {
        Some(match *self {
            ApiError::HyperErr(ref err) => err as &Error,
            ApiError::IoError(ref err) => err as &Error,
            ApiError::ParseError(ref err) => err as &Error,
            ApiError::TwitchError(ref err) => err as &Error,
            ApiError::EmptyResponse(ref err) => err as &Error,
        })
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApiError::HyperErr(ref err) => fmt::Display::fmt(err, f),
            ApiError::IoError(ref err) => fmt::Display::fmt(err, f),
            ApiError::ParseError(ref err) => fmt::Display::fmt(err, f),
            ApiError::TwitchError(ref err) => fmt::Display::fmt(err, f),
            ApiError::EmptyResponse(ref err) => fmt::Display::fmt(err, f),
        }
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
    pub cause: Option<Box<Error>>,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TwitchError: (Status: {}, Error: {}, Message: {})",
            &self.status, self.error, &self.message
        )
    }
}

impl Error for ErrorResponse {
    fn description(&self) -> &str {
        &self.error
    }

    fn cause(&self) -> Option<&Error> {
        if let Some(ref cause) = self.cause {
            return Some(cause.as_ref());
        } else {
            None
        }
    }
}

///////////////////////////////////////
// EmptyResponse
///////////////////////////////////////
#[derive(Debug)]
pub struct EmptyResponse {}

impl fmt::Display for EmptyResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EmptyResponse")
    }
}

impl Error for EmptyResponse {
    fn description(&self) -> &str {
        "EmptyResponse"
    }

    fn cause(&self) -> Option<&Error> {
        None
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
        } else {
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
                    } else {
                        new_url.push_str("&cursor=");
                        new_url.push_str(cursor.clone().as_str());
                    }
                }
            }
            match $obj.client.get::<$serde>(&new_url) {
                Ok(r) => {
                    if r.$lst.len() > 0 {
                        values_exist = true;
                    } else {
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
        } else {
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
