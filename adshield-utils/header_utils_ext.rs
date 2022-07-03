use http::header::{CONTENT_TYPE, COOKIE, HOST, USER_AGENT};
use http::HeaderMap;

use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;

static REGEX_CHECK_BOT: Lazy<Regex> =
    Lazy::new(|| Regex::new(include_str!("check_bots.regex").trim()).unwrap());

static REGEX_COOKIE: Lazy<Regex> = Lazy::new(|| Regex::new("(.*?)=((.*?)[;,]|.*$)").unwrap());

pub trait HeaderUtilsExt {
    fn hostname(&self) -> Option<&str>;
    fn is_healthcheck(&self) -> bool;
    fn is_bot(&self) -> bool;
    fn is_text_html(&self) -> bool;
    fn get_cookies(&self) -> HashMap<String, String>;
}

impl HeaderUtilsExt for HeaderMap {
    fn hostname(&self) -> Option<&str> {
        if let Some(v) = self.get(HOST) {
            return v.to_str().ok();
        }

        None
    }

    fn is_healthcheck(&self) -> bool {
        if let Some(Ok(v)) = self.get(USER_AGENT).map(|v| v.to_str()) {
            return v.contains("HAAgent");
        }

        false
    }

    fn is_bot(&self) -> bool {
        if let Some(Ok(v)) = self.get(USER_AGENT).map(|v| v.to_str()) {
            return REGEX_CHECK_BOT.is_match(v);
        }

        false
    }

    fn is_text_html(&self) -> bool {
        if let Some(Ok(v)) = self.get(CONTENT_TYPE).map(|v| v.to_str()) {
            return v.contains("text/html");
        }

        false
    }

    fn get_cookies(&self) -> HashMap<String, String> {
        let cookies = match self.get(COOKIE) {
            None => return HashMap::new(),
            Some(cookie) => cookie,
        };

        let mut result = HashMap::new();
        for cap in REGEX_COOKIE.captures_iter(cookies.to_str().unwrap()) {
            if let Some(cap_key) = cap.get(1) {
                if let Some(val) = cap.get(3) {
                    result.insert(
                        cap_key.as_str().trim().to_string(),
                        val.as_str().to_string(),
                    );

                    continue;
                }

                if let Some(val) = cap.get(2) {
                    result.insert(
                        cap_key.as_str().trim().trim_end_matches(';').to_string(),
                        val.as_str().to_string(),
                    );

                    continue;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bots1() {
        assert!(REGEX_CHECK_BOT.is_match("Bingbot"));
        assert!(REGEX_CHECK_BOT.is_match("GoogleBot"));
        assert!(REGEX_CHECK_BOT
            .is_match("Mozilla/5.0 (compatible; bingbot/2.0; +http://www.bing.com/bingbot.htm)"));
    }
}
