# AniTUI

While `ani-cli` is pretty good, it aims to be minimalistic. This has compromized some functionality.

AniTUI attemts to have all the features of `ani-cli` and add some more interactivity.

# Features

**This project is being refactored so latest stable commit is [f4755d0b](https://github.com/m-kuzmin/ani-tui/tree/f4755d0bfd7e1874da82f0b6851e2e9b4e6f7b65)**

You can already search and view basic info about an anime title.

# Documentation

Command line options are automatically documented via `clap`. Do `--help` or `-h` to read a short manual.

Code docs are not written yet due to unstable API

# When is this going to crates.io?

Not yet. For a `0.1.0` I would like to have most ani-cli features to be working.

---

# Contributing

1. View open issues or projects or create your own issue.
2. Fork and code.
3. Make a pull request explaining the changes.

## Branch guide

### dev
This is for WIP commits. However when looking through commit history you may want to look only at major points when things were finally implemented. This is why main exists

### main
Acts like a squash for dev branch without squashing anything. Only put major commits here and use fast-forward merge.

See [this answer on SO: dev branch](https://stackoverflow.com/questions/32826370/why-do-we-need-dev-branch#32826537) for details.
