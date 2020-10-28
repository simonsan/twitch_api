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
    self,
    collections::HashMap,
    io::Write,
};

use chrono::prelude::*;
use serde::Deserialize;

use super::channels::Channel;

use crate::{
    response::TwitchResult,
    TwitchClient,
};

/// Gets stream information (the stream object) for a specified user
///
/// #### Authentication: `None`
pub fn get(
    c: &TwitchClient,
    chan_id: &str,
) -> TwitchResult<StreamByUser>
{
    let r = c.get::<StreamByUser>(&format!("/streams/{}", chan_id))?;
    Ok(r)
}

/// Gets a list of live streams
///
/// #### Authentication: `None`
pub fn live<'c>(
    c: &'c TwitchClient,
    channel_ids: Option<&[&str]>,
    game: Option<String>,
    language: Option<String>,
) -> TwitchResult<LiveStreamsIterator<'c>>
{
    let channels = match channel_ids {
        Some(ch) => Some(ch.join(",")),
        None => None,
    };

    let iter = LiveStreamsIterator {
        client: c,
        cur: None,
        channel: channels,
        game,
        language,
        offset: 0,
    };
    Ok(iter)
}

/// Gets a summary of live streams
///
/// #### Authentication: `None`
pub fn summary(
    c: &TwitchClient,
    game: Option<&str>,
) -> TwitchResult<Summary>
{
    let mut url = String::from("/streams/summary");
    if let Some(game) = game {
        url.push_str("?game=");
        url.push_str(game);
    }
    let r = c.get::<Summary>(&url)?;
    Ok(r)
}

/// Gets a list of all featured live streams
///
/// #### Authentication: `None`
pub fn featured(c: &TwitchClient) -> TwitchResult<FeaturedIterator> {
    let iter = FeaturedIterator {
        client: c,
        cur: None,
        offset: 0,
    };
    Ok(iter)
}

/// Gets a list of online streams a user is following,
/// based on a specified OAuth token.
///
/// #### Authentication: `user_read`
pub fn followed(c: &TwitchClient) -> TwitchResult<FollowedStreams> {
    let mut lst = Vec::new();
    let mut r = c.get::<FollowedStreams>("/streams/followed?limit=100")?;
    lst.append(&mut r._streams);
    while let Some(cursor) = r._cursor {
        r = c.get::<FollowedStreams>(&format!(
            "/streams/followsed?cursor={}&limit=100",
            cursor
        ))?;
        lst.append(&mut r._streams);
    }
    r.streams = lst;
    Ok(r)
}

///////////////////////////////////////
// GetStreamByUser
///////////////////////////////////////
#[derive(Deserialize, Debug)]
pub struct StreamByUser {
    pub stream: Option<Stream>,
}

#[derive(Deserialize, Debug)]
pub struct Stream {
    #[serde(rename = "_id")]
    pub id: i64,
    pub game: String,
    pub viewers: i32,
    pub video_height: i32,
    pub average_fps: i32,
    pub delay: i32,
    pub created_at: DateTime<Utc>,
    pub is_playlist: bool,
    pub preview: HashMap<String, String>,
    pub channel: Channel,
}

///////////////////////////////////////
// GetLiveStreams
///////////////////////////////////////
pub struct LiveStreamsIterator<'c> {
    client: &'c TwitchClient,
    cur: Option<SerdeLiveStreams>,
    channel: Option<String>,
    game: Option<String>,
    language: Option<String>,
    offset: i32,
}

#[derive(Deserialize, Debug)]
struct SerdeLiveStreams {
    streams: Vec<Stream>,
}

impl<'c> Iterator for LiveStreamsIterator<'c> {
    type Item = Stream;

    fn next(&mut self) -> Option<Stream> {
        let mut url = format!("/streams?limit=100&offset={}", self.offset);
        if let Some(ref ch) = self.channel {
            url.push_str("&channel=");
            url.push_str(&ch);
        }
        if let Some(ref game) = self.game {
            url.push_str("&game=");
            url.push_str(&game);
        }
        if let Some(ref lang) = self.language {
            url.push_str("&language=");
            url.push_str(&lang);
        }
        next_result!(self, &url, SerdeLiveStreams, streams)
    }
}

///////////////////////////////////////
// GetStreamsSummary
///////////////////////////////////////
#[derive(Deserialize, Debug)]
pub struct Summary {
    pub channels: Option<i32>,
    pub viewers: Option<i32>,
    pub error: Option<String>,
    pub status: Option<i32>,
    pub message: Option<String>,
}

