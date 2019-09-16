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

use std;
use std::collections::HashMap;
use std::io::Write;

use super::response::TwitchResult;
use super::TwitchClient;

/// Gets games sorted by number of current viewers on Twitch, most popular first
///
/// #### Authentication: `None`
///
pub fn top<'c>(c: &'c TwitchClient) -> TwitchResult<TopGames<'c>> {
    let iter = TopGames {
        client: c,
        cur: None,
        offset: 0,
    };
    Ok(iter)
}

///////////////////////////////////////
// GetTopGames
///////////////////////////////////////
pub struct TopGames<'c> {
    client: &'c TwitchClient,
    cur: Option<SerdeTopGames>,
    offset: i32,
}

#[derive(Deserialize, Debug)]
pub struct TopGame {
    pub channels: i32,
    pub viewers: i32,
    pub game: Game,
}

#[derive(Deserialize, Debug)]
pub struct Game {
    #[serde(rename = "_id")]
    pub id: i64,
    #[serde(rename = "box")]
    pub _box: HashMap<String, String>,
    pub giantbomb_id: i64,
    pub logo: HashMap<String, String>,
    pub name: String,
    pub popularity: i32,
}

#[derive(Deserialize, Debug)]
struct SerdeTopGames {
    top: Vec<TopGame>,
}

impl<'c> Iterator for TopGames<'c> {
    type Item = TopGame;

    fn next(&mut self) -> Option<TopGame> {
        let url = &format!("/games/top?limit=100&offset={}", self.offset);
        next_result!(self, &url, SerdeTopGames, top)
    }
}

///////////////////////////////////////
// TESTS
///////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::super::new;
    use super::super::tests::CLIENTID;

    #[test]
    fn top() {
        let c = new(String::from(CLIENTID));
        let mut r = super::top(&c).unwrap();
        r.next();
    }
}
