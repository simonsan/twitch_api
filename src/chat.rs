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

use std::collections::HashMap;

use super::response::TwitchResult;
use super::TwitchClient;

/// Gets a list of badges that can be used in chat for a specified channel
///
/// #### Authentication: `None`
///
pub fn get_badges(c: &TwitchClient, chan_id: &str) -> TwitchResult<BadgeSet> {
    let r = r#try!(c.get::<BadgeSet>(&format!("/chat/{}/badges", chan_id)));
    Ok(r)
}

/// Gets all chat emoticons (not including their images) in one or more specified sets
///
/// #### Authentication: `None`
///
/// # Remarks
/// Caution: When not specifying the emotesets parameter,
/// this endpoint returns a large amount of data.
///
pub fn get_emote_sets(c: &TwitchClient, sets: &[&str]) -> TwitchResult<EmotesBySet> {
    let r = r#try!(c.get::<EmotesBySet>(&format!(
        "/chat/emoticon_images?emotesets={}",
        sets.join(",")
    )));
    Ok(r)
}

/// Gets all chat emoticons (including their images)
///
/// #### Authentication: `None`
///
/// # Remarks
/// Caution: This endpoint returns a large amount of data.
///
pub fn get_emotes(c: &TwitchClient) -> TwitchResult<ChatEmotes> {
    let r = r#try!(c.get::<ChatEmotes>("/chat/emoticons"));
    Ok(r)
}

///////////////////////////////////////
// GetChatBadgesByChannel
///////////////////////////////////////
pub type BadgeSet = HashMap<String, Option<Badge>>;

#[derive(Deserialize, Debug)]
pub struct Badge {
    pub alpha: String,
    pub image: String,
    pub svg: String,
}

///////////////////////////////////////
// GetChatEmotesBySets
///////////////////////////////////////
#[derive(Deserialize, Debug)]
pub struct EmotesBySet {
    pub emoticon_sets: HashMap<String, Vec<EmoteSet>>,
}

#[derive(Deserialize, Debug)]
pub struct EmoteSet {
    pub id: i64,
    pub code: String,
}

///////////////////////////////////////
// GetAllChatEmotes
///////////////////////////////////////
#[derive(Deserialize, Debug)]
pub struct ChatEmotes {
    pub emoticons: Vec<ChatEmote>,
}

#[derive(Deserialize, Debug)]
pub struct ChatEmote {
    pub regex: String,
    pub images: Vec<ChatEmoteImage>,
}

#[derive(Deserialize, Debug)]
pub struct ChatEmoteImage {
    pub width: i32,
    pub height: i32,
    pub url: String,
    pub emoticon_set: Option<i64>,
}

///////////////////////////////////////
// TESTS
///////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::super::new;
    use super::super::tests::CLIENTID;

    #[test]
    fn get_badges() {
        let c = new(String::from(CLIENTID));

        match super::get_badges(&c, "12826") {
            Ok(r) => assert!(r.contains_key("global_mod")),
            Err(r) => {
                println!("{:?}", r);
                assert!(false);
            }
        }
    }

    #[test]
    fn get_emote_sets() {
        let c = new(String::from(CLIENTID));

        match super::get_emote_sets(&c, &["19151"]) {
            Ok(r) => assert!(r.emoticon_sets.contains_key("19151")),
            Err(r) => {
                println!("{:?}", r);
                assert!(false);
            }
        }
    }

    #[test]
    fn get_emotes() {
        let c = new(String::from(CLIENTID));

        match super::get_emotes(&c) {
            Ok(r) => assert!(r.emoticons.len() > 0),
            Err(r) => {
                println!("{:?}", r);
                assert!(false);
            }
        }
    }
}
