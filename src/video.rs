use std::io::{Read, Write};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Video {
    width: u16,
    height: u16,
    crop_x: f32,
    crop_y: f32,
    background: String,
    background_start_time: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Music {
    audio: String,
    volume: f32,
    start_time: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProgressBar {
    enable: bool,
    solid: bool,
    height: u16,
    color: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Overlay {
    opacity: f32,
    padding: u16,
    overlays: Vec<OverlayMeta>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OverlayMeta {
    image: String,
    audio: String,
    duration: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    video: Video,
    music: Music,
    progress_bar: ProgressBar,
    overlay: Overlay,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            video: Video {
                width: 1080,
                height: 1920,
                crop_x: 1166.6,
                crop_y: 0.0,
                background: "color=white".to_owned(),
                background_start_time: 0.0,
            },
            music: Music {
                audio: "".to_owned(),
                volume: 0.1,
                start_time: 0.0,
            },
            progress_bar: ProgressBar {
                enable: true,
                solid: false,
                height: 10,
                color: "#FF4500".to_owned(),
            },
            overlay: Overlay {
                opacity: 1.0,
                padding: 100,
                overlays: vec![],
            },
        }
    }
}

impl Project {
    pub fn load_toml(path: &str) -> Result<Self> {
        let mut project = String::new();
        std::fs::File::open(path)?.read_to_string(&mut project)?;
        Ok(toml::from_str::<Self>(&project)?)
    }

    pub fn save_toml(&self, path: &str) -> Result<()> {
        std::fs::File::create(path)?.write(toml::to_string_pretty(&self)?.as_bytes())?;
        Ok(())
    }

    pub fn add_overlay(&mut self, image: &str, tts: &str, duration: f32) {
        self.overlay.overlays.push(OverlayMeta {
            image: image.to_owned(),
            audio: tts.to_owned(),
            duration: duration,
        });
    }

    pub fn len(&self) -> f32 {
        self.overlay
            .overlays
            .iter()
            .map(|x| x.duration)
            .sum::<f32>()
    }

    fn has_background(&self) -> bool {
        if self.video.background.starts_with("color=") {
            false
        } else {
            true
        }
    }

    fn has_bgm(&self) -> bool {
        if self.music.audio == "" {
            false
        } else {
            true
        }
    }

    fn merge_tts(&self) -> Result<()> {
        let mut tts_file = std::fs::File::create("audio/tts.txt")?;

        for overlay in &self.overlay.overlays {
            tts_file.write(
                format!("file '{}'\n", overlay.audio.trim_start_matches("audio/")).as_bytes(),
            )?;
        }

        std::process::Command::new("ffmpeg")
            .args([
                "-y",
                "-f",
                "concat",
                "-i",
                "audio/tts.txt",
                "-c",
                "copy",
                "audio/tts.mp3",
            ])
            .stderr(std::process::Stdio::null())
            .spawn()?
            .wait()?;

        Ok(())
    }

    fn generate(&self) -> Vec<String> {
        let mut args = vec!["-hide_banner", "-y"];
        let mut filter_complex = "".to_string();
        let background_start_time = self.video.background_start_time.to_string();
        let bgm_start_time = self.music.start_time.to_string();
        let video_length = self.len().to_string();
        let has_background = self.has_background();
        let has_bgm = self.has_bgm();

        // Trim
        if has_background {
            args.push("-ss");
            args.push(&background_start_time);
            args.push("-t");
            args.push(&video_length);
        }

        // Background
        if has_background {
            args.push("-i");
            args.push(&self.video.background);
        }

        // Comments
        for overlay in &self.overlay.overlays {
            args.push("-i");
            args.push(&overlay.image);
        }

        // TTS
        args.push("-i");
        args.push("audio/tts.mp3");

        // Background Music
        if has_bgm {
            args.push("-ss");
            args.push(&bgm_start_time);
            args.push("-t");
            args.push(&video_length);
            args.push("-i");
            args.push(&self.music.audio);
        }

        // Filter Duration
        if !has_background {
            args.push("-t");
            args.push(&video_length);
        }

        // Video Filter
        args.push("-filter_complex");

        // Background Resize And Crop
        if has_background {
            filter_complex += &format!(
                "[0:v]scale=-1:{height},crop={width}:{height}:{crop_x}:{crop_y}[main];",
                height = self.video.height,
                width = self.video.width,
                crop_x = self.video.crop_x,
                crop_y = self.video.crop_y
            );
        } else {
            filter_complex += &format!(
                "{}:s={}x{}[main];",
                self.video.background, self.video.width, self.video.height
            );
        }

        // Comments Overlay
        let mut previous_duration = 0.0;

        for (i, overlay) in self.overlay.overlays.iter().enumerate() {
            let index = if has_background { i + 1 } else { i };

            // Scale
            filter_complex += &format!(
                "[{}:v]scale={}-{}:-1[overlay{}];",
                index, self.video.width, self.overlay.padding, index
            );

            // Opacity
            if self.overlay.opacity < 0.99 {
                filter_complex += &format!(
                    "[overlay{}]format=argb,colorchannelmixer=aa={}[overlay{}];",
                    index, self.overlay.opacity, index
                );
            }

            // Duration
            filter_complex += &format!("[main][overlay{}]overlay=(main_w-overlay_w)/2:(main_h-overlay_h)/2:enable='between(t,{},{})'[main];", index, previous_duration, previous_duration + overlay.duration);
            previous_duration += overlay.duration;
        }

        // Progress Bar
        if self.progress_bar.enable {
            if self.progress_bar.solid {
                filter_complex += &format!(
                    "color=c=#525252:s={}x{}[solid_bar];",
                    self.video.width, self.progress_bar.height
                );
                filter_complex +=
                    "[main][solid_bar]overlay=main_w-overlay_w:main_h-overlay_h:shortest=1[main];";
            }

            filter_complex += &format!(
                "color=c={}:s={}x{}[overlay_bar];",
                self.progress_bar.color, self.video.width, self.progress_bar.height
            );
            filter_complex += &format!(
                "[main][overlay_bar]overlay=-w+(w/{})*t:H-h:shortest=1[main];",
                video_length
            );
        }

        // Audio Filter
        let audio_filter_complex = format!(
            "[{}:a]volume={}[bgm];[{}:a][bgm]amerge=inputs=2[bgm]",
            if has_background {
                self.overlay.overlays.len() + 2
            } else {
                self.overlay.overlays.len() + 1
            },
            self.music.volume,
            if has_background {
                self.overlay.overlays.len() + 1
            } else {
                self.overlay.overlays.len()
            }
        );

        // Add Filters
        args.push(&filter_complex.trim_end_matches(";"));

        if has_bgm {
            args.push("-filter_complex");
            args.push(&audio_filter_complex);
        }

        // Map Video
        args.push("-map");
        args.push("[main]");

        // Map Audio
        let audio = if has_background {
            format!("{}:a", self.overlay.overlays.len() + 1)
        } else {
            format!("{}:a", self.overlay.overlays.len())
        };

        if has_bgm {
            args.push("-map");
            args.push("[bgm]");
            args.push("-shortest");
        } else {
            args.push("-map");
            args.push(&audio);
        }

        // Streams
        if !has_background {
            args.push("-c:v");
            args.push("libx264");
        }

        if !has_bgm {
            args.push("-c:a");
            args.push("copy");
        }

        // Output
        let mut output = "export.mp4".to_owned();
        if std::path::Path::new(&output).exists() {
            for i in 0..9999 {
                output = format!("export ({}).mp4", i + 1);
                if !std::path::Path::new(&output).exists() {
                    break;
                }
            }
        }
        args.push(&output);
        args.iter().map(|x| x.to_string()).collect::<Vec<String>>()
    }

    pub fn command(&self) -> String {
        let mut ffmpeg_command = "ffmpeg".to_owned();

        for arg in self.generate() {
            if arg.contains(" ") || arg.contains(";") {
                ffmpeg_command += &format!(" \"{}\"", arg);
            } else {
                ffmpeg_command += &format!(" {}", arg);
            }
        }

        ffmpeg_command
    }

    pub fn render(&self) -> Result<()> {
        self.merge_tts()?;
        std::process::Command::new("ffmpeg")
            .args(self.generate())
            .spawn()?
            .wait()?;
        Ok(())
    }
}

pub fn duration(path: &str) -> Result<f32> {
    let output = std::process::Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            path,
        ])
        .output()?;

    Ok(String::from_utf8(output.stdout)?
        .trim_end()
        .parse::<f32>()?)
}
