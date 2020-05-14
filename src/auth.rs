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
		+ Some(&c.cred.client_id.clone()).unwrap()
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
