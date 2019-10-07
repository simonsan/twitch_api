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

extern crate chrono;
extern crate serde_json;

use super::response::TwitchResult;
use super::users::User;
use super::TwitchClient;

use serde_json::Value;
use std;
use std::collections::HashMap;
use std::io::Write;

/// Gets a specified community
///
/// The name of the community is specified in
/// a required query-string parameter. It must
/// be 3-25 characters.
///
/// #### Authentication: `None`
///
pub fn get_by_name(c: &TwitchClient, name: &str) -> TwitchResult<Community> {
    let r = r#try!(c.get::<Community>(&format!("/communities?name={}", name)));
    Ok(r)
}

/// Gets a specified community
///
/// #### Authentication: `None`
///
pub fn get_by_id(c: &TwitchClient, id: &str) -> TwitchResult<Community> {
    let r = r#try!(c.get::<Community>(&format!("/communities/{}", id)));
    Ok(r)
}

/// Updates a specified community
///
/// #### Authentication: `communities_edit`
///
pub fn update<'a>(
    c: &TwitchClient,
    community_id: &str,
    data: &'a UpdateSettings,
) -> TwitchResult<Community> {
    let mut settings: HashMap<String, &str> = HashMap::new();
    if let Some(summary) = data.summary {
        settings.insert("summary".to_owned(), summary);
    }
    if let Some(description) = data.description {
        settings.insert("description".to_owned(), description);
    }
    if let Some(rules) = data.rules {
        settings.insert("rules".to_owned(), rules);
    }
    if let Some(email) = data.email {
        settings.insert("email".to_owned(), email);
    }
    let r = r#try!(c.put::<HashMap<String, &str>, Community>(
        &format!("/communities/{}", community_id),
        &settings
    ));
    Ok(r)
}

/// Gets a list of banned users for a specified community
///
/// #### Authentication: `communities_moderate`
///
pub fn bans<'c>(c: &'c TwitchClient, community_id: &str) -> TwitchResult<CommunityBanIterator<'c>> {
    let iter = CommunityBanIterator {
        client: c,
        community_id: String::from(community_id),
        cur: None,
        cursor: None,
    };
    Ok(iter)
}

/// Adds a specified user to the ban list of a specified community
///
/// #### Authentication: `communities_moderate`
///
pub fn ban(c: &TwitchClient, community_id: &str, user_id: &str) -> TwitchResult<Value> {
    let r = r#try!(c.put::<Value, Value>(
        &format!("/communities/{}/bans/{}", community_id, user_id),
        &Value::Null
    ));
    Ok(r)
}

/// Deletes a specified user from the ban list of a specified community
///
/// #### Authentication: `communities_moderate`
///
pub fn unban(c: &TwitchClient, community_id: &str, user_id: &str) -> TwitchResult<Value> {
    let r = r#try!(c.delete::<Value>(&format!("/communities/{}/bans/{}", community_id, user_id)));
    Ok(r)
}

/// Adds a specified image as the avatar of a specified community
///
/// #### Authentication: `communities_edit`
///
pub fn set_avatar_image(
    c: &TwitchClient,
    community_id: &str,
    avatar_img: &str,
) -> TwitchResult<Value> {
    let mut data: HashMap<String, &str> = HashMap::new();
    data.insert("avatar_image".to_owned(), avatar_img);
    let r = r#try!(c.post::<HashMap<String, &str>, Value>(
        &format!("/communities/{}/images/avatar", community_id),
        &data
    ));
    Ok(r)
}

/// Deletes the avatar image of a specified community
///
/// #### Authentication: `communities_edit`
///
pub fn delete_avatar_image(c: &TwitchClient, community_id: &str) -> TwitchResult<Value> {
    let r = r#try!(c.delete::<Value>(&format!("/communities/{}/images/avatar", community_id)));
    Ok(r)
}

