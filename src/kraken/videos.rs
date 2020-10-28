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

use chrono::prelude::*;
use serde::Deserialize;

use std::{
    self,
    collections::HashMap,
    fmt,
    io::Write,
};

use super::super::{
    response::TwitchResult,
    TwitchClient,
};

/// Gets a specified video object
///
/// #### Authentication: `None`
pub fn get(
    c: &TwitchClient,
    video_id: &str,
) -> TwitchResult<Video>
{
    let r = c.get::<Video>(&format!("/videos/{}", video_id))?;
    Ok(r)
}

/// Gets the top videos based on viewcount, optionally
/// filtered by game or time period
///
/// #### Authentication: `None`
pub fn top<'c>(
    c: &'c TwitchClient,
    game: Option<&str>,
    period: Option<TopVideoPeriod>,
) -> TwitchResult<TopVideoIterator<'c>>
{
    let game = match game {
        Some(g) => Some(String::from(g)),
        None => None,
    };
    let iter = TopVideoIterator {
        client: c,
        game,
        period,
        cur: None,
        offset: 0,
    };
    Ok(iter)
}

/// Gets the videos from channels followed by a user,
/// based on a specified OAuth token
///
/// #### Authentication: `user_read`
pub fn followed(c: &TwitchClient) -> TwitchResult<FollowedVideoIterator> {
    let iter = FollowedVideoIterator {
        client: c,
        cur: None,
        offset: 0,
    };
    Ok(iter)
}

///////////////////////////////////////
// GetVideo
///////////////////////////////////////
#[derive(Deserialize, Debug)]
pub struct Video {
    #[serde(rename = "_id")]
    pub id: String,
    pub broadcast_id: i64,
    pub broadcast_type: String,
    pub channel: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub description: String,
    pub description_html: String,
    pub fps: HashMap<String, f64>,
    pub game: String,
    pub language: String,
    pub length: i32,
    pub muted_segments: Option<Vec<HashMap<String, i32>>>,
    pub preview: HashMap<String, String>,
    pub published_at: DateTime<Utc>,
    pub resolutions: HashMap<String, String>,
    pub status: String,
    pub tag_list: String,
    pub thumbnails: HashMap<String, Vec<HashMap<String, String>>>,
    pub title: String,
    pub url: String,
    pub viewable: String,
    pub viewable_at: Option<DateTime<Utc>>,
    pub views: i32,
}

///////////////////////////////////////
// GetTopVideos
///////////////////////////////////////
#[derive(Debug)]
pub struct TopVideoIterator<'c> {
    client: &'c TwitchClient,
    game: Option<String>,
    period: Option<TopVideoPeriod>,
    cur: Option<SerdeTopVideos>,
    offset: i32,
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum TopVideoPeriod {
    week,
    month,
    all,
}

impl fmt::Display for TopVideoPeriod {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result
    {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Deserialize, Debug)]
struct SerdeTopVideos {
    vods: Vec<Video>,
}

impl<'c> Iterator for TopVideoIterator<'c> {
    type Item = Video;

    fn next(&mut self) -> Option<Video> {
        let mut url = format!("/videos/top?offset={}", self.offset);
        if let Some(ref game) = self.game {
            url.push_str("&game=");
            url.push_str(&game);
        }
        if let Some(ref period) = self.period {
            url.push_str("&period=");
            url.push_str(&period.to_string());
        }
        next_result!(self, &url, SerdeTopVideos, vods)
    }
}

///////////////////////////////////////
// GetFollowedVideos
///////////////////////////////////////
#[derive(Debug)]
pub struct FollowedVideoIterator<'c> {
    client: &'c TwitchClient,
    cur: Option<SerdeFollowedVideos>,
    offset: i32,
}

#[derive(Deserialize, Debug)]
struct SerdeFollowedVideos {
    videos: Vec<Video>,
}

impl<'c> Iterator for FollowedVideoIterator<'c> {
    type Item = Video;

    fn next(&mut self) -> Option<Video> {
        let url = "/videos/followed";
        next_result!(self, &url, SerdeFollowedVideos, videos)
    }
}

///////////////////////////////////////
// TESTS
///////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::{
        new,
        tests::{
            CLIENTID,
            TOKEN,
        },
    };

    #[test]
    fn videos() {
        let mut c = new(String::from(CLIENTID));
        c.set_oauth_token(TOKEN);

        if let Some(video) = match super::followed(&c) {
            Ok(mut r) => r.next(),
            Err(r) => {
                println!("{:?}", r);
                assert!(false);
                None
            }
        } {
            match super::get(&c, &video.id) {
                Ok(r) => assert_eq!(r.id, video.id),
                Err(r) => {
                    println!("{:?}", r);
                    assert!(false);
                }
            }
        }
        match super::top(&c, None, None) {
            Ok(mut r) => assert!(r.next().is_some()),
            Err(r) => {
                println!("{:?}", r);
                assert!(false);
            }
        }
        match super::top(&c, Some("IRL"), None) {
            Ok(mut r) => assert!(r.next().is_some()),
            Err(r) => {
                println!("{:?}", r);
                assert!(false);
            }
        }
        match super::top(&c, None, Some(super::TopVideoPeriod::month)) {
            Ok(mut r) => assert!(r.next().is_some()),
            Err(r) => {
                println!("{:?}", r);
                assert!(false);
            }
        }
        match super::top(&c, Some("IRL"), Some(super::TopVideoPeriod::month)) {
            Ok(mut r) => assert!(r.next().is_some()),
            Err(r) => {
                println!("{:?}", r);
                assert!(false);
            }
        }
    }
}
