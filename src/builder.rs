//! Build static files
use std::path::PathBuf;

use anyhow::Result;
use glob::glob;

use crate::models::{Article, TopPage, UpdatedTitle};
use crate::parser::parse;
use crate::writer::write;
use crate::Generator;

use crate::errors::ChabloError;

pub fn build() -> Result<(), ChabloError> {
    let path = "diary/**/*.md";

    // Prepare articles to build static website
    let articles = collect_articles(path)?;
    build_articles(articles.clone())?;
    let toppage = TopPage { articles };
    let template = toppage.generate()?;

    let path = "public/index.html".to_string();
    write(&template, &path)?;

    Ok(())
}

// Build static htmls of an articles
pub fn build_articles(articles: Vec<Article>) -> Result<(), ChabloError> {
    for article in articles {
        let path = if article.title.contains('/') {
            let removed_title = remove_slashes(&article.title);
            "public/".to_string() + &removed_title.0 + ".html"
        } else {
            "public/".to_string() + &article.title + ".html"
        };
        let template = article.generate()?;

        write(&template, &path)?;
    }

    Ok(())
}

fn collect_articles(path: &str) -> Result<Vec<Article>, ChabloError> {
    let mut paths: Vec<PathBuf> = collect_paths(path)?;
    paths.reverse();
    let mut articles: Vec<Article> = vec![];

    for path in paths {
        let article = parse(path)?;
        articles.push(article);
    }

    Ok(articles)
}

fn collect_paths(path: &str) -> Result<Vec<PathBuf>, ChabloError> {
    let mut paths: Vec<PathBuf> = vec![];

    for entry in glob(path)? {
        match entry {
            Ok(path) => paths.push(path),
            Err(e) => eprintln!("Failed to execute event {e:?}"),
        }
    }

    Ok(paths)
}

fn remove_slashes(title: &str) -> UpdatedTitle {
    UpdatedTitle(title.replace('/', ""))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_build_ok() {
        let result = build();

        assert!(result.is_ok());
    }

    #[test]
    fn test_articles_ok() {
        let path = "tests/fixtures/2050_05_30.md";
        let articles = collect_articles(path).unwrap();
        let result = build_articles(articles);

        assert!(result.is_ok());
    }

    #[test]
    fn test_collect_article_ok() {
        let path = "tests/fixtures/2050_05_30.md";
        let result = collect_articles(path);

        assert!(result.is_ok());
    }

    #[test]
    fn test_collect_article_with_right_pattern() {
        let path = "tests/**/*.md";
        let result = collect_paths(path).unwrap();

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_collect_article_with_wrong_pattern() {
        let path = "/article/**/*.html";
        let result = collect_paths(path).unwrap();
        let expected_result: Vec<PathBuf> = vec![];

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_collect_paths_order() {
        let path = "tests/**/*.md";
        let result = collect_paths(path);

        assert!(result.is_ok());
    }

    #[test]
    fn test_remove_slashes() {
        let title = "2023/01/01";
        let result = remove_slashes(title);

        assert_eq!(result, UpdatedTitle("20230101".to_owned()));
    }
}
