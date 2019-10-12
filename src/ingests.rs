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
use super::TwitchClient;

/// Gets a list of Twitch ingest servers
///
/// The Twitch ingesting system is the first stop for
/// a broadcast stream. An ingest server receives your
/// stream, and the ingesting system authorizes and
/// registers streams, then prepares them for viewers.
///
/// #### Authentication: `None`
///
pub fn servers(c: &TwitchClient) -> TwitchResult<IngestServerList> {
    let r = r#try!(c.get::<IngestServerList>("/ingests"));
    Ok(r)
}

///////////////////////////////////////
// GetIngestServerList
///////////////////////////////////////
#[derive(Deserialize, Debug)]
pub struct IngestServerList {
    pub ingests: Vec<IngestServer>,
}

#[derive(Deserialize, Debug)]
pub struct IngestServer {
    pub _id: i64,
    pub availability: f32,
    pub default: bool,
    pub name: String,
    pub url_template: String,
}

///////////////////////////////////////
// TESTS
///////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::super::new;
    use super::super::tests::CLIENTID;

    #[test]
    fn servers() {
        let c = new(String::from(CLIENTID));
        match super::servers(&c) {
            Ok(r) => assert!(r.ingests.len() > 0),
            Err(r) => {
                println!("{:?}", r);
                assert!(false);
            }
        }
    }
}
