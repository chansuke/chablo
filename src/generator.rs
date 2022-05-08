use anyhow::Result;
use askama::Template;

use crate::models::{Article, ArticleTemplate, TopPage, TopPageTemplate, DESCRIPTION, SUB_TITLE};

pub trait Generator<T> {
    fn generate(self) -> Result<String, askama::Error>;
}

impl Generator<Article> for Article {
    fn generate(self) -> Result<String, askama::Error> {
        let result = ArticleTemplate {
            title: &self.title,
            body: self.body,
            date: self.date,
            description: DESCRIPTION,
        };

        result.render()
    }
}

impl Generator<TopPage> for TopPage {
    fn generate(self) -> Result<String, askama::Error> {
        let result = TopPageTemplate {
            title: SUB_TITLE,
            articles: self.articles,
            description: DESCRIPTION,
        };

        result.render()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::HtmlBody;

    #[test]
    fn test_generate_article_ok() {
        let date = curent_datetime();
        let content = "This is a test".to_string();
        let path = "filepath.html".to_string();

        let article = Article {
            id: "id".to_string(),
            title: "This is an article".to_string(),
            body: HtmlBody(content),
            date,
            path,
        };

        let result = article.generate();

        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_toppage_ok() {
        let date = curent_datetime();
        let content = "This is a test".to_string();
        let path = "filepath.html".to_string();
        let article = Article {
            id: "test".to_string(),
            title: "title".to_string(),
            body: HtmlBody(content),
            date,
            path,
        };
        let articles: Vec<Article> = vec![article];

        let toppage = TopPage { articles };

        let result = toppage.generate();

        assert!(result.is_ok());
    }
}