///////////////////////////////////////
// GetFeaturedStreams
///////////////////////////////////////
pub struct FeaturedIterator<'c> {
    client: &'c TwitchClient,
    cur: Option<SerdeFeaturedStreams>,
    offset: i32,
}

#[derive(Deserialize, Debug)]
pub struct Featured {
    pub image: String,
    pub priority: i32,
    pub scheduled: bool,
    pub sponsored: bool,
    pub stream: Stream,
    pub text: String,
    pub title: String,
}

#[derive(Deserialize, Debug)]
struct SerdeFeaturedStreams {
    featured: Vec<Featured>,
}

impl<'c> Iterator for FeaturedIterator<'c> {
    type Item = Featured;

    fn next(&mut self) -> Option<Featured> {
        if self.cur.is_none() {
            if let Ok(r) = self.client.get::<SerdeFeaturedStreams>(&format!(
                "/streams/featured?limit=100&offset={}",
                self.offset
            )) {
                self.offset += r.featured.len() as i32;
                self.cur = Some(r);
            }
            else {
                return None;
            }
        }

        let mut x = None;
        let mut cnt = 0;
        if let Some(ref mut cur) = self.cur {
            x = cur.featured.pop();
            cnt = cur.featured.len();
        }
        if cnt == 0 {
            self.cur = None;
        }
        x
    }
}

///////////////////////////////////////
// GetFollowedStreams
///////////////////////////////////////
#[derive(Deserialize, Debug)]
pub struct FollowedStreams {
    #[serde(skip_deserializing, default = "Vec::new")]
    pub streams: Vec<Stream>,

    #[serde(rename = "streams")]
    _streams: Vec<Stream>,
    _cursor: Option<String>,
}

///////////////////////////////////////
// TESTS
///////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::{
        new,
        tests::{
            CHANID,
            CLIENTID,
            TOKEN,
        },
    };

    #[test]
    fn get() {
        let c = new(String::from(CLIENTID));

        match super::get(&c, CHANID) {
            Ok(_r) => (),
            Err(r) => {
                println!("{:?}", r);
                assert!(false);
            }
        }
    }

    #[test]
    fn live() {
        let c = new(String::from(CLIENTID));

        match super::live(&c, None, None, None) {
            Ok(mut r) => assert_ne!(r.next().unwrap().id, 0),
            Err(r) => {
                println!("{:?}", r);
                assert!(false);
            }
        }

        if let Some(chan) =
            match super::live(&c, None, Some("IRL".to_owned()), None) {
                Ok(mut r) => Some(r.next().unwrap().channel),
                Err(r) => {
                    println!("{:?}", r);
                    assert!(false);
                    None
                }
            }
        {
            assert_ne!(chan.id, 0);

            match super::live(&c, Some(&[&chan.id.to_string()]), None, None) {
                Ok(mut r) => match r.next() {
                    Some(st) => assert_ne!(st.id, 0),
                    None => {
                        println!("{:?}", chan);
                        assert!(false);
                    }
                },
                Err(r) => {
                    println!("{:?}", r);
                    assert!(false);
                }
            }
        }

        match super::live(&c, None, None, Some("en".to_owned())) {
            Ok(mut r) => assert_ne!(r.next().unwrap().id, 0),
            Err(r) => {
                println!("{:?}", r);
                assert!(false);
            }
        }
    }

    #[test]
    fn summary() {
        let c = new(String::from(CLIENTID));

        if let Some(all_cnt) = match super::summary(&c, None) {
            Ok(r) => r.viewers,
            Err(r) => {
                println!("{:?}", r);
                assert!(false);
                None
            }
        } {
            match super::summary(&c, Some("IRL")) {
                Ok(r) => assert!(all_cnt > r.viewers.expect("2")),
                Err(r) => {
                    println!("{:?}", r);
                    assert!(false);
                }
            }
        }
        else {
            println!("None viewers");
            assert!(false);
        }
    }

    #[test]
    fn featured() {
        let c = new(String::from(CLIENTID));

        match super::featured(&c) {
            Ok(mut r) => match r.next() {
                Some(st) => assert_ne!(st.stream.id, 0),
                None => assert!(false),
            },
            Err(r) => {
                println!("{:?}", r);
                assert!(false);
            }
        }
    }

    #[test]
    fn followed() {
        let mut c = new(String::from(CLIENTID));
        c.set_oauth_token(TOKEN);

        match super::followed(&c) {
            Ok(_r) => (),
            Err(r) => {
                println!("{:?}", r);
                assert!(false);
            }
        }
    }
}
