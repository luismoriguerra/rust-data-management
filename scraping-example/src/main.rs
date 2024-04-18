use scraper::{Html, Selector};
use select::document::Document;
use select::predicate::{Class, Name, Predicate};

async fn get_ycon_titles() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://news.ycombinator.com/";
    let html = reqwest::get(url).await?.text().await?;

    let document = Document::from_read(html.as_bytes())?;

    for (n, node) in document.find(Class("athing")).enumerate() {
        let story = node
            .find(Class("title").descendant(Name("a")))
            .next()
            .unwrap()
            .text();
        println!("{}. {}", n + 1, story);
    }

    Ok(())
}

async fn get_ycon_with_scaper() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://news.ycombinator.com/";
    let html = reqwest::get(url).await?.text().await?;

    let doc = Html::parse_document(&html);

    let stories = Selector::parse(".athing").unwrap();
    let titles = Selector::parse("td.title").unwrap();
    let a_tag = Selector::parse("a").unwrap();

    for story in doc.select(&stories) {
        let title = story.select(&titles).skip(1).next().unwrap();
        let a = title.select(&a_tag).next().unwrap();
        let story_title = a.text().collect::<Vec<_>>();
        println!("{}", story_title[0]);
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    get_ycon_titles().await.unwrap();

    get_ycon_with_scaper().await.unwrap();
}
