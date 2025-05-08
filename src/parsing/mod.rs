use std::fs::create_dir_all;
use log::error;
use scraper::{Html, Selector};
use serde_json::Value;
use regex::Regex;
use serde::Deserialize;
use crate::Conf;

#[derive(Debug, Clone)]
pub enum TaskType {
    PARSE,
    ATTACH
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase", tag = "task_type")]
pub enum Task {
    Parse{url: String, body: String, head: String},
    Attach{url: String, file_path: String}
}

impl Task {
    fn url(&self) -> &str {
        match self {
            Task::Parse{url, ..} => url,
            Task::Attach{url, ..} => url
        }
    }
}

pub trait Executable {
    fn execute(&self, conf: Conf) -> Result<(), String>;
}

impl Executable for Task { 
    fn execute(&self, conf: Conf) -> Result<(), String> {
        // save to filesystem
        // extract path from URL
        let path = remove_scheme_and_last_path_part_from_url(&self.url());
        if path.is_none() {
            error!("can't extract path from URL {}", self.url());
            return Err("can't extract path from URL".to_string());
        }
        let path = path.unwrap();
        // and create directory structure
        if create_dir_all(&path).is_err() {
            error!("path {} can't be created for URL {}", &path, self.url());
            return Err("path can't be created for URL".to_string());
        }
        // save
        
        // parse html
        // save links to database

        return Err("path can't be created for URL".to_string());
    }
}


pub enum DESERIALIZATION_ERROR {
    NO_TASK_TYPE,
    MISSING_FIELD,
    UNKNOWN_TASK_TYPE
}

pub struct ParsedHtml {
    assets: Vec<String>,
    url: String
}



/*
extract URLs from these HTML elements:
<iframe />
<object />
<img />
<picture />
<embed />
<object />
<link />
<script />
<audio />
<video />
<track />
<a>
 */
pub fn parse_html(html: &str, url: &str) -> ParsedHtml {
    let document = Html::parse_document(html);
    let mut parsed_html = ParsedHtml {
        url: url.to_string(),
        assets: vec![]
    };

    // a, link
    ["a", "link"].iter().for_each(|element| { 
        let selector = Selector::parse(element).unwrap();
        for element in document.select(&selector) {
            if let Some(url) = element.value().attr("href") {
                if (!parsed_html.assets.contains(&url.to_string())) {
                    parsed_html.assets.push(url.to_string());
                }
            }
        }
    });
    
    // img, iframe, audio, source
    ["img", "iframe", "audio", "source"].iter().for_each(|element| {
        let selector = Selector::parse(element).unwrap();
        for element in document.select(&selector) {
            if let Some(url) = element.value().attr("src") {
                if (!parsed_html.assets.contains(&url.to_string())) {
                    parsed_html.assets.push(url.to_string());
                }
            }
        }
    });

    parsed_html
}

pub fn remove_scheme_and_last_path_part_from_url(url: &str) -> Option<String> {
    let parts: Vec<&str> = url.split("/").into_iter().collect::<Vec<&str>>();
    let re = Regex::new(r"^https?://(.+)/([^/]+)$").unwrap();
    if let Some(caps) = re.captures(&url) {
        Some(caps.get(1).unwrap().as_str().to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_html_a() {
        let html = r#"
    <ul>
        <a href="link1"></li>
        <a href="link2">Bar</li>
        <a href="link1">Baz</li>
    </ul>
"#;
        let parsed_html = parse_html(html, "url");
        assert_eq!(parsed_html.assets.len(), 2);
        assert_eq!(parsed_html.url, "url".to_string());
        assert_eq!(parsed_html.assets[0], "link1".to_string());
        assert_eq!(parsed_html.assets[1], "link2".to_string());
    }

    #[test]
    fn test_parse_html_a_img() {
        let html = r#"
    <ul>
        <a href="link1"></li>
        <a href="link2">Bar</li>
        <a href="link1">Baz</li>
        <img src="link1">
        <img src="link3">
    </ul>
"#;
        let parsed_html = parse_html(html, "url");
        assert_eq!(parsed_html.assets.len(), 3);
        assert_eq!(parsed_html.url, "url".to_string());
        assert_eq!(parsed_html.assets[0], "link1".to_string());
        assert_eq!(parsed_html.assets[1], "link2".to_string());
        assert_eq!(parsed_html.assets[2], "link3".to_string());
    }

    #[test]
    fn test_remove_scheme_and_last_path_part_from_url() {
        let url = "https://www.example.com/foo/bar/baz";
        let expected = "www.example.com/foo/bar";
        assert_eq!(remove_scheme_and_last_path_part_from_url(url), Some(expected.to_string()));
    }
    
    #[test]
    fn test_deserialize_task_parse() {
        let json = r#"{"task_type": "parse", "url": "https://www.example.com/foo/bar/baz", "body": "body", "head": "head"}"#;
        let task: Task = serde_json::from_str(json).unwrap();
        assert_eq!(task.url(), "https://www.example.com/foo/bar/baz");
        
        if let Task::Parse{url, body, head} = task {
            assert_eq!(body, "body");
            assert_eq!(head, "head");
        } else {
            panic!("task is not Parse");
        }
    }

    #[test]
    fn test_deserialize_task_attach() {
        let json = r#"{"task_type": "attach", "url": "https://www.example.com/foo/bar/baz", "file_path":"/path/to/file"}"#;
        let task: Task = serde_json::from_str(json).unwrap();
        assert_eq!(task.url(), "https://www.example.com/foo/bar/baz");

        if let Task::Attach{url, file_path} = task {
            assert_eq!(file_path, "/path/to/file");
        } else {
            panic!("task is not Attach");
        }
    }
}