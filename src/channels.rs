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

use self::chrono::prelude::*;

use super::communities::Community;
use super::response::TwitchResult;
use super::users::User;
use super::videos::Video;
use super::TwitchClient;

use serde_json::Value;
use std;
use std::collections::HashMap;
use std::io::Write;

/// Gets a channel object based on a specified OAuth token
///
/// #### Authentication: `channel_read`
///
pub fn get(c: &TwitchClient) -> TwitchResult<Channel> {
    let r = r#try!(c.get::<Channel>("/channel"));
    Ok(r)
}

/// Gets a specified channel object
///
/// #### Authentication: `None`
///
pub fn get_by_id(c: &TwitchClient, chan_id: &str) -> TwitchResult<Channel> {
    let r = r#try!(c.get::<Channel>(&format!("/channels/{}", chan_id)));
    Ok(r)
}

/// Gets a list of users who are editors for a specified channel
///
/// #### Authentication: `channel_read`
///
pub fn editors(c: &TwitchClient, chan_id: &str) -> TwitchResult<ChannelEditors> {
    let r = r#try!(c.get::<ChannelEditors>(&format!("/channels/{}/editors", chan_id)));
    Ok(r)
}

/// Gets a list of users who follow a specified channel,
/// sorted by the date when they started following the channel
/// (newest first, unless specified otherwise)
///
/// #### Authentication: `None`
///
pub fn followers(c: &TwitchClient, chan_id: &str) -> TwitchResult<ChannelFollowers> {
    let mut followers = ChannelFollowers {
        follows: Vec::new(),
    };
    let mut r =
        r#try!(c.get::<SerdeChannelFollowers>(&format!("/channels/{}/follows?limit=100", chan_id)));
    followers.follows.append(&mut r.follows);
    while let Some(cursor) = r.cursor {
        r = r#try!(c.get::<SerdeChannelFollowers>(&format!(
            "/channels/{}/follows?cursor={}&limit=100",
            chan_id, cursor
        )));
        followers.follows.append(&mut r.follows);
    }
    Ok(followers)
}

/// Gets a list of teams to which a specified channel belongs
///
/// #### Authentication: `None`
///
pub fn teams(c: &TwitchClient, chan_id: &str) -> TwitchResult<ChannelTeams> {
    let r = r#try!(c.get::<ChannelTeams>(&format!("/channels/{}/teams", chan_id)));
    Ok(r)
}

/// Gets a list of users subscribed to a specified channel,
/// sorted by the date when they subscribed
///
/// #### Authentication: `channel_subscriptions`
///
pub fn subscribers(c: &TwitchClient, chan_id: &str) -> TwitchResult<ChannelSubscribers> {
    let mut subs = Vec::new();
    let mut r = r#try!(
        c.get::<ChannelSubscribers>(&format!("/channels/{}/subscriptions?limit=100", chan_id))
    );
    let mut cnt = r._subscriptions.len();
    subs.append(&mut r._subscriptions);
    while cnt > 0 {
        r = r#try!(c.get::<ChannelSubscribers>(&format!(
            "/channels/{}/subscriptions?offset={}&limit=100",
            chan_id,
            subs.len()
        )));
        cnt = r._subscriptions.len();
        if cnt > 0 {
            subs.append(&mut r._subscriptions);
        }
    }
    r.subscriptions = subs;
    Ok(r)
}

/// Checks if a specified channel has a specified user subscribed to it.
/// Intended for use by channel owners
///
/// Returns a subscription object which includes the user
/// if that user is subscribed. Requires authentication
/// for the channel.
///
/// #### Authentication: `channel_check_subscription`
///
pub fn subscription(
    c: &TwitchClient,
    chan_id: &str,
    user_id: &str,
) -> TwitchResult<ChannelSubscription> {
    let r = r#try!(
        c.get::<ChannelSubscription>(&format!("/channels/{}/subscriptions/{}", chan_id, user_id))
    );
    Ok(r)
}

/// Gets a list of videos from a specified channel
///
/// #### Authentication: `None`
///
pub fn videos<'c>(c: &'c TwitchClient, chan_id: &str) -> TwitchResult<VideosIterator<'c>> {
    let iter = VideosIterator {
        client: c,
        chan_id: String::from(chan_id),
        cur: None,
        offset: 0,
    };
    Ok(iter)
}

/// Gets the community for a specified channel
///
/// #### Authentication: `channel_editor`
///
pub fn community(c: &TwitchClient, chan_id: &str) -> TwitchResult<Community> {
    let r = r#try!(c.get::<Community>(&format!("/channels/{}/community", chan_id)));
    Ok(r)
}

/// Sets a specified channel to be in a specified community
///
/// #### Authentication: `channel_editor`
///
pub fn set_community(c: &TwitchClient, chan_id: &str, community_id: &str) -> TwitchResult<Channel> {
    let r = r#try!(c.put::<Value, Channel>(
        &format!("/channels/{}/community/{}", chan_id, community_id),
        &Value::Null
    ));
    Ok(r)
}

