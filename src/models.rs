//! Models for chablo
use std::path::{Path, PathBuf};

use askama::Template;
use chrono::{Datelike, Local, NaiveDate};
use regex::Regex;

pub const BLOG_TITLE: &str = "blog.chansuke.info";
pub const DESCRIPTION: &str = "日記、メモ、個人開発、生活の記録";

/// Markdown file path
#[derive(Debug)]
pub struct MarkdownPath(pub PathBuf);

/// Converted HTML content from Markdown
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HtmlBody(pub String);

impl std::fmt::Display for HtmlBody {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

pub fn created_datetime(path: &Path) -> Option<NaiveDate> {
    let path_str = path.to_string_lossy();

    // Convert extracted time
    extract_time(&path_str).map(|date| NaiveDate::from_ymd(date[0], date[1] as u32, date[2] as u32))
}

fn extract_time(path_str: &str) -> Option<Vec<i32>> {
    let re_str = r#"\d{1,}"#;
    let re = Regex::new(re_str).unwrap();
    // Extract year, month, date
    let year_month_day = re
        .captures_iter(path_str)
        .map(|cap| cap.get(0).unwrap().as_str().parse::<i32>().unwrap())
        .collect();

    Some(year_month_day)
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
        let result = extract_time(before).unwrap();
        let expected_result = vec![2050, 0o5, 30];

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_created_datetime() {
        let before = "tests/fixtures/2050/05/30.md";
        let path = PathBuf::from(before);
        let result = created_datetime(&path).unwrap();
        let expected_result = NaiveDate::from_ymd(2050, 0o5, 30);

        assert_eq!(result, expected_result);
    }
}
