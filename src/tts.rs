use std::io::Write;

use anyhow::{bail, Result};
use clap::ArgEnum;

#[derive(Debug, Clone, ArgEnum)]
pub enum Voices {
    DeDETiktokFemale,
    DeDETiktokMale,
    EnAUPollyNicole,
    EnAUPollyRussell,
    EnAUTiktokFemale,
    EnAUTiktokMale,
    EnINPollyRaveena,
    EnUKPollyAmy,
    EnUKPollyBrian,
    EnUKPollyEmma,
    EnUKTiktokMale1,
    EnUKTiktokMale2,
    EnUSPollyIvy,
    EnUSPollyJoanna,
    EnUSPollyJoey,
    EnUSPollyJustin,
    EnUSPollyKendra,
    EnUSPollyKimberly,
    EnUSPollyMatthew,
    EnUSPollySalli,
    EnUSTiktokC3PO,
    EnUSTiktokChewbacca,
    EnUSTiktokFemale1,
    EnUSTiktokFemale2,
    EnUSTiktokGhostface,
    EnUSTiktokMale1,
    EnUSTiktokMale2,
    EnUSTiktokMale3,
    EnUSTiktokRocket,
    EnUSTiktokStitch,
    EnUSTiktokStormtrooper,
    EnUSTiktokUnknown,
    EnWelshPollyGeraint,
    EsESTiktokMale,
    EsMXTiktokMale,
    FrFRTiktokMale1,
    FrFRTiktokMale2,
    IdIdTiktokFemale,
    JpJPTiktokFemale1,
    JpJPTiktokFemale2,
    JpJPTiktokFemale3,
    JpJPTiktokMale,
    KrKRTiktokFemale,
    KrKRTiktokMale1,
    KrKRTiktokMale2,
    PtBRTiktokFemale1,
    PtBRTiktokFemale2,
    PtBRTiktokFemale3,
    PtBRTiktokMale,
}

impl Voices {
    fn code(&self) -> String {
        let voice = match self {
            Voices::DeDETiktokFemale => "de_001",
            Voices::DeDETiktokMale => "de_002",
            Voices::EnAUPollyNicole => "Nicole",
            Voices::EnAUPollyRussell => "Russell",
            Voices::EnAUTiktokFemale => "en_au_001",
            Voices::EnAUTiktokMale => "en_au_002",
            Voices::EnINPollyRaveena => "Raveena",
            Voices::EnUKPollyAmy => "Amy",
            Voices::EnUKPollyBrian => "Brian",
            Voices::EnUKPollyEmma => "Emma",
            Voices::EnUKTiktokMale1 => "en_uk_001",
            Voices::EnUKTiktokMale2 => "en_uk_003",
            Voices::EnUSPollyIvy => "Ivy",
            Voices::EnUSPollyJoanna => "Joanna",
            Voices::EnUSPollyJoey => "Joey",
            Voices::EnUSPollyJustin => "Justin",
            Voices::EnUSPollyKendra => "Kendra",
            Voices::EnUSPollyKimberly => "Kimberly",
            Voices::EnUSPollyMatthew => "Matthew",
            Voices::EnUSPollySalli => "Salli",
            Voices::EnUSTiktokC3PO => "en_us_c3po",
            Voices::EnUSTiktokChewbacca => "en_us_chewbacca",
            Voices::EnUSTiktokFemale1 => "en_us_001",
            Voices::EnUSTiktokFemale2 => "en_us_002",
            Voices::EnUSTiktokGhostface => "en_us_ghostface",
            Voices::EnUSTiktokMale1 => "en_us_006",
            Voices::EnUSTiktokMale2 => "en_us_007",
            Voices::EnUSTiktokMale3 => "en_us_009",
            Voices::EnUSTiktokRocket => "en_us_rocket",
            Voices::EnUSTiktokStitch => "en_us_stitch",
            Voices::EnUSTiktokStormtrooper => "en_us_stormtrooper",
            Voices::EnUSTiktokUnknown => "en_us_010",
            Voices::EnWelshPollyGeraint => "Geraint",
            Voices::EsESTiktokMale => "es_002",
            Voices::EsMXTiktokMale => "es_mx_002",
            Voices::FrFRTiktokMale1 => "fr_001",
            Voices::FrFRTiktokMale2 => "fr_002",
            Voices::IdIdTiktokFemale => "id_001",
            Voices::JpJPTiktokFemale1 => "jp_001",
            Voices::JpJPTiktokFemale2 => "jp_003",
            Voices::JpJPTiktokFemale3 => "jp_005",
            Voices::JpJPTiktokMale => "jp_006",
            Voices::KrKRTiktokFemale => "kr_003",
            Voices::KrKRTiktokMale1 => "kr_002",
            Voices::KrKRTiktokMale2 => "kr_004",
            Voices::PtBRTiktokFemale1 => "br_001",
            Voices::PtBRTiktokFemale2 => "br_003",
            Voices::PtBRTiktokFemale3 => "br_004",
            Voices::PtBRTiktokMale => "br_005",
        };
        voice.to_owned()
    }

