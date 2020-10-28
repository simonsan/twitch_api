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

use serde::{
    Deserialize,
    Serialize,
};

use super::users::User;

use crate::{
    response::TwitchResult,
    TwitchClient,
};

use serde_json::Value;
use std::{
    self,
    collections::HashMap,
    io::Write,
};

/// Gets a specified post from a specified channel feed
///
/// If authentication is provided, the user_ids
/// array in the response body contains the
/// requesting user’s ID, if they have reacted to a post.
///
/// #### Authentication: *Optional scope: any scope*
pub fn get_post(
    c: &TwitchClient,
    chan_id: &str,
    post_id: &str,
) -> TwitchResult<FeedPost>
{
    let r =
        c.get::<FeedPost>(&format!("/feed/{}/posts/{}", chan_id, post_id))?;
    Ok(r)
}

/// Gets posts from a specified channel feed
///
/// If authentication is provided, the user_ids
/// array in the response body contains the
/// requesting user’s ID, if they have reacted to a post.
///
/// #### Authentication: *Optional scope: any scope*
pub fn get_posts<'c>(
    c: &'c TwitchClient,
    chan_id: &str,
) -> TwitchResult<FeedPosts<'c>>
{
    let iter = FeedPosts {
        client: c,
        chan_id: String::from(chan_id),
        cur: None,
        cursor: None,
    };
    Ok(iter)
}

/// Creates a post in a specified channel feed
///
/// # Arguments
///
/// * `data` - Text of the post
///
/// #### Authentication: `channel_feed_edit`
pub fn new_post(
    c: &TwitchClient,
    chan_id: &str,
    data: &str,
) -> TwitchResult<NewFeedPostResponse>
{
    let r = c.post::<NewContent, NewFeedPostResponse>(
        &format!("/feed/{}/posts", chan_id),
        &NewContent { content: data },
    )?;
    Ok(r)
}

/// Deletes a specified post in a specified channel feed
///
/// #### Authentication: `channel_feed_edit`
pub fn delete_post(
    c: &TwitchClient,
    chan_id: &str,
    post_id: &str,
) -> TwitchResult<FeedPost>
{
    let r =
        c.delete::<FeedPost>(&format!("/feed/{}/posts/{}", chan_id, post_id))?;
    Ok(r)
}

/// Creates a reaction to a specified post in a specified channel feed
///
/// The reaction is specified by an emote value, which is
/// either an ID (for example, “25” is Kappa) or the string
/// “endorse” (which corresponds to a default face emote).
///
/// #### Authentication: `channel_feed_edit`
pub fn new_post_reaction(
    c: &TwitchClient,
    chan_id: &str,
    post_id: &str,
    emote_id: &str,
) -> TwitchResult<NewReactionResponse>
{
    let r = c.post::<Value, NewReactionResponse>(
        &format!(
            "/feed/{}/posts/{}/reactions?emote_id={}",
            chan_id, post_id, emote_id
        ),
        &Value::Null,
    )?;
    Ok(r)
}

/// Deletes a specified reaction to a specified post in a specified channel feed
///
/// The reaction is specified by an emote ID (for example,
/// “25” is Kappa) or the string “endorse” (which corresponds
/// to a default face emote).
///
/// #### Authentication: `channel_feed_edit`
pub fn delete_post_reaction(
    c: &TwitchClient,
    chan_id: &str,
    post_id: &str,
    emote_id: &str,
) -> TwitchResult<DelReactionResponse>
{
    let r = c.delete::<DelReactionResponse>(&format!(
        "/feed/{}/posts/{}/reactions?emote_id={}",
        chan_id, post_id, emote_id
    ))?;
    Ok(r)
}

/// Gets all comments on a specified post in a specified channel feed
///
/// If authentication is provided, the permissions for
/// the comment are returned in the response; otherwise,
/// no permissions are returned.
///
/// #### Authentication: *Optional scope: any scope*
pub fn get_comments<'c>(
    c: &'c TwitchClient,
    chan_id: &str,
    post_id: &str,
) -> TwitchResult<FeedPostCommentIterator<'c>>
{
    let iter = FeedPostCommentIterator {
        client: c,
        chan_id: String::from(chan_id),
        post_id: String::from(post_id),
        cur: None,
        cursor: None,
    };
    Ok(iter)
}

/// Creates a comment to a specified post in a specified channel feed
///
/// # Arguments
///
/// * `data` - Text of the comment
///
/// #### Authentication: `channel_feed_edit`
pub fn new_comment(
    c: &TwitchClient,
    chan_id: &str,
    post_id: &str,
    data: &str,
) -> TwitchResult<FeedPostComment>
{
    let r = c.post::<NewContent, FeedPostComment>(
        &format!("/feed/{}/posts/{}/comments", chan_id, post_id),
        &NewContent { content: data },
    )?;
    Ok(r)
}

