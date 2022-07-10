use anyhow::{Context, Result};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RedditThreadComment {
    pub body: String,
    id: String,
    permalink: String,
}

impl RedditThreadComment {
    pub fn url(&self) -> String {
        format!("https://reddit.com{}", self.permalink)
    }

    pub fn css_selector(&self) -> String {
        // "#t1_{} > Comment.t1_{}"
        format!("div.t1_{}:nth-child(2)", self.id)
    }

    pub fn save_json(&self, path: &str) -> Result<()> {
        serde_json::to_writer_pretty(std::fs::File::create(path)?, self)?;
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct RedditThread {
    pub id: String,
    pub over_18: bool,
    // num_comments: u64,
    permalink: String,
    // score: i64,
    pub selftext: String,
    pub title: String,
    // upvote_ratio: f64,
}

impl RedditThread {
    pub fn new(child: &serde_json::Value) -> Option<Self> {
        if child["data"]["stickied"].as_bool().unwrap() {
            return None;
        }

        Some(Self {
            id: child["data"]["id"].as_str().unwrap().to_owned(),
            over_18: child["data"]["over_18"].as_bool().unwrap(),
            // num_comments: child["data"]["num_comments"].as_u64().unwrap(),
            permalink: child["data"]["permalink"].as_str().unwrap().to_owned(),
            // score: child["data"]["score"].as_i64().unwrap(),
            selftext: child["data"]["selftext"].as_str().unwrap().to_owned(),
            title: child["data"]["title"].as_str().unwrap().to_owned(),
            // upvote_ratio: child["data"]["upvote_ratio"].as_f64().unwrap(),
        })
    }

    pub fn url(&self) -> String {
        format!("https://reddit.com{}", self.permalink)
    }

    pub fn save_json(&self, path: &str) -> Result<()> {
        serde_json::to_writer_pretty(std::fs::File::create(path)?, self)?;
        Ok(())
    }

    pub fn comments(
        &self,
        client: &reqwest::blocking::Client,
        max_length: usize,
    ) -> Result<Vec<RedditThreadComment>> {
        let mut comments = vec![];
        let res = client
            .get(&format!(
                "https://reddit.com{}.json?depth=0",
                self.permalink
            ))
            .send()?
            .json::<serde_json::Value>()?;

        for child in res.as_array().unwrap()[1]["data"]["children"]
            .as_array()
            .unwrap()
        {
            if child["kind"].as_str().unwrap() == "more" {
                continue;
            }

            let body = child["data"]["body"].as_str().unwrap();
            let id = child["data"]["id"].as_str().unwrap();
            let permalink = child["data"]["permalink"].as_str().unwrap();

            if child["data"]["stickied"].as_bool().unwrap()
                || body.contains("[removed]")
                || body.contains("[deleted]")
                || body.len() > max_length
            {
                continue;
            }

            comments.push(RedditThreadComment {
                body: body.to_owned(),
                id: id.to_owned(),
                permalink: permalink.to_owned(),
            });
        }

        Ok(comments)
    }
}

pub fn fetch(client: &reqwest::blocking::Client, url: &str, limit: u16) -> Result<Vec<RedditThread>> {
    let re = regex::Regex::new(r"r/\w*").unwrap();
    let subreddit_prefixed = re
        .captures(url)
        .context("No subreddit is provided.")?
        .get(0)
        .unwrap()
        .as_str();

    // Check whether the given url is thread of that subreddit.
    // (?<=comments\/)\w*
    // error: look-around, including look-ahead and look-behind, is not supported
    let re = regex::Regex::new(r"comments/\w*").unwrap();

    if let Some(thread_id) = re.captures(url) {
        let thread_url = format!(
            "https://reddit.com/{}/{}/.json?limit=0&depth=0",
            subreddit_prefixed,
            thread_id.get(0).unwrap().as_str()
        );

        let res = client
            .get(&thread_url)
            .send()?
            .json::<serde_json::Value>()?;

        if let Some(thread) = RedditThread::new(
            &res.as_array().unwrap()[0]["data"]["children"]
                .as_array()
                .unwrap()[0],
        ) {
            return Ok(vec![thread]);
        }
    } else {
        let hot_url = format!(
            "https://reddit.com/{}/hot.json?limit={}",
            subreddit_prefixed,
            limit,
        );

        let res = client.get(&hot_url).send()?.json::<serde_json::Value>()?;
        let mut hot_threads = vec![];

        for child in res["data"]["children"].as_array().unwrap() {
            if let Some(thread) = RedditThread::new(&child) {
                hot_threads.push(thread);
            }
        }

        return Ok(hot_threads);
    }

    Ok(vec![])
}

// let res = client
//     .get("https://www.reddit.com/r/rust/about.json")
//     .send()?
//     .json::<serde_json::Value>()?;

// println!(
//     "Using subreddit: {}\nTitle:{}\nDescription: {}",
//     res["data"]["display_name_prefixed"].as_str().unwrap(),
//     res["data"]["title"].as_str().unwrap(),
//     res["data"]["public_description"].as_str().unwrap()
// );
