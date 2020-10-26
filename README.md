# Typesync...but in Rust!

[Typesync](http://typesync.tech) is a web application in React with an api written in Python.  This is a rewrite of both the front end and the api in Rust.  This project leverages the [seed](), [rocket](), and [diesel]() crates.  I owe it to [this tutorial](https://erwabook.com/intro/index.html) for describing how I could design and implement a full stack Rust project.

## Features Missing from Rewrite

There are still features in the original version of the site that need to be rewritten.

- [ ] Use MySQL and not Sqlite3
- [ ] Cache (artist, title) searches
- [x] Make the song data and the leaderboard data distinct API calls
- [ ] Add styling, make it look pretty, format leaderboards, etc.
- [ ] Add small song display on test page
- [ ] Add progress bar on test page
- [ ] Display default leaderboard when no results are available
- [ ] Disallow user from starting song without selecting mode
- [ ] Host at rust.typesync.tech, or typesync.tech if I'm confident enough

## New features to add

This should be done only after completing the above, as it will be equivalent or better than the previous site at that point.

- [ ] Add most popular songs to home page
- [ ] Add timed mode (how much of a song can you type in 1 minute)
- [ ] Add a bit more UX, many people don't know what the website done on entry (perhaps a message board for new ips?)
- [ ] Routing, that is, the song summary of id 2 should be typesync.tech/song/2
- [ ] A comprehensive leaderboard page, possibly with statistics and graphs
