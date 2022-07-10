<h1 align="center">redtake</h1>

<p align="center">
  <a href="https://github.com/clitic/redtake">
    <img src="https://img.shields.io/github/downloads/clitic/redtake/total?logo=github&style=flat-square">
  </a>
  <!-- <a href="https://crates.io/crates/redtake">
    <img src="https://img.shields.io/crates/d/redtake?logo=rust&style=flat-square">
  </a>
  <a href="https://crates.io/crates/redtake">
    <img src="https://img.shields.io/crates/v/redtake?style=flat-square">
  </a>
  <a href="https://docs.rs/vsd/latest/redtake">
    <img src="https://img.shields.io/docsrs/redtake?logo=docsdotrs&style=flat-square">
  </a> -->
  <a href="https://github.com/clitic/redtake">
    <img src="https://img.shields.io/github/license/clitic/redtake?style=flat-square">
  </a>
  <a href="https://github.com/clitic/redtake">
    <img src="https://img.shields.io/github/repo-size/clitic/redtake?logo=github&style=flat-square">
  </a>
  <a href="https://github.com/clitic/redtake">
    <img src="https://img.shields.io/tokei/lines/github/clitic/redtake?style=flat-square">
  </a>
</p>

<p align="center">
  <a href="#Installations">Installations</a>
  &nbsp;&nbsp;&nbsp;|&nbsp;&nbsp;&nbsp;
  <a href="#Usage">Usage</a>
</p>

redtake is based on [RedditVideoMakerBot](https://github.com/elebumm/RedditVideoMakerBot) and shares similar functionality with it's [v2.1](https://github.com/elebumm/RedditVideoMakerBot/releases/tag/2.1).

redtake is abbreviation of **red**dit **take**s. It can be used to create videos from reddit posts with screenshots of title and comments with an tts voice narrating them. No video editing is required to create these videos.

## Video Samples

[Post](https://www.reddit.com/r/AskReddit/comments/vus3ch/what_hits_different_at_2am)

https://user-images.githubusercontent.com/71246790/178131299-da1c480e-be85-434c-bd2d-412ce5648ede.mp4

## Features

- [x] Choose between subreddit or subreddit thread.
- [x] Full control over rendering of videos with customization options with rich toml configuration file.
- [x] Light and dark screenshot themes.
- [x] No login and authentication required.
- [x] NSFW post filter.
- [x] Select between various text to speech voices.
- [x] Supports to add background music to video.
- [x] Supports to add progress bar to video.
- [x] Supports to render videos without a background video.
- [ ] Google text to speech voice.
- [ ] Post translation.

## Installations

Dependencies

- [ffmpeg](https://www.ffmpeg.org/download.html) and [ffprobe](https://www.ffmpeg.org/download.html)
- [chrome](https://www.google.com/chrome) / [chromium](https://www.chromium.org/getting-involved/download-chromium) 

Visit [releases](https://github.com/clitic/redtake/releases) for prebuilt binaries. You just need to copy that binary to any path specified in your `PATH` environment variable.

> **Releases are built from main branch and updated once a commit is made to main branch**

<!-- ### Through Cargo

```bash
cargo install redtake
```

### On x86_64 Linux

```bash
$ wget https://github.com/clitic/redtake/releases/download/v0.1.0-main/redtake-v0.1.0-mainx86_64-unknown-linux-musl.tar.gz -O redtake-v0.1.0-main.tar.gz
$ tar -xzf redtake-v0.1.0-main.tar.gz -C /usr/local/bin/
$ chmod +x /usr/local/bin/redtake
$ rm redtake-v0.1.0-main.tar.gz
``` -->

## Usage

First create a video project and download all required assets with these commands.

- For creating multiple video project from subreddit hots.

```bash
$ redtake new r/AskReddit
```

- For creating a single video project from a specific subreddit thread.

```bash
$ redtake new https://www.reddit.com/r/AskReddit/comments/vus3ch/what_hits_different_at_2am
```

Above given commands will create directories for threads from `r/AskReddit` starting with `Project THREAD_ID ...`. These created directories are video project. Each video project contains an [Project.toml](https://github.com/clitic/redtake/blob/main/docs/Project.toml) which defines how to render the video.

Now you can go inside any video project directory and run.

```bash
$ redtake render
```

Once a video project is rendered you will see `export.mp4` file. Running render again will create `export (1).mp4` and so on.

To see the underlying ffmpeg commad used for rendering video use `-c / --command` flag.

```bash
$ redtake render -c
```

## Building From Source

- Install [Rust](https://www.rust-lang.org)
- Clone Repository

```bash
git clone https://github.com/clitic/redtake.git
```

- Build Release (inside redtake directory)

```bash
cargo build --release
```

## License

&copy; 2022 clitic

This repository is licensed under the MIT license. See LICENSE for details.
