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

extern crate chrono;
extern crate serde_json;

use self::chrono::prelude::*;

use super::channels::Channel;
use super::chat::EmotesBySet;
use super::response::{ApiError, TwitchResult};
use super::TwitchClient;

use serde_json::Value;
use std;
use std::collections::HashMap;
use std::io::Write;

/// Gets a user object based on the OAuth token provided
///
/// #### Authentication: `user_read`
///
pub fn get(c: &TwitchClient) -> TwitchResult<User> {
    let r = r#try!(c.get::<User>("/user"));
    Ok(r)
}

/// Gets a specified user object
///
/// #### Authentication: `None`
///
pub fn get_by_id(c: &TwitchClient, user_id: &str) -> TwitchResult<User> {
    let r = r#try!(c.get::<User>(&format!("/users/{}", user_id)));
    Ok(r)
}

/// Gets a list of the emojis and emoticons that the specified user can use in chat
///
/// These are both the globally available ones and
/// the channel-specific ones (which can be accessed
/// by any user subscribed to the channel).
///
/// #### Authentication: `user_subscriptions`
///
pub fn emotes(c: &TwitchClient, user_id: &str) -> TwitchResult<EmotesBySet> {
    let r = r#try!(c.get::<EmotesBySet>(&format!("/users/{}/emotes", user_id)));
    Ok(r)
}

/// Checks if a specified user is subscribed to a specified channel
///
/// #### Authentication: `user_subscription`
///
pub fn subscription(
    c: &TwitchClient,
    user_id: &str,
    channel_id: &str,
) -> TwitchResult<UserSubFollow> {
    let r =
        r#try!(c.get::<UserSubFollow>(&format!("/users/{}/subscriptions/{}", user_id, channel_id)));
    Ok(r)
}

/// Gets a list of all channels followed by a specified
/// user, sorted by the date when they started following each channel
///
/// #### Authentication: `None`
///
pub fn following<'c>(c: &'c TwitchClient, user_id: &str) -> TwitchResult<UserFollowIterator<'c>> {
    let iter = UserFollowIterator {
        client: c,
        user_id: String::from(user_id),
        cur: None,
        offset: 0,
    };
    Ok(iter)
}

/// Checks if a specified user follows a specified channel
///
/// If the user is following the channel, a follow object is returned.
///
/// #### Authentication: `None`
///
pub fn is_following(
    c: &TwitchClient,
    user_id: &str,
    channel_id: &str,
) -> TwitchResult<Option<UserSubFollow>> {
    let r = c.get::<UserSubFollow>(&format!(
        "/users/{}/follows/channels/{}",
        user_id, channel_id
    ));
    match r {
        Ok(r) => Ok(Some(r)),
        Err(e) => match e {
            ApiError::TwitchError(te) => {
                if te.status == 404 {
                    return Ok(None);
                } else {
                    return Err(ApiError::from(te));
                }
            }
            _ => Err(e),
        },
    }
}

/// Adds a specified user to the followers of a specified channel
///
/// #### Authentication: `user_follows_edit`
///
pub fn follow(
    c: &TwitchClient,
    user_id: &str,
    chan_id: &str,
    notifications: bool,
) -> TwitchResult<UserSubFollow> {
    let mut data: HashMap<String, bool> = HashMap::new();
    data.insert("notifications".to_owned(), notifications);
    let r = r#try!(c.put::<HashMap<String, bool>, UserSubFollow>(
        &format!("/users/{}/follows/channels/{}", user_id, chan_id),
        &data
    ));
    Ok(r)
}

/// Deletes a specified user from the followers of a specified channel
///
/// #### Authentication: `user_follows_edit`
///
pub fn unfollow(c: &TwitchClient, user_id: &str, chan_id: &str) -> TwitchResult<()> {
    let r = c.delete::<()>(&format!("/users/{}/follows/channels/{}", user_id, chan_id));
    match r {
        Ok(_) => Ok(assert!(false)), // this should never happen
        Err(r) => match r {
            ApiError::EmptyResponse(_) => Ok(()),
            _ => Err(r),
        },
    }
}

/// Gets a user’s block list. List sorted by recency, newest first
///
/// #### Authentication: `user_blocks_read`
///
pub fn blocking<'c>(c: &'c TwitchClient, user_id: &str) -> TwitchResult<UserBlockIterator<'c>> {
    let iter = UserBlockIterator {
        client: c,
        user_id: String::from(user_id),
        cur: None,
        offset: 0,
    };
    Ok(iter)
}

/// Blocks a user; that is, adds a specified target user
/// to the blocks list of a specified source user
///
/// #### Authentication: `user_blocks_edit`
///
pub fn block(c: &TwitchClient, src_user_id: &str, tgt_user_id: &str) -> TwitchResult<UserBlock> {
    let r = r#try!(c.put::<Value, UserBlock>(
        &format!("/users/{}/blocks/{}", src_user_id, tgt_user_id),
        &Value::Null
    ));
    Ok(r)
}

/// Unblocks a user; that is, deletes a specified target
/// user from the blocks list of a specified source user
///
/// There is an error if the target user is not on the
/// source user’s block list (404 Not Found) or the
/// delete failed (422 Unprocessable Entity).
///
/// #### Authentication: `user_blocks_edit`
///
pub fn unblock(c: &TwitchClient, src_user_id: &str, tgt_user_id: &str) -> TwitchResult<()> {
    let r = c.delete::<()>(&format!("/users/{}/blocks/{}", src_user_id, tgt_user_id));
    match r {
        Ok(_) => Ok(assert!(false)), // this should never happen
        Err(r) => match r {
            ApiError::EmptyResponse(_) => Ok(()),
            _ => Err(r),
        },
    }
}