/// Adds a specified image as the cover image of a specified community
///
/// #### Authentication: `communities_edit`
///
pub fn set_cover_image(
    c: &TwitchClient,
    community_id: &str,
    cover_img: &str,
) -> TwitchResult<Value> {
    let mut data: HashMap<String, &str> = HashMap::new();
    data.insert("cover_image".to_owned(), cover_img);
    let r = r#try!(c.post::<HashMap<String, &str>, Value>(
        &format!("/communities/{}/images/cover", community_id),
        &data
    ));
    Ok(r)
}

/// Deletes the cover image of a specified community
///
/// #### Authentication: `communities_edit`
///
pub fn delete_cover_image(c: &TwitchClient, community_id: &str) -> TwitchResult<Value> {
    let r = r#try!(c.delete::<Value>(&format!("/communities/{}/images/cover", community_id)));
    Ok(r)
}

/// Gets a list of moderators of a specified community
///
/// #### Authentication: `communities_edit`
///
pub fn moderators(c: &TwitchClient, community_id: &str) -> TwitchResult<Moderators> {
    let r = r#try!(c.get::<Moderators>(&format!("/communities/{}/moderators", community_id)));
    Ok(r)
}

/// Adds a specified user to the list of moderators of a specified community
///
/// #### Authentication: `communities_edit`
///
pub fn new_moderator(c: &TwitchClient, community_id: &str, user_id: &str) -> TwitchResult<Value> {
    let r = r#try!(c.put::<Value, Value>(
        &format!("/communities/{}/moderators/{}", community_id, user_id),
        &Value::Null
    ));
    Ok(r)
}

/// Deletes a specified user from the list of moderators of a specified community
///
/// #### Authentication: `communities_edit`
///
pub fn delete_moderator(
    c: &TwitchClient,
    community_id: &str,
    user_id: &str,
) -> TwitchResult<Value> {
    let r = r#try!(c.delete::<Value>(&format!(
        "/communities/{}/moderators/{}",
        community_id, user_id
    )));
    Ok(r)
}

/// Gets a list of actions users can perform in a specified community
///
/// #### Authentication: `Any`
///
pub fn permissions(c: &TwitchClient, community_id: &str) -> TwitchResult<HashMap<String, bool>> {
    let r = r#try!(
        c.get::<HashMap<String, bool>>(&format!("/communities/{}/permissions", community_id))
    );
    Ok(r)
}

/// Reports a specified channel for violating the rules of a specified community
///
/// #### Authentication: `None`
///
pub fn report_channel(
    c: &TwitchClient,
    community_id: &str,
    channel_id: &str,
) -> TwitchResult<Value> {
    let mut data: HashMap<String, &str> = HashMap::new();
    data.insert("channel_id".to_owned(), channel_id);
    let r = r#try!(c.post::<HashMap<String, &str>, Value>(
        &format!("/communities/{}/report_channel", community_id),
        &data
    ));
    Ok(r)
}

/// Gets a list of users who are timed out in a specified community
///
/// #### Authentication: `communities_moderate`
///
pub fn timeouts<'c>(c: &'c TwitchClient, community_id: &str) -> TwitchResult<TimeoutIterator<'c>> {
    let iter = TimeoutIterator {
        client: c,
        community_id: String::from(community_id),
        cur: None,
        cursor: None,
    };
    Ok(iter)
}

/// Adds a specified user to the timeout list of a specified community
///
/// #### Authentication: `communities_moderate`
///
pub fn timeout(
    c: &TwitchClient,
    community_id: &str,
    user_id: &str,
    duration: i32,
    reason: Option<String>,
) -> TwitchResult<Value> {
    let mut data: HashMap<String, String> = HashMap::new();
    data.insert("duration".to_owned(), duration.to_string());
    if let Some(reason) = reason {
        data.insert("reason".to_owned(), reason);
    }
    let r = r#try!(c.put::<HashMap<String, String>, Value>(
        &format!("/communities/{}/timeouts/{}", community_id, user_id),
        &data
    ));
    Ok(r)
}

/// Deletes a specified user from the timeout list of a specified community
///
/// #### Authentication: `communities_moderate`
///
pub fn delete_timeout(c: &TwitchClient, community_id: &str, user_id: &str) -> TwitchResult<Value> {
    let r = r#try!(c.delete::<Value>(&format!(
        "/communities/{}/timeouts/{}",
        community_id, user_id
    )));
    Ok(r)
}