/// Deletes a specified comment on a specified post in a specified channel feed
///
/// #### Authentication: `channel_feed_edit`
pub fn delete_comment(
    c: &TwitchClient,
    chan_id: &str,
    post_id: &str,
    comment_id: &str,
) -> TwitchResult<FeedPostComment>
{
    let r = c.delete::<FeedPostComment>(&format!(
        "/feed/{}/posts/{}/comments/{}",
        chan_id, post_id, comment_id
    ))?;
    Ok(r)
}

/// Creates a reaction to a specified comment on a specified post in a specified
/// channel feed
///
/// The reaction is specified by an emote value, which
/// is either an ID (for example, “25” is Kappa) or the
/// string “endorse” (which corresponds to a default
/// face emote).
///
/// #### Authentication: `channel_feed_edit`
pub fn new_comment_reaction(
    c: &TwitchClient,
    chan_id: &str,
    post_id: &str,
    comment_id: &str,
) -> TwitchResult<NewReactionResponse>
{
    let r = c.post::<Value, NewReactionResponse>(
        &format!(
            "/feed/{}/posts/{}/comments/{}/reactions?emote_id=endorse",
            chan_id, post_id, comment_id
        ),
        &Value::Null,
    )?;
    Ok(r)
}

/// Deletes a reaction to a specified comment on a specified post in a specified
/// channel feed
///
/// The reaction is specified by an emote value, which
/// is either an ID (for example, “25” is Kappa) or the
/// string “endorse” (which corresponds to a default
/// face emote).
///
/// #### Authentication: `channel_feed_edit`
pub fn delete_comment_reaction(
    c: &TwitchClient,
    chan_id: &str,
    post_id: &str,
    comment_id: &str,
) -> TwitchResult<DelReactionResponse>
{
    let r = c.delete::<DelReactionResponse>(&format!(
        "/feed/{}/posts/{}/comments/{}/reactions?emote_id=endorse",
        chan_id, post_id, comment_id
    ))?;
    Ok(r)
}

///////////////////////////////////////
// FeedPost definitions
///////////////////////////////////////
#[derive(Deserialize, Debug)]
pub struct FeedPost {
    pub body: String,
    pub comments: Option<SerdeFeedPostComments>,
    pub created_at: DateTime<Utc>,
    pub deleted: Option<bool>,
    pub embeds: Option<Vec<Value>>,
    pub emotes: Option<Vec<Value>>,
    pub id: String,
    pub permissions: Option<FeedPostPermissions>,
    pub reactions: Option<HashMap<String, Value>>,
    pub user: Option<User>,
}

#[derive(Deserialize, Debug)]
pub struct FeedPostPermissions {
    pub can_delete: bool,
    pub can_moderate: bool,
    pub can_reply: bool,
}

#[derive(Deserialize, Debug)]
pub struct FeedPostCommentPermissions {
    pub can_delete: bool,
}

#[derive(Deserialize, Debug)]
pub struct FeedPostEmotes {
    pub start: i32,
    pub end: i32,
    pub id: i64,
    pub set: i64,
}

#[derive(Serialize, Debug)]
pub struct NewContent<'a> {
    pub content: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct NewFeedPostResponse {
    pub post: FeedPost,
    pub tweet: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct NewReactionResponse {
    pub created_at: DateTime<Utc>,
    pub emote_id: String,
    pub id: String,
    pub user: Option<User>,
}

#[derive(Deserialize, Debug)]
pub struct DelReactionResponse {
    pub deleted: bool,
}

///////////////////////////////////////
// FeedPosts
///////////////////////////////////////
#[derive(Debug)]
pub struct FeedPosts<'c> {
    client: &'c TwitchClient,
    chan_id: String,
    cur: Option<SerdeFeedPosts>,
    cursor: Option<String>,
}

impl<'c> Iterator for FeedPosts<'c> {
    type Item = FeedPost;

    fn next(&mut self) -> Option<FeedPost> {
        let url = format!("/feed/{}/posts?", &self.chan_id);
        next_result_cursor!(self, &url, SerdeFeedPosts, posts)
    }
}

#[derive(Deserialize, Debug)]
struct SerdeFeedPosts {
    pub posts: Vec<FeedPost>,
    pub _cursor: Option<String>,
}

///////////////////////////////////////
// FeedPostComments
///////////////////////////////////////
#[derive(Debug)]
pub struct FeedPostCommentIterator<'c> {
    client: &'c TwitchClient,
    chan_id: String,
    post_id: String,
    cur: Option<SerdeFeedPostComments>,
    cursor: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct FeedPostComment {
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub deleted: bool,
    pub emotes: Vec<FeedPostEmotes>,
    pub id: String,
    pub permissions: Option<FeedPostCommentPermissions>,
    pub reactions: HashMap<String, Value>,
    pub user: User,
}

