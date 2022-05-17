# AniTUI

While `ani-cli` is pretty good, it aims to be minimalistic. This has compromized some functionality.

AniTUI attemts to have all the features of `ani-cli` while adding some more interactivity.

# Features

## CLI

- Searching on <https://goload.pro>
- Getting information about an anime such as its description and episode list

# Documentation

- Command line options are automatically documented via `clap`. Do `--help` or `-h` to read a short manual.

- Code documentation is avaliable via `cargo doc`

# When is this going to crates.io?

Not yet. For a `0.1.0` I would like to have most `ani-cli` features complete, such as:

- [x] Searching by title
- [x] Episode list
- [x] Description
- [ ] Watching anime in mpv

---

# Contributing

1. View open issues/projects or create your own issue
2. Fork 
2. Write a test → see the test fail
3. Make the test pass
4. Refactor
4. Write docs
3. Make a pull request explaining the changes

# Branch guide

### main

A "checkpoint" of sorts. Contains the latest stable commit. Never commit to main, instead commit to `dev` and merge (PR not required).

### dev

Contains the work in progress commits. Unlike main this can have failing tests and missing docs.
