use dotenv::dotenv;
use newsapi::{get_articles, Articles};
use std::error::Error;

mod theme;

fn render_articles(articles: &Articles) {
    let theme = theme::default();
    theme.print_text("# Top headlines\n\n");
    for i in &articles.articles {
        theme.print_text(&format!("`{}`", i.title));
        theme.print_text(&format!("`  | {}`", i.url));
        theme.print_text("\n\n");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv();

    let api_key = std::env::var("API_KEY")?;
    let url = format!(
        "{}{}",
        "https://newsapi.org/v2/top-headlines?sources=bbc-news&apiKey=", api_key
    );
    let articles = get_articles(&url)?;

    render_articles(&articles);

    Ok(())
}