#[derive(Deserialize, Debug)]
pub struct SerdeFeedPostComments {
    pub _total: i32,
    pub _cursor: Option<String>,
    pub comments: Vec<FeedPostComment>,
}

impl<'c> Iterator for FeedPostCommentIterator<'c> {
    type Item = FeedPostComment;

    fn next(&mut self) -> Option<FeedPostComment> {
        let url = format!(
            "/feed/{}/posts/{}/comments?",
            &self.chan_id, &self.post_id
        );
        next_result_cursor!(self, &url, SerdeFeedPostComments, comments)
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
            CHANID,
            CLIENTID,
            TOKEN,
        },
    };

    use crate::kraken::channel_feed::{
        delete_comment,
        delete_comment_reaction,
        delete_post,
        delete_post_reaction,
        get_posts,
        new_comment,
        new_comment_reaction,
        new_post,
        new_post_reaction,
    };

    #[test]
    #[ignore]
    fn post() {
        let mut c = new(String::from(CLIENTID));
        c.set_oauth_token(TOKEN);

        // create post
        match new_post(&c, CHANID, "channel_feed::tests::FeedPost") {
            Ok(r) => assert_eq!(r.post.body, "channel_feed::tests::FeedPost"),
            Err(r) => {
                println!("{:?}", r);
                assert!(false);
            }
        };

        // count posts
        match get_posts(&c, CHANID) {
            Ok(r) => assert!(r.count() > 0),
            Err(r) => {
                println!("{:?}", r);
                assert!(false);
            }
        }

        // create post reactions
        for post in get_posts(&c, CHANID).unwrap() {
            match new_post_reaction(&c, CHANID, &post.id, "25") {
                Ok(_r) => (),
                Err(_r) => assert!(false),
            }
        }

        // read and delete post reactions
        for post in get_posts(&c, CHANID).unwrap() {
            match post.reactions.expect("no reactions for post").get("25") {
                Some(_r) => (),
                None => assert!(false),
            }

            match delete_post_reaction(&c, CHANID, &post.id, "25") {
                Ok(r) => assert!(r.deleted),
                Err(r) => {
                    println!("{:?}", r);
                    assert!(false);
                }
            }
        }

        // count post reactions
        for post in get_posts(&c, CHANID).unwrap() {
            // count reactions
            assert_eq!(
                post.reactions
                    .expect("no reactions for post")
                    .keys()
                    .count(),
                0
            );
        }

        // delete posts
        for post in get_posts(&c, CHANID).unwrap() {
            match delete_post(&c, CHANID, &post.id) {
                Ok(r) => assert!(r.id == post.id && r.deleted.unwrap() == true),
                Err(r) => {
                    println!("{:?}", r);
                    assert!(false);
                }
            }
        }

        // count posts
        match get_posts(&c, CHANID) {
            Ok(r) => assert!(r.count() == 0),
            Err(r) => {
                println!("{:?}", r);
                assert!(false);
            }
        }
    }

    #[test]
    #[ignore]
    fn comment() {
        let mut c = new(String::from(CLIENTID));
        c.set_oauth_token(TOKEN);

        // create post
        if let Ok(r) =
            new_post(&c, CHANID, "channel_feed::tests::FeedPostComment")
        {
            let post = r.post;

            // create comment
            if let Ok(comment) = new_comment(
                &c,
                CHANID,
                &post.id,
                "channel_feed::tests::FeedPostComment comment",
            ) {
                // create comment reaction
                match new_comment_reaction(&c, CHANID, &post.id, &comment.id) {
                    Ok(_r) => (),
                    Err(r) => {
                        println!("{:?}", r);
                        assert!(false);
                    }
                };

                // delete comment reaction
                match delete_comment_reaction(&c, CHANID, &post.id, &comment.id)
                {
                    Ok(r) => assert_eq!(r.deleted, true),
                    Err(r) => {
                        println!("{:?}", r);
                        assert!(false);
                    }
                }

                // delete comment
                match delete_comment(&c, CHANID, &post.id, &comment.id) {
                    Ok(r) => assert_eq!(r.deleted, true),
                    Err(r) => {
                        println!("{:?}", r);
                        assert!(false);
                    }
                }
            }
            else {
                assert!(false);
            }

            // delete post
            for post in get_posts(&c, CHANID).unwrap() {
                match delete_post(&c, CHANID, &post.id) {
                    Ok(_r) => (),
                    Err(r) => {
                        println!("{:?}", r);
                        assert!(false);
                    }
                }
            }
        }
        else {
            assert!(false);
        }
    }
}
