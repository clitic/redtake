use anyhow::{bail, Result};
use clap::ArgEnum;
use headless_chrome::{Browser, LaunchOptionsBuilder};

use redtake::args::Commands;

fn main() -> Result<()> {
    let args = redtake::args::parse();

    match args.command {
        Commands::New(new_args) => {
            let client = reqwest::blocking::Client::new();
            let reddit_threads =
                redtake::subreddit::fetch(&client, &new_args.input, new_args.limit)?;
            let reddit_threads = reddit_threads
                .iter()
                .filter(|x| {
                    if x.over_18 {
                        if new_args.nsfw {
                            return true;
                        } else {
                            return false;
                        }
                    }

                    true
                })
                .collect::<Vec<_>>();

            let browser = Browser::new(
                LaunchOptionsBuilder::default()
                    .window_size(Some((1920, 1080)))
                    .build()
                    .unwrap(),
            )?;
            let tab = redtake::screenshot::tab(&browser, !new_args.light)?;

            for (i, thread) in reddit_threads.iter().enumerate() {
                // Directory Structure
                let mut thread_dir = if reddit_threads.len() == 1 {
                    format!("Project {} ({})", thread.id, thread.title)
                } else {
                    format!("Project {} Hot {} ({})", thread.id, i + 1, thread.title)
                };

                if std::path::Path::new(&thread_dir).exists() {
                    if new_args.skip {
                        println!("Skipped creating {}", thread.id);
                        continue;
                    }
                    std::fs::remove_dir_all(&thread_dir)?;
                }

                if std::fs::create_dir(&thread_dir).is_err() {
                    thread_dir = if reddit_threads.len() == 1 {
                        format!("Project {}", thread.id)
                    } else {
                        format!("Project {} Hot {}", thread.id, i + 1)
                    };

                    if std::path::Path::new(&thread_dir).exists() {
                        std::fs::remove_dir_all(&thread_dir)?;
                    }
                }

                std::fs::create_dir_all(format!("{}/images", thread_dir))?;
                std::fs::create_dir_all(format!("{}/audio", thread_dir))?;
                std::fs::create_dir_all(format!("{}/data", thread_dir))?;

                thread.save_json(&format!("{}/data/thread.json", thread_dir))?;

                let mut pb = kdam::tqdm!(
                    total = (new_args.max_duration * 1000.0) as usize,
                    dynamic_ncols = true
                );
                let mut video = redtake::video::Project::default();

                println!("Generating tts for thread title");
                new_args.voice.save(
                    &client,
                    &format!("{}. {}", thread.title, thread.selftext),
                    &format!("{}/audio/title.mp3", thread_dir),
                    new_args.tts_proxy,
                )?;
                let duration =
                    redtake::video::duration(&format!("{}/audio/title.mp3", thread_dir))?;
                let pb_update_factor = (duration * 500.0) as usize;
                pb.update(pb_update_factor);

                pb.write("Taking screenshot of thread title".to_owned());
                redtake::screenshot::take_title_screenshot(
                    &tab,
                    &thread,
                    &format!("{}/images/title.png", thread_dir),
                )?;
                pb.update(pb_update_factor);

                video.add_overlay("images/title.png", "audio/title.mp3", duration);

                if video.len() > new_args.max_duration {
                    pb.write(
                        "Thread content is of more duration than max length. \
                    No comments will be added"
                            .to_owned(),
                    );
                } else {
                    let comments = thread.comments(&client, new_args.max_length)?;
                    let mut comment_count = 1;

                    for comment in &comments {
                        comment.save_json(&format!(
                            "{}/data/comment_{}.json",
                            thread_dir, comment_count
                        ))?;
                        let tts_path =
                            format!("{}/audio/comment_{}.mp3", thread_dir, comment_count);
                        pb.write(format!("Generating tts for {} comment.", comment_count));
                        new_args.voice.save(
                            &client,
                            &comment.body,
                            &tts_path,
                            new_args.tts_proxy,
                        )?;
                        let duration = redtake::video::duration(&tts_path)?;
                        let pb_update_factor = (duration * 500.0) as usize;
                        pb.update(pb_update_factor);

                        if video.len() + duration > new_args.max_duration {
                            std::fs::remove_file(&tts_path)?;
                            break;
                        }

                        pb.write(format!("Taking screenshot for {} comment.", comment_count));
                        redtake::screenshot::take_comment_screenshot(
                            &tab,
                            &comment,
                            &format!("{}/images/comment_{}.png", thread_dir, comment_count),
                        )?;
                        pb.update(pb_update_factor);

                        video.add_overlay(
                            &format!("images/comment_{}.png", comment_count),
                            &format!("audio/comment_{}.mp3", comment_count),
                            duration,
                        );

                        comment_count += 1;
                    }
                }

                pb.write(format!("Saving {}/Project.toml", thread_dir));
                video.save_toml(&format!("{}/Project.toml", thread_dir))?;
                eprintln!();
            }
        }
        Commands::Render(render_args) => {
            if !std::path::Path::new("Project.toml").exists() {
                bail!("Project.toml not found in root directory.")
            }

            let video = redtake::video::Project::load_toml("Project.toml")?;
            if render_args.command {
                println!("{}", video.command());
            } else {
                video.render()?;
            }
        }

        Commands::Tts(tts_args) => {
            tts_args.voice.save(
                &reqwest::blocking::Client::new(),
                &tts_args.text,
                &tts_args.output,
                tts_args.tts_proxy,
            )?;

            if tts_args.duration {
                println!("{}", redtake::video::duration(&tts_args.output)?);
            }
        }
        Commands::Voices => {
            redtake::tts::Voices::value_variants()
                .iter()
                .for_each(|x| println!("{}", x.to_possible_value().unwrap().get_name()));
        }
    }

    Ok(())
}