/// Gets the top communities by viewer count
///
/// #### Authentication: `None`
///
pub fn top<'c>(c: &'c TwitchClient) -> TwitchResult<TopCommunities<'c>> {
    let iter = TopCommunities {
        client: c,
        cur: None,
        cursor: None,
    };
    Ok(iter)
}

///////////////////////////////////////
// Community
///////////////////////////////////////
#[derive(Deserialize, Debug)]
pub struct Community {
    #[serde(rename = "_id")]
    pub id: String,
    pub avatar_image_url: String,
    pub cover_image_url: String,
    pub description: String,
    pub description_html: String,
    pub language: String,
    pub name: String,
    pub owner_id: String,
    pub rules: String,
    pub rules_html: String,
    pub summary: String,
}

pub struct UpdateSettings<'a> {
    pub summary: Option<&'a str>,
    pub description: Option<&'a str>,
    pub rules: Option<&'a str>,
    pub email: Option<&'a str>,
}

#[derive(Deserialize, Debug)]
pub struct Moderators {
    pub moderators: Vec<User>,
}

///////////////////////////////////////
// TopCommunities
///////////////////////////////////////
#[derive(Debug)]
pub struct TopCommunities<'c> {
    client: &'c TwitchClient,
    cur: Option<SerdeTopCommunities>,
    cursor: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct TopCommunity {
    #[serde(rename = "_id")]
    pub id: String,
    pub avatar_image_url: String,
    pub channels: i32,
    pub name: String,
    pub viewers: i32,
}

#[derive(Deserialize, Debug)]
struct SerdeTopCommunities {
    pub communities: Vec<TopCommunity>,
    pub _cursor: Option<String>,
}

impl<'c> Iterator for TopCommunities<'c> {
    type Item = TopCommunity;

    fn next(&mut self) -> Option<TopCommunity> {
        let url = format!("/communities/top?");
        next_result_cursor!(self, &url, SerdeTopCommunities, communities)
    }
}

///////////////////////////////////////
// Community Bans
///////////////////////////////////////
pub struct CommunityBanIterator<'c> {
    client: &'c TwitchClient,
    community_id: String,
    cur: Option<SerdeCommunityBan>,
    cursor: Option<String>,
}

#[derive(Deserialize, Debug)]
struct SerdeCommunityBan {
    pub banned_users: Vec<CommunityBan>,
    pub _cursor: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CommunityBan {
    pub user_id: String,
    pub display_name: String,
    pub name: String,
    pub bio: Option<String>,
    pub avatar_image_url: Option<String>,
    pub start_timestamp: i64,
}

impl<'c> Iterator for CommunityBanIterator<'c> {
    type Item = CommunityBan;

    fn next(&mut self) -> Option<CommunityBan> {
        let url = format!("/communities/{}/bans?", &self.community_id);
        next_result_cursor!(self, &url, SerdeCommunityBan, banned_users)
    }
}

///////////////////////////////////////
// Community Timeouts
///////////////////////////////////////
pub struct TimeoutIterator<'c> {
    client: &'c TwitchClient,
    community_id: String,
    cur: Option<SerdeTimeout>,
    cursor: Option<String>,
}

#[derive(Deserialize, Debug)]
struct SerdeTimeout {
    pub timed_out_users: Vec<TimeoutUser>,
    pub _cursor: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct TimeoutUser {
    pub user_id: String,
    pub display_name: String,
    pub name: String,
    pub bio: Option<String>,
    pub avatar_image_url: Option<String>,
    pub start_timestamp: i64,
    pub end_timestamp: i64,
}

impl<'c> Iterator for TimeoutIterator<'c> {
    type Item = TimeoutUser;

    fn next(&mut self) -> Option<TimeoutUser> {
        let url = format!("/communities/{}/timeouts?", &self.community_id);
        next_result_cursor!(self, &url, SerdeTimeout, timed_out_users)
    }
}
