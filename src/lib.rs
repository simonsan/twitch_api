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

// (Modifications|Other (data|code)|Everything else) Copyright 2019 the libtwitch-rs authors.
//  See copying.md for further legal info.

//! # Twitch API
//!
//! Rust library for interacting with the Twitch API:
//! https://dev.twitch.tv/docs/
//!
//! # Examples
//!
//! ```
//! extern crate twitch_api;
//!
//! use twitch_api::games;
//!
//! let c = twitch_api::new("<clientid>".to_owned());
//! // Print the name of the top 20 games
//! if let Ok(games) = games::TopGames::get(&c) {
//!     for entry in games.take(20) {
//!         println!("{}: {}", entry.game.name, entry.viewers);
//!     }
//! }
//! ```

#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate hyper_rustls;
extern crate serde;
extern crate serde_json;

#[macro_use]
pub mod response;
pub mod channel_feed;
pub mod channels;
pub mod chat;
pub mod communities;
pub mod games;
pub mod ingests;
pub mod search;
pub mod streams;
pub mod teams;
pub mod users;
pub mod videos;

use response::{ApiError, ErrorResponse, TwitchResult};

use hyper::client::RequestBuilder;
use hyper::header::{qitem, Accept, Authorization, ContentType, Headers};
use hyper::mime::{Attr, Mime, SubLevel, TopLevel, Value};
use hyper::net::HttpsConnector;
use hyper::Client;

use serde::de::Deserialize;
use serde::Serialize;
use std::io::Write;
use std::io::{stderr, Read};
use std::env;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
    client_id: String,
    // add when channel_ids are a thing
    // channel_id: String,
    token: Option<String>
}

impl Credentials{
    pub fn new(cid: String) -> Credentials{
        Credentials{
            client_id: cid.clone(),
            token: None,
        }
    }

    pub fn set_from_file(&mut self, file: String){
        let file_content = match fs::read_to_string(file){
            Ok(s) => s,
            Err(e) => panic!("There was a problem reading the file: {:?}", e)
        };
        match toml::from_str::<Credentials>(&file_content){
            Ok(cred) => {
                self.client_id = cred.client_id;
                self.token = cred.token
            },
            Err(e) => panic!("There was a problem parsing the toml file: {:?}", e),
        }
    }

    pub fn write_to_file(&self, file: String){
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
        client: Client::with_connector(HttpsConnector::new(hyper_rustls::TlsClient::new())),
        cred: Credentials::new(clientid),
    }
}

impl TwitchClient {

    fn build_request<'a, F>(&self, path: &str, build: F) -> RequestBuilder<'a>
    where
        F: Fn(&str) -> RequestBuilder<'a>,
    {
        let url = String::from("https://api.twitch.tv/kraken") + path;
        let mut headers = Headers::new();

        headers.set_raw("Client-ID", vec![self.cred.client_id.clone().into_bytes()]);
        headers.set(Accept(vec![qitem(Mime(
            TopLevel::Application,
            SubLevel::Ext("vnd.twitchtv.v5+json".to_owned()),
            vec![],
        ))]));
        headers.set(ContentType(Mime(
            TopLevel::Application,
            SubLevel::Json,
            vec![(Attr::Charset, Value::Utf8)],
        )));
        if let Some(ref token) = self.cred.token {
            headers.set(Authorization(format!("OAuth {}", token)));
        }

        build(&url).headers(headers)
    }

    pub fn set_oauth_token(&mut self, token: &str) {
        self.cred.token = Some(String::from(token));
    }

    pub fn get<T: Deserialize>(&self, path: &str) -> TwitchResult<T> {
        let mut r = r#try!(self.build_request(path, |url| self.client.get(url)).send());
        let mut s = String::new();
        let _ = r#try!(r.read_to_string(&mut s));
        if s.len() == 0 {
            return Err(ApiError::empty_response());
        } else {
            match serde_json::from_str(&s) {
                Ok(x) => Ok(x),
                Err(err) => {
                    if let Ok(mut e) = serde_json::from_str::<ErrorResponse>(&s) {
                        e.cause = Some(Box::new(err));
                        return Err(ApiError::from(e));
                    }
                    writeln!(&mut stderr(), "Serde Parse Fail:\n\"{}\"", &s).unwrap();
                    Err(ApiError::from(err))
                }
            }
        }
    }

    pub fn post<T, R>(&self, path: &str, data: &T) -> TwitchResult<R>
    where
        T: Serialize,
        R: Deserialize,
    {
        let mut r = r#try!(self
            .build_request(path, |url| self.client.post(url))
            .body(&r#try!(serde_json::to_string(data)))
            .send());
        let mut s = String::new();
        let _ = r#try!(r.read_to_string(&mut s));
        if s.len() == 0 {
            return Err(ApiError::empty_response());
        } else {
            match serde_json::from_str(&s) {
                Ok(x) => Ok(x),
                Err(err) => {
                    if let Ok(mut e) = serde_json::from_str::<ErrorResponse>(&s) {
                        e.cause = Some(Box::new(err));
                        return Err(ApiError::from(e));
                    }
                    writeln!(&mut stderr(), "Serde Parse Fail:\n\"{}\"", &s).unwrap();
                    Err(ApiError::from(err))
                }
            }
        }
    }

    pub fn put<T, R>(&self, path: &str, data: &T) -> TwitchResult<R>
    where
        T: Serialize,
        R: Deserialize,
    {
        let mut r = r#try!(self
            .build_request(path, |url| self.client.put(url))
            .body(&r#try!(serde_json::to_string(data)))
            .send());
        let mut s = String::new();
        let _ = r#try!(r.read_to_string(&mut s));
        if s.len() == 0 {
            return Err(ApiError::empty_response());
        } else {
            match serde_json::from_str(&s) {
                Ok(x) => Ok(x),
                Err(err) => {
                    if let Ok(mut e) = serde_json::from_str::<ErrorResponse>(&s) {
                        e.cause = Some(Box::new(err));
                        return Err(ApiError::from(e));
                    }
                    writeln!(&mut stderr(), "Serde Parse Fail:\n\"{}\"", &s).unwrap();
                    Err(ApiError::from(err))
                }
            }
        }
    }

    pub fn delete<T: Deserialize>(&self, path: &str) -> TwitchResult<T> {
        let mut r = r#try!(self
            .build_request(path, |url| self.client.delete(url))
            .send());
        let mut s = String::new();
        let _ = r#try!(r.read_to_string(&mut s));
        if s.len() == 0 {
            return Err(ApiError::empty_response());
        } else {
            match serde_json::from_str(&s) {
                Ok(x) => Ok(x),
                Err(err) => {
                    if let Ok(mut e) = serde_json::from_str::<ErrorResponse>(&s) {
                        e.cause = Some(Box::new(err));
                        return Err(ApiError::from(e));
                    }
                    writeln!(&mut stderr(), "Serde Parse Fail:\n\"{}\"", &s).unwrap();
                    Err(ApiError::from(err))
                }
            }
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
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
    ) -> String {
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
    ) -> String {
        gen_auth_url(c, "code", redirect_url, scope, state)
    }

    pub fn imp_grant_flow(
        c: &TwitchClient,
        redirect_url: &str,
        scope: &[Scope],
        state: &str,
    ) -> String {
        gen_auth_url(c, "token", redirect_url, scope, state)
    }
}

#[cfg(test)]
mod tests {
    pub const CLIENTID: &'static str = "";
    pub const TOKEN: &'static str = "";
    pub const CHANID: &'static str = "";
    pub const TESTCH: i64 = 12826;
}
