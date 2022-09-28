use serde::Deserialize;
use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum NewsApiError {
    #[error("Failed fetching articles")]
    RequestFailed,

    #[error("Failed converting response to string")]
    FailedResponseToString,

    #[error("Article Parsing failed")]
    ArticleParseFailed,
}

#[derive(Deserialize, Debug)]
pub struct Articles {
    pub articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
pub struct Article {
    pub title: String,
    pub url: String,
}

pub fn get_articles(url: &str) -> Result<Articles, NewsApiError> {
    let response = ureq::get(url)
        .call()
        .map_err(|_| NewsApiError::RequestFailed)?
        .into_string()
        .map_err(|_| NewsApiError::FailedResponseToString)?;

    let articles: Articles =
        serde_json::from_str(&response).map_err(|_| NewsApiError::ArticleParseFailed)?;

    Ok(articles)
}
