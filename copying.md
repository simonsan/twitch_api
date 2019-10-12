Any file in this project that doesn't state otherwise, and isn't listed as an
exception below, is Copyright 2019-2019 libtwitch-rs authors, and licensed
under the terms of the GNU Affero General Public License Version 3, or
(at your option) any later version ("AGPL3+").
A copy of the license can be found in [legal/AGPL-v3](/legal/AGPL-v3).

_the libtwitch-rs authors_ are:

| Full name                   | aliases                     | E-Mail                                            |
|-----------------------------|-----------------------------|---------------------------------------------------|
| Simon San                   | simonsan                    | simon à systemli dawt org                         |
|-----------------------------|-----------------------------|---------------------------------------------------|
| Henry Wang                  | lavisheng                   | hen.wang à mail dawt utoronto dawt ca
|-----------------------------|-----------------------------|---------------------------------------------------|
| XXX                         | XXX                         | XXX à XXX dawt XXX                                |


If you're a first-time committer, add yourself to the above list. This is not
just for legal reasons, but also to keep an overview of all those nicknames.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License Version 3 for more details.

If you wish to include a file from libtwitch-rs in your project, make sure to
include all required legal info. The easiest way to do this would probably
be to include a copy of this file (`copying.md`), and to leave the file's
copyright header untouched.

Per-file license header guidelines:

In addition to this file, to prevent legal caveats, every source file *must*
include a header.

**libtwitch-rs-native** source files, that is, files that were created by
_the libtwitch-rs authors_, require the following one-line header, preferably in
the first line, as a comment:

    Copyright 20XX-20YY the libtwitch-rs authors. See copying.md for legal info.

`20XX` is the year when the file was created, and `20YY` is the year when the
file was last edited. When editing a file, make sure the last-modification year
is still correct.

**3rd-party** source files, that is, files that were taken from other open-
source projects, require the following, longer header:

    This file was ((taken|adapted)|contains (data|code)) from $PROJECT,
    Copyright 1337-2013 copyright owners name.
    It's licensed under the terms of the 3-clause BSD license.
    < any amount of lines of further legal information required by $PROJECT,
      such as a reference to a copy of the $PROJECT's README or AUTHORS file >
    < if third-party files from more than the one project were used in this
      file, copy the above any number of times >
    (Modifications|Other (data|code)|Everything else) Copyright 2014-2014 the libtwitch-rs authors.
    See copying.md for further legal info.

In addition to the libtwitch-rs header, the file's original license header should
be retained if in doubt.

The "license" line is required only if the file is not licensed as
"AGPLv3 or higher".

Authors of 3rd-party files should generally not be entered in the
"libtwitch-rs authors" list.

All 3rd-party files **must** be included in the following list.

List of all 3rd-party files in libtwitch-rs:

From [hatmatter/twitch_api](https://github.com/hatmatter/twitch_api) ([Apache License 2.0](/legal/Apache-v2))

 - `src/channel_feed.rs`
 - `src/channel.rs`
 - `src/chat.rs`
 - `src/communities.rs`
 - `src/games.rs`
 - `src/ingests.rs`
 - `src/lib.rs`
 - `src/response.rs`
 - `src/search.rs`
 - `src/streams.rs`
 - `src/teams.rs`
 - `src/users.rs`
 - `src/videos.rs`
