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
extern crate urlparse;

use std;
use std::io::Write;

use self::chrono::prelude::*;

use super::response::TwitchResult;
use super::users::User;
use super::TwitchClient;

/// Gets all active teams
///
/// #### Authentication: `None`
///
pub fn get_all<'c>(c: &'c TwitchClient) -> TwitchResult<TeamIterator<'c>> {
    let iter = TeamIterator {
        client: c,
        cur: None,
        offset: 0,
    };
    Ok(iter)
}

/// Gets a specified team object
///
/// #### Authentication: `None`
///
pub fn get(c: &TwitchClient, team_name: &str) -> TwitchResult<Team> {
    let r = r#try!(c.get::<Team>(&format!("/teams/{}", team_name)));
    Ok(r)
}

///////////////////////////////////////
// GetAllTeams
///////////////////////////////////////
pub struct TeamIterator<'c> {
    client: &'c TwitchClient,
    cur: Option<SerdeAllTeams>,
    offset: i32,
}

#[derive(Deserialize, Debug)]
pub struct Team {
    #[serde(rename = "_id")]
    pub id: i64,
    pub background: Option<String>,
    pub banner: String,
    pub created_at: DateTime<UTC>,
    pub display_name: String,
    pub info: String,
    pub logo: String,
    pub name: String,
    pub updated_at: DateTime<UTC>,
    pub users: Option<Vec<User>>,
}

#[derive(Deserialize, Debug)]
struct SerdeAllTeams {
    teams: Vec<Team>,
}

impl<'c> Iterator for TeamIterator<'c> {
    type Item = Team;

    fn next(&mut self) -> Option<Team> {
        let url = &format!("/teams?limit=100&offset={}", self.offset);
        next_result!(self, &url, SerdeAllTeams, teams)
    }
}

///////////////////////////////////////
// TESTS
///////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::super::new;
    use super::super::response;
    use super::super::tests::{CHANID, CLIENTID, TOKEN};

    #[test]
    fn get_all() {
        let c = new(String::from(CLIENTID));
        match super::get_all(&c) {
            Ok(mut r) => match r.next() {
                Some(team) => assert_ne!(team.id, 0),
                None => assert!(false),
            },
            Err(r) => {
                println!("{:?}", r);
                assert!(false);
            }
        }
    }
}
