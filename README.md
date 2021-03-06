AniTUI is a CLI (and in the future a TUI) app for searching and
wathching anime in MPV. This is a Rust rewrite (quite literally a
rewrite) of [Pystardust\'s
ani-cli](https://github.com/pystardust/ani-cli). Thanks to ani-cli for
decoding the magic of goload.pro or whatever mirror they have in the
future.

# Usage

AniTUI uses an ID in a format of `<source:id>`. To perform commands like
`watch` or `ep-count` you need to first get an ID via `search`.

``` console
$ ani-tui search "keywords"
```

The output will contain a list of titles and IDs in `<>`. Copy the ID
and use it in other commands.

``` console
$ ani-tui detail "<ID>"

$ ani-tui list-eps "<ID>"
```

`detail` will give you the most info about an anime like its
description, ID, episode count and of course the title. `list-eps` only
yields the title and number of episodes.

``` console
$ ani-tui watch "<ID>" 1
```

Watch an episode. Replace `ID` and `1` (episode number) with your
values.

You can use either `help command`, `-h` or `--help` to get a help
message explaining how to use AniTUI.

------------------------------------------------------------------------

# Contributing

1.  View open issues/projects or create your own issue
2.  Fork
3.  Write some code
4.  Refactor
5.  Write docs
6.  Make a pull request explaining the changes

## Branch guide

### main

A \"checkpoint\" of sorts. Contains the latest stable commit. Never
commit to main, instead commit to \`dev\` and merge (PR not required).

### dev

Contains the work in progress commits. Unlike main this can have failing
tests and missing docs.
