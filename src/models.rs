//! Models for chablo
use std::path::PathBuf;

use askama::Template;
use chrono::{Datelike, Local, NaiveDate};

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
