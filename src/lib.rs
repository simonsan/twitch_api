#![feature(in_band_lifetimes)]
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
//! You can either set the TWITCH_CLIENT_ID, TWITCH_OAUTH_TOKEN environment
//! variables on your system or pass a path to new() in which you specify the
//! client-id and oauth token. You can also set it with
//! TwitchClient::set_oauth_token().
//!
//! ```
//! extern crate libtwitch_rs;
//!
//! use libtwitch_rs::*;
//!
//! let c = libtwitch_rs::new("<path-to-credential.rs>".to_owned());
//!
//! // Print the name of the top 20 games
//! if let Ok(games) = games::TopGames::get(&c) {
//! 	for entry in games.take(20) {
//! 		println!("{}: {}", entry.game.name, entry.viewers);
//! 		}
//! 	}
//! ```
#![recursion_limit = "512"]

extern crate reqwest;

#[macro_use]
extern crate serde;
extern crate serde_json;

#[macro_use]
pub mod response;
pub mod kraken;

use http::StatusCode;

use reqwest::{
	header,
	header::{
		HeaderMap,
		HeaderName,
		HeaderValue,
		ACCEPT,
		AUTHORIZATION,
	},
	Client,
	Error,
	RequestBuilder,
};

use response::{
	ApiError,
	ErrorResponse,
	TwitchResult,
};

use crate::response::ApiError::HyperErr;
use serde::{
	de::Deserialize,
	Serialize,
};
use std::{
	convert::TryFrom,
	env,
	fs,
	future,
	io::{
		stderr,
		Read,
		Write,
	},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
	pub client_id: Option<String>,
	pub token: Option<String>,
	// pub channel_id: String,
}

impl Credentials {
	pub fn new(file: Option<String>) -> Credentials {
		// if the Environment variables are not set, set it from file
		match file {
			Some(p) => Credentials::set_from_file(p),
			None => Credentials {
				client_id: Some(
					env::var("TWITCH_CLIENT_ID").unwrap_or_default(),
				),
				token: Some(env::var("TWITCH_OAUTH_TOKEN").unwrap_or_default()),
			},
		}
	}

	pub fn save(
		&self,
		path: String,
	)
	{
		Credentials::write_to_file(&self, path)
	}

	fn set_from_file(file: String) -> Credentials {
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

	fn write_to_file(
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

pub fn new(file: String) -> TwitchClient {
	TwitchClient {
		client: Client::builder().use_rustls_tls().build().unwrap(),
		cred: Credentials::new(Option::from(file)),
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
		// This is for the old API v5
		let root_url = "https://api.twitch.tv/kraken".to_string() + path;

		let mut headers = HeaderMap::new();

		headers.insert(
			ACCEPT,
			"application/vnd.twitchtv.v5+json".parse().unwrap(),
		);

		headers.insert(
			"Client-ID",
			HeaderValue::try_from(
				self.cred.client_id.clone().unwrap().into_bytes(),
			)
			.unwrap(),
		);

		headers.insert(
			AUTHORIZATION,
			format!("OAuth {}", self.cred.token.clone().unwrap())
				.parse()
				.unwrap(),
		);

		// TODO
		// headers.set(ContentType(Mime(
		// 	TopLevel::Application,
		// 	SubLevel::Json,
		// 	vec![(Attr::Charset, Value::Utf8)],
		// )));

		build(&root_url).headers(headers)
	}

	pub fn set_oauth_token(
		&mut self,
		token: &str,
	)
	{
		self.cred.token = Some(String::from(token));
	}

	pub async fn get<T: Deserialize<'de>>(
		&self,
		path: &str,
	) -> TwitchResult<T>
	{
		let mut r = self
			.build_request(path, |url| self.client.get(url.parse().unwrap()))
			.send()
			.await?
			.json()
			.await?;

		assert!(StatusCode::OK.is_success());

		match r {
			None => Err(ApiError::empty_response()),
			Some(T) => Ok(T),
		}
	}
}

pub async fn post<T, R>(
	&self,
	path: &str,
	data: &T,
) -> TwitchResult<R>
where
	T: Serialize,
	R: Deserialize<'de>,
{
	let mut r = self
		.build_request(path, |url| self.client.post(url))
		.body(&serde_json::to_string(data)?)
		.send()
		.await?;
	let mut s = String::new();
	let _ = r.read_to_string(&mut s)?;
	if s.is_empty() {
		Err(ApiError::empty_response())
	}
	else {
		match serde_json::from_str(&s) {
			Ok(x) => Ok(x),
			Err(err) => {
				if let Ok(mut e) = serde_json::from_str::<ErrorResponse>(&s) {
					e.cause = Some(Box::new(err));
					return Err(ApiError::from(e));
				}
				writeln!(&mut stderr(), "Serde Parse Fail:\n\"{}\"", &s)
					.unwrap();
				Err(ApiError::from(err))
			}
		}
	}
}

pub async fn put<T, R>(
	&self,
	path: &str,
	data: &T,
) -> TwitchResult<R>
where
	T: Serialize,
	R: Deserialize<'de>,
{
	let mut r = self
		.build_request(path, |url| self.client.put(url))
		.body(&serde_json::to_string(data)?)
		.send()
		.await?;
	let mut s = String::new();
	let _ = r.read_to_string(&mut s)?;
	if s.is_empty() {
		Err(ApiError::empty_response())
	}
	else {
		match serde_json::from_str(&s) {
			Ok(x) => Ok(x),
			Err(err) => {
				if let Ok(mut e) = serde_json::from_str::<ErrorResponse>(&s) {
					e.cause = Some(Box::new(err));
					return Err(ApiError::from(e));
				}
				writeln!(&mut stderr(), "Serde Parse Fail:\n\"{}\"", &s)
					.unwrap();
				Err(ApiError::from(err))
			}
		}
	}
}

pub async fn delete<T: Deserialize<'de>>(
	&self,
	path: &str,
) -> TwitchResult<T>
{
	let mut r = self
		.build_request(path, |url| self.client.delete(url))
		.send()
		.await?;
	let mut s = String::new();
	let _ = r.read_to_string(&mut s)?;
	if s.is_empty() {
		Err(ApiError::empty_response())
	}
	else {
		match serde_json::from_str(&s) {
			Ok(x) => Ok(x),
			Err(err) => {
				if let Ok(mut e) = serde_json::from_str::<ErrorResponse>(&s) {
					e.cause = Some(Box::new(err));
					return Err(ApiError::from(e));
				}
				writeln!(&mut stderr(), "Serde Parse Fail:\n\"{}\"", &s)
					.unwrap();
				Err(ApiError::from(err))
			}
		}
	}
}

pub mod auth {
	use std::fmt;

	use super::TwitchClient;
	use std::fmt::Debug;

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
			+ rtype + "&client_id="
			+ &c.cred.client_id
			+ "&redirect_uri="
			+ redirect_url
			+ "&scope=" + &format_scope(scope)
			+ "&state=" + state
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