    pub fn save(
        &self,
        client: &reqwest::blocking::Client,
        text: &str,
        path: &str,
        proxy: bool,
    ) -> Result<()> {
        let service = self.to_possible_value().unwrap().get_name();
        let mut texts = vec![text.to_owned()];
        let mut file = std::fs::File::create(path)?;

        if service.contains("tiktok") {
            if text.len() >= 300 {
                texts.clear();
                texts = text_chunks(&text, 299);
            }

            let url = if proxy {
                "https://warp-co.rs/https://api16-normal-useast5.us.tiktokv.com/media/api/text/speech/invoke/"
            } else {
                "https://api16-normal-useast5.us.tiktokv.com/media/api/text/speech/invoke/"
            };

            for text_chunk in texts {
                let mut res = client.post(url).query(&[
                    ("text_speaker", self.code().as_str()),
                    ("req_text", text_chunk.as_str()),
                    ("speaker_map_type", "0"),
                ]);

                if proxy {
                    res = res.header("Origin", "https://api16-normal-useast5.us.tiktokv.com")
                }

                let res = res.send()?.json::<serde_json::Value>()?;

                let data = res["data"]["v_str"].as_str().unwrap();
                if data != "" {
                    let mp3 = base64::decode(data)?;
                    file.write(&mp3)?;
                } else {
                    bail!(res["message"].as_str().unwrap().to_owned())
                }
            }
        } else if service.contains("polly") {
            if text.len() >= 500 {
                texts.clear();
                texts = text_chunks(&text, 499);
            }

            let url = if proxy {
                "https://warp-co.rs/https://streamlabs.com/polly/speak"
            } else {
                "https://streamlabs.com/polly/speak"
            };

            for text_chunk in texts {
                let mut res = client.post(url).query(&[
                    ("voice", self.code().as_str()),
                    ("text", text_chunk.as_str()),
                    ("service", "polly"),
                ]);

                if proxy {
                    res = res.header("Origin", "https://streamlabs.com/polly/speak")
                }

                let res = res.send()?.json::<serde_json::Value>()?;

                if res["error"].is_null() {
                    let success = res["success"].as_bool().unwrap();
                    let speak_url = res["speak_url"].as_str().unwrap();

                    if success {
                        let mp3 = client.get(speak_url).send()?.bytes()?;
                        file.write(&mp3)?;
                    } else {
                        bail!("Couldn't perform tts.")
                    }
                } else {
                    bail!(res["error"].as_str().unwrap().to_owned())
                }
            }
        }

        Ok(())
    }
}

fn text_chunks(text: &str, size: usize) -> Vec<String> {
    let mut chunks = vec![];
    let length = text.len();

    for i in (0..length).step_by(size) {
        let mut end = i + size;

        while end > length {
            end -= 1;
        }

        chunks.push(text[i..end].to_owned());
    }

    chunks
}

// TODO
// import re
// def sanitize_text(text):
//     """
//     Sanitizes the text for tts.
//        What gets removed:
//     - following characters`^_~@!&;#:-%“”‘"%*/{}[]()\|<>?=+`
//     - any http or https links
//     """

//     # remove any urls from the text
//     regex_urls = r"((http|https)\:\/\/)?[a-zA-Z0-9\.\/\?\:@\-_=#]+\.([a-zA-Z]){2,6}([a-zA-Z0-9\.\&\/\?\:@\-_=#])*"

//     result = re.sub(regex_urls, " ", text)

//     # note: not removing apostrophes
//     regex_expr = r"\s['|’]|['|’]\s|[\^_~@!&;#:\-%“”‘\"%\*/{}\[\]\(\)\\|<>=+]"
//     result = re.sub(regex_expr, " ", result)

//     # remove extra whitespace
//     return " ".join(result.split())
