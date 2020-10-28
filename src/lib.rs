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

//! # libtwitch-rs
//!
//! Rust library for interacting with the Twitch API:
//! https://dev.twitch.tv/docs/
//!
//! # Examples
//!
//! ```
//! extern crate libtwitch_rs;
//!
//! use libtwitch_rs::kraken::games::*;
//!
//! let c = libtwitch_rs::new("<clientid>".to_owned());
//! // Print the name of the top 20 games
//! if let Ok(games) = games::TopGames::get(&c) {
//!     for entry in games.take(20) {
//!         println!("{}: {}", entry.game.name, entry.viewers);
//!     }
//! }
//! ```
#![recursion_limit = "512"]

#[macro_use]
pub mod response;
pub mod kraken;

use serde::{
    de::DeserializeOwned,
    Deserialize,
    Serialize,
};

use response::{
    ApiError,
    TwitchResult,
};

use reqwest::{
    blocking::{
        Client,
        RequestBuilder,
    },
    header::{
        HeaderMap,
        HeaderName,
        HeaderValue,
        ACCEPT,
        AUTHORIZATION,
        CONTENT_TYPE,
    },
};

use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
    pub client_id: String,
    // pub channel_id: String,
    pub token: String,
}

impl Credentials {
    pub fn new(clid: String) -> Credentials {
        Credentials {
            client_id: clid,
            // channel_id: None,
            token: "".to_string(),
        }
    }

    pub fn set_from_file(file: String) -> Credentials {
        let file_content = match fs::read_to_string(file) {
            Ok(s) => s,
            Err(e) => panic!("There was a problem reading the file: {:?}", e),
        };
        match toml::from_str::<Credentials>(&file_content) {
            Ok(cred) => Credentials {
                client_id: cred.client_id,
                // channel_id: cred.channel_id,
                token: cred.token,
            },
            Err(e) => {
                panic!("There was a problem parsing the toml file: {:?}", e)
            }
        }
    }

    pub fn write_to_file(
        &self,
        file: String,
    )
    {
        let content = toml::to_string(self).unwrap();
        fs::write(file, content).expect("Error writing toml file");
    }
}

#[derive(Debug)]
pub struct TwitchClient {
    client: Client,
    cred: Credentials,
}

pub fn new(clientid: String) -> TwitchClient {
    TwitchClient {
        client: Client::new(),
        cred: Credentials::new(clientid),
    }
}

impl TwitchClient {
    fn build_request<F>(
        &self,
        path: &str,
        build: F,
    ) -> RequestBuilder
    where
        F: Fn(&str) -> RequestBuilder,
    {
        let url = String::from("https://api.twitch.tv/kraken") + path;
        let oauth = format!("OAuth {}", self.cred.token);
        let cid = format!("{:#?}", self.cred.client_id);

        let mut headers = HeaderMap::new();

        headers.insert(
            HeaderName::from_static("CLIENT-ID"),
            HeaderValue::from_str(&cid).unwrap(),
        );
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=UTF-8"),
        );
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/vnd.twitchtv.v5+json"),
        );
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&oauth).unwrap());

        build(&url).headers(headers)
    }

    pub fn set_oauth_token(
        &mut self,
        token: &str,
    )
    {
        self.cred.token = String::from(token);
    }

    pub fn get<T: DeserializeOwned>(
        &self,
        path: &str,
    ) -> TwitchResult<T>
    {
        let r = self
            .build_request(path, |url| self.client.get(url))
            .send()?
            .error_for_status();

        match r {
            Err(err) => Err(ApiError::from(err)),
            Ok(x) => match x.json() {
                Err(err) => Err(ApiError::from(err)),
                Ok(x) => Ok(x),
            },
        }
    }

    pub fn post<T, R>(
        &self,
        path: &str,
        data: &T,
    ) -> TwitchResult<R>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let r = self
            .build_request(path, |url| self.client.post(url))
            .json(&data)
            .send()?
            .error_for_status();

        match r {
            Err(err) => Err(ApiError::from(err)),
            Ok(x) => match x.json() {
                Err(err) => Err(ApiError::from(err)),
                Ok(x) => Ok(x),
            },
        }
    }

    pub fn put<T, R>(
        &self,
        path: &str,
        data: &T,
    ) -> TwitchResult<R>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let r = self
            .build_request(path, |url| self.client.put(url))
            .json(&data)
            .send()?
            .error_for_status();

        match r {
            Err(err) => Err(ApiError::from(err)),
            Ok(x) => match x.json() {
                Err(err) => Err(ApiError::from(err)),
                Ok(x) => Ok(x),
            },
        }
    }

    pub fn delete<T: DeserializeOwned>(
        &self,
        path: &str,
    ) -> TwitchResult<T>
    {
        let r = self
            .build_request(path, |url| self.client.delete(url))
            .send()?
            .error_for_status();

        match r {
            Err(err) => Err(ApiError::from(err)),
            Ok(x) => match x.json() {
                Err(err) => Err(ApiError::from(err)),
                Ok(x) => Ok(x),
            },
        }
    }
}

pub mod auth {
    use std::fmt;

    use super::TwitchClient;

    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    pub enum Scope {
        channel_check_subscription,
        channel_commercial,
        channel_editor,
        channel_feed_edit,
        channel_feed_read,
        channel_read,
        channel_stream,
        channel_subscriptions,
        chat_login,
        user_blocks_edit,
        user_blocks_read,
        user_follows_edit,
        user_read,
        user_subscriptions,
        viewing_activity_ready,
    }

    impl fmt::Display for Scope {
        fn fmt(
            &self,
            f: &mut fmt::Formatter,
        ) -> fmt::Result
        {
            fmt::Debug::fmt(self, f)
        }
    }

    // TODO: replace with:
    // https://doc.rust-lang.org/std/slice/trait.SliceConcatExt.html
    fn format_scope(scopes: &[Scope]) -> String {
        let mut res = String::with_capacity(27 * scopes.len());
        for scope in scopes.iter() {
            res.push_str(&scope.to_string());
            res.push('+');
        }
        res.trim_end_matches('+').to_owned()
    }

    fn gen_auth_url(
        c: &TwitchClient,
        rtype: &str,
        redirect_url: &str,
        scope: &[Scope],
        state: &str,
    ) -> String
    {
        String::from("https://api.twitch.tv/kraken/oauth2/authorize")
            + "?response_type="
            + rtype
            + "&client_id="
            + &c.cred.client_id
            + "&redirect_uri="
            + redirect_url
            + "&scope="
            + &format_scope(scope)
            + "&state="
            + state
    }

    pub fn auth_code_flow(
        c: &TwitchClient,
        redirect_url: &str,
        scope: &[Scope],
        state: &str,
    ) -> String
    {
        gen_auth_url(c, "code", redirect_url, scope, state)
    }

    pub fn imp_grant_flow(
        c: &TwitchClient,
        redirect_url: &str,
        scope: &[Scope],
        state: &str,
    ) -> String
    {
        gen_auth_url(c, "token", redirect_url, scope, state)
    }
}

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[allow(dead_code)]
mod tests {

    include!("../credentials.rs");
}