/// Updates specified properties of a specified channel
///
/// #### Authentication:
/// * To update `delay` or `channel_feed_enabled` parameter: a `channel_editor` token from the channel owner
/// * To update other parameters: `channel_editor`
///
pub fn update<'a>(
    c: &TwitchClient,
    chan_id: &str,
    data: &'a UpdateSettings,
) -> TwitchResult<Channel> {
    let mut channel: HashMap<String, &str> = HashMap::new();
    if let Some(status) = data.status {
        channel.insert("status".to_owned(), status);
    }
    if let Some(game) = data.game {
        channel.insert("game".to_owned(), game);
    }
    if let Some(delay) = data.delay {
        channel.insert("delay".to_owned(), delay);
    }
    if let Some(channel_feed_enabled) = data.channel_feed_enabled {
        channel.insert("channel_feed_enabled".to_owned(), channel_feed_enabled);
    }
    let mut settings: HashMap<String, HashMap<String, &str>> = HashMap::new();
    settings.insert("channel".to_owned(), channel);
    let r = r#try!(c.put::<HashMap<String, HashMap<String, &str>>, Channel>(
        &format!("/channels/{}", chan_id),
        &settings
    ));
    Ok(r)
}

/// Starts a commercial (advertisement) on a specified channel
///
/// This is valid only for channels that are Twitch partners.
/// You cannot start a commercial more often than once every 8 minutes.
///
/// The length of the commercial (in seconds) is specified in
/// the request body, with a required `duration` parameter.
/// Valid values are 30, 60, 90, 120, 150, and 180.
///
/// There is an error response (422 Unprocessable Entity)
/// if an invalid length is specified, an attempt is made
/// to start a commercial less than 8 minutes after the
/// previous commercial, or the specified channel is not a
/// Twitch partner.
///
/// #### Authentication: `channel_commercial`
///
pub fn commercial(
    c: &TwitchClient,
    chan_id: &str,
    duration: i32,
) -> TwitchResult<CommercialResponse> {
    let r = r#try!(c.post::<CommercialDuration, CommercialResponse>(
        &format!("/channels/{}/commercial", chan_id),
        &CommercialDuration { duration: duration }
    ));
    Ok(r)
}

/// Deletes the stream key for a specified channel.
/// Once it is deleted, the stream key is automatically reset
///
/// A stream key (also known as authorization key) uniquely
/// identifies a stream. Each broadcast uses an RTMP URL
/// that includes the stream key. Stream keys are assigned
/// by Twitch.
///
/// #### Authentication: `channel_stream`
///
pub fn reset_stream_key(c: &TwitchClient, chan_id: &str) -> TwitchResult<Channel> {
    let r = r#try!(c.delete::<Channel>(&format!("/channels/{}/stream_key", chan_id)));
    Ok(r)
}

///////////////////////////////////////
// GetChannel
///////////////////////////////////////
#[derive(Deserialize, Debug)]
pub struct Channel {
    #[serde(rename = "_id")]
    pub id: i64,
    pub broadcaster_language: String,
    pub created_at: DateTime<UTC>,
    pub display_name: String,
    pub email: Option<String>,
    pub followers: i32,
    pub game: String,
    pub language: String,
    pub logo: String,
    pub mature: Option<bool>,
    pub name: String,
    pub partner: bool,
    pub profile_banner: Option<String>,
    pub profile_banner_background_color: Option<String>,
    pub status: String,
    pub stream_key: Option<String>,
    pub updated_at: DateTime<UTC>,
    pub url: String,
    pub video_banner: Option<String>,
    pub views: i32,
}

///////////////////////////////////////
// Channel definitions
///////////////////////////////////////
#[derive(Deserialize, Debug)]
pub struct ChannelEditors {
    pub users: Vec<ChannelEditor>,
}

#[derive(Deserialize, Debug)]
pub struct ChannelEditor {
    pub _id: i64,
    pub bio: Option<String>,
    pub created_at: DateTime<UTC>,
    pub display_name: String,
    pub logo: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub updated_at: DateTime<UTC>,
}

#[derive(Debug)]
pub struct ChannelFollowers {
    pub follows: Vec<ChannelFollow>,
}

#[derive(Deserialize, Debug)]
pub struct ChannelFollow {
    pub created_at: DateTime<UTC>,
    pub notifications: bool,
    pub user: User,
}

