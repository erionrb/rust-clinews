use colour::{dark_blue, e_dark_magenta};
use dotenv::dotenv;
use newsapi::{get_articles, Articles};
use std::error::Error;

fn render_articles(articles: &Articles) {
    for i in &articles.articles {
        dark_blue!("> {}\n", i.title);
        e_dark_magenta!("> {}\n", i.url);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;

    let api_key = std::env::var("API_KEY")?;

    let url = format!(
        "{}{}",
        "https://newsapi.org/v2/top-headlines?sources=bbc-news&apiKey=", api_key
    );
    let articles = get_articles(&url)?;

    render_articles(&articles);

    Ok(())
}
