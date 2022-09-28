use colour::{dark_blue, e_dark_magenta};
use newsapi::{get_articles, Articles};
use std::error::Error;

fn render_articles(articles: &Articles) {
    for i in &articles.articles {
        dark_blue!("> {}\n", i.title);
        e_dark_magenta!("> {}\n", i.url);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://newsapi.org/v2/top-headlines?sources=bbc-news&apiKey=abc29fcdb38f451b9e69aa34f22162e7";
    let articles = get_articles(url)?;

    render_articles(&articles);

    Ok(())
}
