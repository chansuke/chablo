//! Models for chablo
use std::path::PathBuf;

use askama::Template;
use chrono::{Datelike, Local, NaiveDate};
use regex::Regex;

pub const BLOG_TITLE: &str = "chansuke.net";
pub const SUB_TITLE: &str = "Some notable things about my life";
pub const DESCRIPTION: &str = "This is a blog";

/// Markdown file path
#[derive(Debug)]
pub struct MarkdownPath(pub PathBuf);

/// Converted HTML content from Markdown
#[derive(Debug)]
pub struct HtmlBody(pub String);

impl std::fmt::Display for HtmlBody {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.0)
    }
}

pub struct Article {
    pub id: String,
    pub title: String,
    pub body: HtmlBody,
    pub date: NaiveDate,
    pub path: String,
}

pub struct TopPage {
    pub articles: Vec<Article>,
}

pub fn curent_datetime() -> NaiveDate {
    let local_time = Local::now().naive_local();
    let year = local_time.date().year();
    let month = local_time.date().month();
    let day = local_time.date().day();

    NaiveDate::from_ymd(year, month, day)
}

pub fn created_datetime(path: PathBuf) -> Option<NaiveDate> {
    let path_str = path.into_os_string().into_string().unwrap_or_default();

    if let Some(date) = parse_time(&path_str) {
        Some(NaiveDate::from_ymd(date[0], date[1] as u32, date[2] as u32))
    } else {
        None
    }
}

fn parse_time(path_str: &str) -> Option<Vec<i32>> {
    let re_str = r#"\d{1,}"#;
    let re = Regex::new(re_str).unwrap();
    // Extract year, month, date
    let year = re
        .captures_iter(path_str)
        .map(|cap| cap.get(0).unwrap().as_str().parse::<i32>().unwrap())
        .collect();

    Some(year)
}

#[derive(Template)]
#[template(path = "article.html")]
pub(crate) struct ArticleTemplate<'a> {
    pub title: &'a str,
    pub body: HtmlBody,
    pub date: NaiveDate,
    pub description: &'a str,
}

#[derive(Template)]
#[template(path = "toppage.html")]
pub(crate) struct TopPageTemplate<'a> {
    pub title: &'a str,
    pub articles: Vec<Article>,
    pub description: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_year() {
        let before = "tests/fixtures/2050_05_30.md";
        let result = parse_time(before).unwrap();
        let expected_result = vec![2050, 05, 30];

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_created_datetime() {
        let before = "tests/fixtures/2050_05_30.md";
        let path = PathBuf::from(before);
        let result = created_datetime(path).unwrap();
        let expected_result = NaiveDate::from_ymd(2050, 05, 30);

        assert_eq!(result, expected_result);
    }
}
