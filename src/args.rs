use clap::{Parser, Subcommand};

use crate::tts::Voices;

/// Create reddit text to speech videos without any editing.
#[derive(Debug, Parser)]
#[clap(version, author = "clitic <clitic21@gmail.com>", about)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    New(New),
    Render(Render),
    Tts(Tts),
    /// List available voices for tts.
    /// Listed voices are in format of [language_code - country_code - service_provider - voice_name].
    Voices,
}

/// Create new project and download all assets.
#[derive(Debug, clap::Args)]
pub struct New {
    /// Links from reddit.com website.
    /// Supported patterns are r/AskReddit and https://reddit.com/r/...
    /// If input doesn't specifies any subreddit thread then redtake will fetch hot threads from that subreddit.
    #[clap(required = true)]
    pub input: String,

    /// Use light theme for taking screenshots instead of dark theme.
    #[clap(long)]
    pub light: bool,

    /// Maximum limit to fetch reddit hot threads from subreddit.
    #[clap(long, default_value_t = 25)]
    pub limit: u16,

    /// Maximum video duration.
    #[clap(long, default_value_t = 50.0)]
    pub max_duration: f32,

    /// Maximum comment length.
    #[clap(long, default_value_t = 500)]
    pub max_length: usize,

    /// Allow generating projects from 18+ threads.
    #[clap(long)]
    pub nsfw: bool,

    /// Use https://warp-co.rs CORS proxy to bypass geolocation restrictions for some tts voices.
    #[clap(short, long)]
    pub tts_proxy: bool,

    /// Skip creating existing projects.
    /// By default projects are overriden if they already exists.
    #[clap(long)]
    pub skip: bool,

    /// Select voice model for speech synthesis.
    #[clap(long, arg_enum, default_value_t = Voices::EnUSTiktokFemale2, hide_possible_values = true)]
    pub voice: Voices,
}

/// Render video using ffmpeg.
#[derive(Debug, clap::Args)]
pub struct Render {
    /// Display ffmpeg command instead of rendering the video.
    /// Generated command may not work as expected if audio/tts.mp3 doesn't exists.
    /// audio/tts.mp3 is concatenation of .mp3 present in audio/ directory.  
    #[clap(short, long)]
    pub command: bool,
}

/// Perform and save text to speech locally.
#[derive(Debug, clap::Args)]
pub struct Tts {
    /// Text to be synthesized.
    #[clap(required = true)]
    pub text: String,

    /// Output path for synthesized speech in .mp3 format.
    #[clap(short, long, required = true)]
    pub output: String,

    /// Print duration of saved file.
    #[clap(short, long)]
    pub duration: bool,

    /// Select voice model for speech synthesis.
    #[clap(long, arg_enum, default_value_t = Voices::EnUSTiktokFemale2, hide_possible_values = true)]
    pub voice: Voices,

    /// Use https://warp-co.rs CORS proxy to bypass geolocation restrictions for some tts voices.
    #[clap(short, long)]
    pub tts_proxy: bool,
}

pub fn parse() -> Args {
    Args::parse()
}