///////////////////////////////////////
// User
///////////////////////////////////////
#[derive(Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: i64,
    pub bio: String,
    pub created_at: DateTime<UTC>,
    pub display_name: String,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub logo: String,
    pub name: String,
    pub notifications: Option<UserNotifications>,
    #[serde(rename = "type")]
    pub _type: String,
    updated_at: DateTime<UTC>,
}

#[derive(Deserialize, Debug)]
pub struct UserSubFollow {
    pub channel: Channel,
    pub created_at: DateTime<UTC>,
    pub notifications: bool,
}

#[derive(Deserialize, Debug)]
pub struct UserNotifications {
    pub email: bool,
    pub push: bool,
}

#[derive(Deserialize, Debug)]
pub struct UserBlock {
    pub user: User,
}

///////////////////////////////////////
// User Iterators
///////////////////////////////////////
pub struct UserFollowIterator<'c> {
    client: &'c TwitchClient,
    user_id: String,
    cur: Option<SerdeUserFollows>,
    offset: i32,
}

#[derive(Deserialize, Debug)]
struct SerdeUserFollows {
    pub follows: Vec<UserSubFollow>,
}

impl<'c> Iterator for UserFollowIterator<'c> {
    type Item = UserSubFollow;

    fn next(&mut self) -> Option<UserSubFollow> {
        let url = &format!(
            "/users/{}/follows/channels?limit=100&offset={}",
            &self.user_id, self.offset
        );
        next_result!(self, &url, SerdeUserFollows, follows)
    }
}

pub struct UserBlockIterator<'c> {
    client: &'c TwitchClient,
    user_id: String,
    cur: Option<SerdeUserBlocks>,
    offset: i32,
}

#[derive(Deserialize, Debug)]
struct SerdeUserBlocks {
    pub blocks: Vec<UserBlock>,
}

impl<'c> Iterator for UserBlockIterator<'c> {
    type Item = UserBlock;

    fn next(&mut self) -> Option<UserBlock> {
        let url = &format!(
            "/users/{}/blocks?limit=100&offset={}",
            &self.user_id, self.offset
        );
        next_result!(self, &url, SerdeUserBlocks, blocks)
    }
}

///////////////////////////////////////
// TESTS
///////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::super::new;
    use super::super::response::ApiError;
    use super::super::tests::{CHANID, CLIENTID, TESTCH, TOKEN};

    #[test]
    fn user() {
        let mut c = new(String::from(CLIENTID));
        c.set_oauth_token(TOKEN);

        if let Some(user) = match super::get(&c) {
            Ok(r) => {
                assert!(r.email.is_some());
                Some(r)
            }
            Err(r) => {
                println!("{:?}", r);
                assert!(false);
                None
            }
        } {
            let user_id = user.id.to_string();

            match super::get_by_id(&c, &user_id) {
                Ok(r) => assert_eq!(r.name, user.name),
                Err(r) => {
                    println!("{:?}", r);
                    assert!(false);
                }
            }
            match super::emotes(&c, &user_id) {
                Ok(r) => assert!(r.emoticon_sets.len() > 0),
                Err(r) => {
                    println!("{:?}", r);
                    assert!(false);
                }
            }
            match super::subscription(&c, &user_id, "1") {
                Ok(_r) => (),
                Err(r) => match r {
                    ApiError::TwitchError(e) => assert_eq!(e.status, 422),
                    _ => {
                        println!("{:?}", r);
                        assert!(false);
                    }
                },
            }
            // follow
            match super::follow(&c, &user_id, &TESTCH.to_string(), false) {
                Ok(r) => assert_eq!(r.channel.id, TESTCH),
                Err(r) => {
                    println!("{:?}", r);
                    assert!(false);
                }
            }
            match super::following(&c, &user_id) {
                Ok(mut r) => assert_eq!(r.next().unwrap().channel.id, TESTCH),
                Err(r) => {
                    println!("{:?}", r);
                    assert!(false);
                }
            }
            match super::is_following(&c, &user_id, &TESTCH.to_string()) {
                Ok(_r) => (),
                Err(r) => {
                    println!("{:?}", r);
                    assert!(false);
                }
            }
            match super::unfollow(&c, &user_id, &TESTCH.to_string()) {
                Ok(_r) => (),
                Err(r) => {
                    println!("{:?}", r);
                    assert!(false);
                }
            }
            match super::is_following(&c, &user_id, &TESTCH.to_string()) {
                Ok(r) => assert!(r.is_none()),
                Err(r) => {
                    println!("{:?}", r);
                    assert!(false);
                }
            }
            // block
            match super::block(&c, &user_id, "1") {
                Ok(r) => assert_eq!(r.user.id, 1),
                Err(r) => {
                    println!("{:?}", r);
                    assert!(false);
                }
            }
            match super::blocking(&c, &user_id) {
                Ok(mut r) => assert_eq!(r.next().unwrap().user.id, 1),
                Err(r) => {
                    println!("{:?}", r);
                    assert!(false);
                }
            }
            match super::unblock(&c, &user_id, "1") {
                Ok(_r) => (),
                Err(r) => {
                    println!("{:?}", r);
                    assert!(false);
                }
            }
            match super::blocking(&c, &user_id) {
                Ok(mut r) => assert!(r.next().is_none()),
                Err(r) => {
                    println!("{:?}", r);
                    assert!(false);
                }
            }
        }
    }
}
