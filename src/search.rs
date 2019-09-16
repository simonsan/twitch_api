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
extern crate urlparse;

use std;
use std::io::Write;

use self::urlparse::quote;

use super::channels::Channel;
use super::games::Game;
use super::response::TwitchResult;
use super::streams::Stream;
use super::TwitchClient;

/// Searches for channels based on a specified query parameter
///
/// A channel is returned if the query parameter is
/// matched entirely or partially, in the channel
/// description or game name.
///
/// #### Authentication: `None`
///
pub fn channels<'c>(c: &'c TwitchClient, query: &str) -> TwitchResult<SearchChannelIterator<'c>> {
    let iter = SearchChannelIterator {
        client: c,
        query: quote(query, b"").ok().unwrap(),
        cur: None,
        offset: 0,
    };
    Ok(iter)
}

/// Searches for games based on a specified query parameter
///
/// A game is returned if the query parameter is
/// matched entirely or partially, in the game name.
///
/// #### Authentication: `None`
///
pub fn games<'c>(
    c: &'c TwitchClient,
    query: &str,
    live_only: bool,
) -> TwitchResult<SearchGameIterator<'c>> {
    let iter = SearchGameIterator {
        client: c,
        query: quote(query, b"").ok().unwrap(),
        live_only: live_only,
        cur: None,
        offset: 0,
    };
    Ok(iter)
}

/// Searches for streams based on a specified query parameter
///
/// A stream is returned if the query parameter is
/// matched entirely or partially, in the channel
/// description or game name.
///
/// #### Authentication: `None`
///
pub fn streams<'c>(
    c: &'c TwitchClient,
    query: &str,
    protocol: Option<Protocol>,
) -> TwitchResult<SearchStreamIterator<'c>> {
    let iter = SearchStreamIterator {
        client: c,
        query: quote(query, b"").ok().unwrap(),
        protocol: protocol,
        cur: None,
        offset: 0,
    };
    Ok(iter)
}

///////////////////////////////////////
// SearchChannels
///////////////////////////////////////
#[derive(Debug)]
pub struct SearchChannelIterator<'c> {
    client: &'c TwitchClient,
    query: String,
    cur: Option<SerdeSearchChannels>,
    offset: i32,
}

#[derive(Deserialize, Debug)]
struct SerdeSearchChannels {
    channels: Vec<Channel>,
}

impl<'c> Iterator for SearchChannelIterator<'c> {
    type Item = Channel;

    fn next(&mut self) -> Option<Channel> {
        let url = &format!(
            "/search/channels?query={}&limit=100&offset={}",
            self.query, self.offset
        );
        next_result!(self, &url, SerdeSearchChannels, channels)
    }
}

///////////////////////////////////////
// SearchGames
///////////////////////////////////////
pub struct SearchGameIterator<'c> {
    client: &'c TwitchClient,
    query: String,
    live_only: bool,
    cur: Option<SerdeSearchGames>,
    offset: i32,
}

#[derive(Deserialize, Debug)]
struct SerdeSearchGames {
    games: Vec<Game>,
}

impl<'c> Iterator for SearchGameIterator<'c> {
    type Item = Game;

    fn next(&mut self) -> Option<Game> {
        let url = &format!(
            "/search/games?query={}&live={}&limit=100&offset={}",
            self.query, self.live_only, self.offset
        );
        next_result!(self, &url, SerdeSearchGames, games)
    }
}

///////////////////////////////////////
// SearchStreams
///////////////////////////////////////
pub struct SearchStreamIterator<'c> {
    client: &'c TwitchClient,
    query: String,
    protocol: Option<Protocol>,
    cur: Option<SerdeSearchStreams>,
    offset: i32,
}

#[derive(Deserialize, Debug)]
struct SerdeSearchStreams {
    streams: Vec<Stream>,
}

pub enum Protocol {
    HLS,
    RTMP,
}

impl<'c> Iterator for SearchStreamIterator<'c> {
    type Item = Stream;

    fn next(&mut self) -> Option<Stream> {
        let mut path = String::from(format!(
            "/search/streams?query={}&limit=100&offset={}",
            self.query, self.offset
        ));
        path = match self.protocol {
            Some(Protocol::HLS) => path + "&hls=true",
            Some(Protocol::RTMP) => path + "&hls=false",
            None => path,
        };
        next_result!(self, &path, SerdeSearchStreams, streams)
    }
}

///////////////////////////////////////
// TESTS
///////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::super::new;
    use super::super::response;
    use super::super::tests::CLIENTID;

    #[test]
    fn channels() {
        let c = new(String::from(CLIENTID));

        match super::channels(&c, "twitch") {
            Ok(mut r) => assert_ne!(r.next().unwrap().id, 0),
            Err(r) => {
                println!("{:?}", r);
                assert!(false);
            }
        }
    }

    #[test]
    fn games() {
        let c = new(String::from(CLIENTID));

        match super::games(&c, "league", false) {
            Ok(mut r) => assert_ne!(r.next().unwrap().id, 0),
            Err(r) => {
                println!("{:?}", r);
                assert!(false);
            }
        }
    }

    #[test]
    fn streams() {
        let c = new(String::from(CLIENTID));

        match super::streams(&c, "twitch", None) {
            Ok(mut r) => assert_ne!(r.next().unwrap().id, 0),
            Err(r) => {
                println!("{:?}", r);
                assert!(false);
            }
        }
    }
}
