//! Parse html to markdown
use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use pulldown_cmark::{html, Options, Parser};

use crate::errors::ChabloError;
use crate::models::{created_datetime, Article, HtmlBody};

pub fn parse(path: PathBuf) -> Result<Article, ChabloError> {
    // Extract the content of a markdown file
    let content = fs::read_to_string(&path)?;
    let created_time = created_datetime(&path).unwrap();
    let title = extract_title(&content)?;
    let body = extract_body(&content)?;
    let html_body = convert_md_to_html(body)?;
    let id = format!("{}_{}", &created_time, title);
    let path = format!("{}{}", &title, ".html");

    let article = Article {
        id,
        title: title.to_string(),
        body: html_body,
        date: created_time,
        path,
    };

    Ok(article)
}

/// Convert markdown to html body
fn convert_md_to_html(md: &str) -> Result<HtmlBody, ChabloError> {
    // Set up options and parser. Strikethroughs are not part of the CommonMark standard
    // and we therefore must enable it explicitly.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(md, options);

    // Write to String buffer.
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    let content = HtmlBody(html_output);

    Ok(content)
}

/// Extract title from raw-content
fn extract_title(content: &str) -> Result<String, ChabloError> {
    let parts: Vec<&str> = content.split("---").collect();

    if parts.len() < 2 {
        return Ok("タイトル無し".to_string());
    }
    if let Some(title_part) = parts[1].split("title:").nth(1) {
        let title = title_part.trim().replace('/', "");
        Ok(title)
    } else {
        Err(ChabloError::ExtractTitleError)
    }
}

/// Extract title from raw-content
fn extract_body(content: &str) -> Result<&str, ChabloError> {
    let v: Vec<&str> = content.split("---").collect();

    match v.len() {
        0 => Ok("本文はありません。"),
        1 => match v[0].is_empty() {
            true => Ok("本文はありません。"),
            false => Ok(v[0]),
        },
        2 => Ok(v[1]),
        3 => match v[2].is_empty() {
            true => Ok("本文はありません。"),
            false => Ok(v[2]),
        },
        _ => Err(ChabloError::MatchError),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::HtmlBody;
    use pretty_assertions::assert_eq;
    use std::path::PathBuf;

    #[test]
    fn test_parse_ok() {
        let path = PathBuf::from("./tests/fixtures/2050_05_30.md");
        let result = parse(path);

        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_with_path() {
        let path = PathBuf::from("./tests/fixtures/2050_05_30.md");
        let created_time = created_datetime(&path).unwrap();
        let result = parse(path).unwrap();

        let html = "<p>本文はありません。</p>\n".to_string();
        let html_body = HtmlBody(html);
        let path = format!("{}{}", "タイトル無し", ".html");

        let article = Article {
            id: "2050-05-30_タイトル無し".to_string(),
            title: "タイトル無し".to_string(),
            body: html_body,
            date: created_time,
            path,
        };

        assert_eq!(result, article);
    }

    #[test]
    fn test_convert_md_to_html_ok() {
        let content = "Hello world, this is a ~~complicated~~ *very simple* example.";
        let result = convert_md_to_html(content).unwrap();

        let html =
            "<p>Hello world, this is a <del>complicated</del> <em>very simple</em> example.</p>\n"
                .to_string();

        let expected_html = HtmlBody(html);

        assert_eq!(result.0, expected_html.0);
    }

    #[test]
    fn test_extract_title_ok() {
        let content: &str = "---\ntitle: dummy title\n---\n\nThis is a dummy body.\n\n\n改行されているか。\n\n\noooooooooooooooo
ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo
oooooooooooooooooooooooooooooo\n\n\noooooooooooo\n";

        let result = extract_title(content).unwrap();
        let expected_title = "dummy title";

        assert_eq!(result, expected_title);
    }

    #[test]
    fn test_extract_title_with_no_title() {
        let content: &str = "Hello world, this is a ~~complicated~~ *very simple* example.";

        let result = extract_title(content).unwrap();
        let expected_title = "タイトル無し";

        assert_eq!(result, expected_title);
    }

    #[test]
    fn test_extract_title_ok_with_ja() {
        let content: &str = "---\ntitle: ブログシステムを作っていた\n---\n\nThis is a dummy body.\n\n\n改行されているか。\n\n\noooooooooooooooo
ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo
oooooooooooooooooooooooooooooo\n\n\noooooooooooo\n";

        let result = extract_title(content).unwrap();
        let expected_title = "ブログシステムを作っていた";

        assert_eq!(result, expected_title);
    }

    #[test]
    fn test_extract_body_ok() {
        let content: &str = "---\ntitle: dummy title\n---This is a dummy body.";

        let result = extract_body(content).unwrap();
        let expected_body = "This is a dummy body.";

        assert_eq!(result, expected_body);
    }

    #[test]
    fn test_extract_body_with_different_format() {
        let content: &str = "Hello world, this is a ~~complicated~~ *very simple* example.";

        let result = extract_body(content).unwrap();
        let expected_body = "Hello world, this is a ~~complicated~~ *very simple* example.";

        assert_eq!(result, expected_body);
    }

    #[test]
    fn test_extract_body_with_no_body() {
        let content: &str = "---\ntitle: dummy title\n---";

        let result = extract_body(content).unwrap();
        let expected_body = "本文はありません。";

        assert_eq!(result, expected_body);
    }

    #[test]
    fn test_extract_body_with_empty_contents() {
        let content: &str = "";

        let result = extract_body(content).unwrap();
        let expected_body = "本文はありません。";

        assert_eq!(result, expected_body);
    }

    #[test]
    fn test_extract_body_with_no_title() {
        let content: &str = "Hello world!!!!!!!!";

        let result = extract_body(content).unwrap();
        let expected_body = "Hello world!!!!!!!!";

        assert_eq!(result, expected_body);
    }
}
