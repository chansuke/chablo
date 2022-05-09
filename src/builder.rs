//! Build static files
use std::path::PathBuf;

use anyhow::Result;
use glob::glob;

use crate::models::{Article, TopPage};
use crate::parser::parse;
use crate::writer::write;
use crate::Generator;

use crate::errors::ChabloError;

pub fn build() -> Result<(), ChabloError> {
    build_articles()?;

    let articles = collect_articles()?;
    let toppage = TopPage { articles };
    let template = toppage.generate()?;

    let path = "public/index.html".to_string();
    write(&template, &path)?;

    Ok(())
}

pub fn build_articles() -> Result<(), ChabloError> {
    let articles = collect_articles()?;

    for article in articles {
        let path = "public/".to_string() + &article.title + ".html";
        let template = article.generate()?;
        write(&template, &path)?;
    }

    Ok(())
}

fn collect_articles() -> Result<Vec<Article>, ChabloError> {
    // Collect articles
    let path = "diary/*.md";
    let mut paths: Vec<PathBuf> = collect_paths(path)?;
    paths.reverse();
    let mut articles: Vec<Article> = vec![];

    for path in paths {
        let article = parse(path)?;
        articles.push(article);
    }

    Ok(articles)
}

// オーダーの変更
fn collect_paths(path: &str) -> Result<Vec<PathBuf>, ChabloError> {
    let mut paths: Vec<PathBuf> = vec![];

    for entry in glob(path)? {
        match entry {
            Ok(path) => paths.push(path),
            Err(e) => eprintln!("Failed to execute event {:?}", e),
        }
    }

    Ok(paths)
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
        let result = build_articles();

        assert!(result.is_ok());
    }

    #[test]
    fn test_collect_article_ok() {
        let path = "/diary/**/*.md";
        let result = collect_paths(path);

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
}
