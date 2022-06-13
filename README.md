# AniTUI

An app to watch your anime from the shell.

While `ani-cli` is pretty good, it aims to be minimalistic. This has compromized some functionality.

AniTUI attemts to have all the features of `ani-cli` while adding some more interactivity.

# `man ani-tui`

This section is a manual on how to use `ani-tui`. Please note that this interface may change later.

## Searching for anime to watch

```console
$ ani-tui search "pokemon"

 • Pokemon XY
   pokemon-xy

 • Pokemon XY&Z
   pokemon-xyz

 • Pokemon (Dub)
   pokemon-dub

 • Pokemon (2019)
   pokemon-2019

 ... more results
```

Here the text near a `•` is the title and text below is an *identifier*. Use this identifier in other subcommands like `detail` or `watch`

## Getting details

```console
$ ani-tui detail "pokemon-generations"

Pokemon Generations
 [pokemon-generations]

Pokémon Generations revisits each generation of the Pokémon video game series to shed new light on some timeless moments. From the earliest days in the Kanto region to the splendor of the Kalos region, go behind the scenes and witness Pokémon history with new eyes!

  18 Pokemon Generations Episode 18
  17 Pokemon Generations Episode 17
  16 Pokemon Generations Episode 16
  15 Pokemon Generations Episode 15
  14 Pokemon Generations Episode 14
  13 Pokemon Generations Episode 13
  12 Pokemon Generations Episode 12
  11 Pokemon Generations Episode 11
  
  ... more episodes
```

- The first two lines are the same as in `search`
- The next paragraph is a description 
- After is a list of episodes

You can also use `list-eps` to only see a list of episodes.

## Watching anime

``` console
$ ani-tui watch "pokemon-xy" 1
Launching MPV
```

First argument is anime identifier and the other one is the episode number. This command will get all the info it needs from the website, passes it to mpv and exits. Please wait for MPV to appear.

# Features

## CLI

- Searching on <https://goload.pro>
- Getting information about an anime such as its description and episode list
- Watching an anime via `mpv`.

# Documentation

- Command line options are automatically documented via `clap`. Do `--help` or `-h` to read a short manual.

- Code documentation is avaliable via `cargo doc`

# When is this going to crates.io?

Not yet. For a `0.1.0` I would like to have most `ani-cli` features complete, such as:

- [x] Searching by title
- [x] Episode list
- [x] Description
- [x] Watching anime in mpv

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
