# libtwitch-rs [![docs.rs][5]][6] [![crates.io version][1]][2] [![dependency status][3]][4] ![GNU AGPLv3][agpl-logo]
A Rust library for the Twitch APIv5 (Kraken).

# Contributing
Help for this project is highly appreciated. This was built against Twitch APIv5 (Kraken). 
Take a look into the [issues](https://github.com/simonsan/libtwitch-rs/issues) if you want to contribute to the project.

Fork it, implement your changes and make a Pull-Request against the `feature-dev` branch of this repo. 

# Usage
```
use libtwitch_rs;
use libtwitch_rs::users;

...

let mut c = libtwitch_rs::new(String::from(CLIENTID));
c.set_oauth_token(TOKEN);

if let Some(user) = match users::get(&c) {
    Ok(r)  => { assert!(r.email.is_some()); Some(r) },
    Err(r) => { println!("{:?}", r); assert!(false); None }
    } {
    let user_id = user.id.to_string();

    match users::get_by_id(&c, &user_id) {
        Ok(r)  => assert_eq!(r.name, user.name),
        Err(r) => { println!("{:?}", r); assert!(false); }
    }
}
```

# Supported API versions

__Important Note:__ Kraken is deprecated and Helix (the new API version) 
is out of scope of this repository currently. 

Endpoints         | Kraken/v5          | 
:----------------:|:------------------:|
Analytics         | :heavy_minus_sign: |
Bits              | :heavy_check_mark: |
Channels          | :heavy_check_mark: |
Chat              | :heavy_check_mark: |
Clips             | :heavy_check_mark: |
Collections       | :heavy_check_mark: |
Entitlements      | :heavy_minus_sign: |
Games             | :heavy_check_mark: |
Ingests           | :heavy_check_mark: |
Moderation        | :heavy_minus_sign: |
Search            | :heavy_check_mark: |
Streams           | :heavy_check_mark: |
Subscriptions     | :heavy_minus_sign: |
Tags              | :heavy_minus_sign: |
Teams             | :heavy_check_mark: |
Users             | :heavy_check_mark: |
Videos            | :heavy_check_mark: |
Webhooks          | :heavy_minus_sign: |


# Links
## Helpful Documentation
- [Token test, infos incl. Channel_ID](https://codepen.io/Alca/pen/VwwazOK)
- [Achieve Authorization for a bot](http://web.archive.org/web/20191016034229/https://d-fischer.github.io/twitch-chat-client/docs/examples/basic-bot.html)
- [Twitch API Guide](https://dev.twitch.tv/docs/api/guide)

## Token Generation
- [Token generator](https://twitchtokengenerator.com/)
- [Twitch IRC OAuth generator](https://twitchapps.com/tmi/)

## Repositories:
- [Twitch library (C#)](https://github.com/TwitchLib/TwitchLib)
- [Twitch library (PHP)](https://github.com/Dkamps18/PHP-Twitch-lib/)
- [Python Twitch client](https://github.com/tsifrer/python-twitch-client)
- [TwitchIO, Twtich Bot/API wrapper (Python)](https://github.com/TwitchIO/TwitchIO)


# License
GNU AGPL-3.0-or-later; see [copying.md](copying.md) and [legal/AGPL-v3](legal/AGPL-v3).


[1]: https://img.shields.io/crates/v/libtwitch-rs.svg?style=flat-square
[2]: https://crates.io/crates/libtwitch-rs
[3]: https://deps.rs/repo/github/age-rs/libtwitch-rs/status.svg
[4]: https://deps.rs/repo/github/age-rs/libtwitch-rs
[5]: https://docs.rs/libtwitch-rs/badge.svg
[6]: https://docs.rs/libtwitch-rs/latest/
[agpl-logo]: https://www.gnu.org/graphics/agplv3-88x31.png
