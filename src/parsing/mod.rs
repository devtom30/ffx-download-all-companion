use scraper::{Html, Selector};
use serde_json::Value;

#[derive(Debug, Clone)]
pub enum TaskType {
    PARSE,
    ATTACH
}

#[derive(Debug, Clone)]
pub struct Task {
    task_type: TaskType,
    url: String
}

impl TryFrom<Option<&str>> for TaskType {
    type Error = DESERIALIZATION_ERROR;
    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        match value {
            Some("parse") => Ok(TaskType::PARSE),
            Some("attach") => Ok(TaskType::ATTACH),
            _ => Err(DESERIALIZATION_ERROR::UNKNOWN_TASK_TYPE)
        }
    }
}



pub enum DESERIALIZATION_ERROR {
    NO_TASK_TYPE,
    MISSING_FIELD,
    UNKNOWN_TASK_TYPE
}
impl TryFrom<&Value> for Task {
    type Error = DESERIALIZATION_ERROR;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if let Some(task_type) = value.get("task_type") {
            if let Ok(task_type) = TaskType::try_from(task_type.as_str()) {
                if let Some(url) = value.get("url") {
                    Ok(Self {
                        task_type: task_type,
                        url: url.to_string()
                    })
                } else {
                    Err(DESERIALIZATION_ERROR::MISSING_FIELD)
                }
            } else {
                Err(DESERIALIZATION_ERROR::UNKNOWN_TASK_TYPE)
            }
        } else {
            return Err(DESERIALIZATION_ERROR::NO_TASK_TYPE);
        }
    }
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
    ["img", "ifram", "audio", "source"].iter().for_each(|element| {
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
}