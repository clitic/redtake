use std::io::Write;
use std::sync::Arc;

use anyhow::Result;
use headless_chrome::protocol::cdp::{Network, Page};
use headless_chrome::{Browser, Tab};

pub fn tab(browser: &Browser, dark: bool) -> Result<Arc<Tab>> {
    let tab = browser.wait_for_initial_tab()?;
    tab.set_default_timeout(std::time::Duration::from_secs(60 * 3));

    // Cookie Value
    // Inspect > Application > Cookies > https://reddit.com
    if dark {
        let dark_theme = Network::CookieParam {
            name: "USER".to_owned(),
            value: "eyJwcmVmcyI6eyJ0b3BDb250ZW50RGlzbWlzc2FsVGltZSI6MCwiZ2xvYmFsVGhlbWUiOiJSRURESVQiLCJuaWdodG1vZGUiOnRydWUsImNvbGxhcHNlZFRyYXlTZWN0aW9ucyI6eyJmYXZvcml0ZXMiOmZhbHNlLCJtdWx0aXMiOmZhbHNlLCJtb2RlcmF0aW5nIjpmYWxzZSwic3Vic2NyaXB0aW9ucyI6ZmFsc2UsInByb2ZpbGVzIjpmYWxzZX0sInRvcENvbnRlbnRUaW1lc0Rpc21pc3NlZCI6MH19".to_owned(),
            url: None,
            domain: Some(".reddit.com".to_owned()),
            path: Some("/".to_owned()),
            secure: None,
            http_only: None,
            same_site: None,
            expires: None,
            priority: None,
            same_party: None,
            source_scheme: None,
            source_port: None,
            partition_key: None,
        };
        tab.set_cookies(vec![dark_theme])?;
    }

    Ok(tab)
}

pub fn take_title_screenshot(tab: &Arc<Tab>, reddit_thread: &crate::subreddit::RedditThread, path: &str) -> Result<()> {
    tab.navigate_to(&reddit_thread.url())?;
    tab.wait_until_navigated()?;

    if reddit_thread.over_18 {
        if let Ok(content_gate) = tab.wait_for_element("[data-testid=\"content-gate\"] button") {
          content_gate.click()?;
           tab.wait_for_element("[data-click-id=\"text\"] button")?
             .click()?;
        }
    }

    std::fs::File::create(path)?.write(
        &tab.wait_for_element("[data-test-id=\"post-content\"]")?
            .capture_screenshot(Page::CaptureScreenshotFormatOption::Png)?,
    )?;

    Ok(())
}

pub fn take_comment_screenshot(
    tab: &Arc<Tab>,
    comment: &crate::subreddit::RedditThreadComment,
    path: &str,
) -> Result<()> {
    tab.navigate_to(&comment.url())?;
    tab.wait_until_navigated()?;

    let viewport = tab
        .wait_for_element(&comment.css_selector())?
        .get_box_model()?
        .border_viewport();

    let image = tab.capture_screenshot(
        Page::CaptureScreenshotFormatOption::Png,
        None,
        Some(viewport),
        true,
    )?;

    std::fs::File::create(path)?.write(&image)?;
    Ok(())
}