#[derive(Deserialize, Debug)]
struct SerdeChannelFollowers {
    follows: Vec<ChannelFollow>,
    cursor: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ChannelTeams {
    pub teams: Vec<ChannelTeam>,
}

#[derive(Deserialize, Debug)]
pub struct ChannelTeam {
    pub _id: i64,
    pub background: Option<String>,
    pub banner: String,
    pub created_at: DateTime<UTC>,
    pub display_name: String,
    pub info: String,
    pub logo: String,
    pub name: String,
    pub updated_at: DateTime<UTC>,
}

#[derive(Deserialize, Debug)]
pub struct ChannelSubscribers {
    #[serde(skip_deserializing, default = "Vec::new")]
    pub subscriptions: Vec<ChannelSubscription>,

    #[serde(rename(deserialize = "subscriptions"))]
    _subscriptions: Vec<ChannelSubscription>,
    _total: i32,
}

#[derive(Deserialize, Debug)]
pub struct ChannelSubscription {
    #[serde(rename = "_id")]
    pub id: String,
    pub created_at: DateTime<UTC>,
    pub user: User,
}

pub struct UpdateSettings<'a> {
    pub status: Option<&'a str>,
    pub game: Option<&'a str>,
    pub delay: Option<&'a str>,
    pub channel_feed_enabled: Option<&'a str>,
}

#[derive(Serialize, Debug)]
pub struct CommercialDuration {
    pub duration: i32,
}

#[derive(Deserialize, Debug)]
pub struct CommercialResponse {
    pub duration: i32,
    pub message: String,
    pub retryafter: i32,
}

///////////////////////////////////////
// Videos
///////////////////////////////////////
#[derive(Debug)]
pub struct VideosIterator<'c> {
    client: &'c TwitchClient,
    chan_id: String,
    cur: Option<SerdeChannelVideos>,
    offset: i32,
}

#[derive(Deserialize, Debug)]
struct SerdeChannelVideos {
    pub videos: Vec<Video>,
}

impl<'c> Iterator for VideosIterator<'c> {
    type Item = Video;

    fn next(&mut self) -> Option<Video> {
        let url = &format!(
            "/channels/{}/videos?limit=100&offset={}",
            &self.chan_id, self.offset
        );
        next_result!(self, &url, SerdeChannelVideos, videos)
    }
}

///////////////////////////////////////
// TESTS
///////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::super::new;
    use super::super::response::ApiError;
    use super::super::tests::{CHANID, CLIENTID, TOKEN};

    #[test]
    fn get() {
        let mut c = new(String::from(CLIENTID));
        c.set_oauth_token(TOKEN);

        match super::get(&c) {
            Ok(r) => assert_eq!(&r.id.to_string(), CHANID),
            Err(r) => {
                println!("{:?}", r);
                assert!(false);
            }
        }
    }

    #[test]
    fn get_by_id() {
        let mut c = new(String::from(CLIENTID));

        match super::get_by_id(&c, CHANID) {
            Ok(r) => assert_eq!(&r.id.to_string(), CHANID),
            Err(r) => {
                println!("{:?}", r);
                assert!(false);
            }
        }
    }

    #[test]
    fn editors() {
        let mut c = new(String::from(CLIENTID));
        c.set_oauth_token(TOKEN);

        match super::editors(&c, CHANID) {
            Ok(r) => assert_eq!(&r.users[0].name, "rust_api_test_editor"),
            Err(r) => {
                println!("{:?}", r);
                assert!(false);
            }
        }
    }

    #[test]
    fn followers() {
        let c = new(String::from(CLIENTID));
        match super::followers(&c, CHANID) {
            Ok(r) => assert_eq!(&r.follows[0].user.name, "rust_api_test_editor"),
            Err(r) => {
                println!("{:?}", r);
                assert!(false);
            }
        }
    }

    #[test]
    fn teams() {
        let c = new(String::from(CLIENTID));
        match super::teams(&c, CHANID) {
            Ok(r) => assert_eq!(r.teams.len(), 0),
            Err(r) => {
                println!("{:?}", r);
                assert!(false);
            }
        }
    }

    #[test]
    fn subscribers() {
        let mut c = new(String::from(CLIENTID));
        c.set_oauth_token(TOKEN);

        match super::subscribers(&c, CHANID) {
            Ok(_r) => (),
            Err(r) => match r {
                ApiError::TwitchError(e) => assert_eq!(e.status, 422),
                _ => {
                    println!("{:?}", r);
                    assert!(false)
                }
            },
        }
    }

    #[test]
    fn subscription() {
        let c = new(String::from(CLIENTID));
        match super::subscription(&c, CHANID, CHANID) {
            Ok(_r) => (),
            Err(r) => match r {
                ApiError::TwitchError(e) => assert_eq!(e.status, 401),
                _ => {
                    println!("{:?}", r);
                    assert!(false);
                }
            },
        }
    }

    #[test]
    fn videos() {
        let c = new(String::from(CLIENTID));
        match super::videos(&c, CHANID) {
            Ok(mut r) => assert_eq!(r.next().unwrap().id, "v131643674"),
            Err(r) => {
                println!("{:?}", r);
                assert!(false);
            }
        }
    }
}
