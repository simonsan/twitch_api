#![feature(in_band_lifetimes)]
// This file was adapted from twitch_api,
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

// Modifications Copyright 2020 the libtwitch-rs authors.
// See copying.md for further legal info.

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
pub mod auth;
pub mod kraken;

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
	StatusCode,
};

use response::{
	ApiError,
	ErrorResponse,
	TwitchResult,
};

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
			None => Credentials::set_from_env(),
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

	fn set_from_env() -> Credentials {
		Credentials {
			client_id: Some(env::var("TWITCH_CLIENT_ID").unwrap_or_default()),
			token: Some(env::var("TWITCH_OAUTH_TOKEN").unwrap_or_derault()),
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

pub fn new(file: Option<String>) -> TwitchClient {
	TwitchClient {
		client: reqwest::Client::builder().use_rustls_tls().build().unwrap(),
		cred: Credentials::new(Option::from(file.unwrap())),
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

	pub async fn get(
		&self,
		path: &str,
	) -> TwitchResult<R>
	{
		let response = self
			.client
			.build_request(path, url)
			.get()
			.send()
			.await?
			.json()
			.await?;

		// TODO: Handle other status codes gracefully
		assert!(StatusCode::OK.is_success());

		match response {
			None => Err(ApiError::empty_response()),
			Some(R) => Ok(R),
		}
	}

	pub async fn post<T>(
		&self,
		path: &str,
		data: &T,
	) -> TwitchResult<R>
	where
		T: Serialize,
	{
		let response = self
			.client
			.build_request(path, url)
			.post()
			.json(data)
			.send()
			.await?
			.json()
			.await?;

		// TODO: Handle other status codes gracefully
		assert!(StatusCode::OK.is_success());

		match response {
			None => Err(ApiError::empty_response()),
			Some(R) => Ok(R),
		}
	}

	pub async fn put<T>(
		&self,
		path: &str,
		data: &T,
	) -> TwitchResult<R>
	where
		T: Serialize,
	{
		let response = self
			.client
			.build_request(path, url)
			.put()
			.json(data)
			.send()
			.await?
			.json()
			.await?;

		// TODO: Handle other status codes gracefully
		assert!(StatusCode::OK.is_success());

		match response {
			None => Err(ApiError::empty_response()),
			Some(R) => Ok(R),
		}
	}

	pub async fn delete<T>(
		&self,
		path: &str,
		data: &T,
	) -> TwitchResult<R>
	where
		T: Serialize,
	{
		// TODO: delete implement
		let response = self
			.client
			.build_request(path, url)
			.put()
			.json(data)
			.send()
			.await?
			.json()
			.await?;

		// TODO: Handle other status codes gracefully
		assert!(StatusCode::OK.is_success());

		match response {
			None => Err(ApiError::empty_response()),
			Some(R) => Ok(R),
		}
	}
}

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[allow(dead_code)]
mod tests {

	include!("../credentials.rs");
}
