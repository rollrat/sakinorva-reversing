use scraper::{Html, Selector};

#[tokio::main]
async fn main() {
    let html = reqwest::get("https://sakinorva.net/functions?lang=kr")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let document = Html::parse_document(&html);
    let id_selector = Selector::parse("input").unwrap();
    let q_selector = Selector::parse("tr > td").unwrap();

    for e in document
        .select(&Selector::parse("body > form > table:nth-child(9) > tbody > tr").unwrap())
        .skip(1)
        .step_by(2)
    {
        let x = e.select(&id_selector).next().unwrap();
        let y = e.select(&q_selector).next().unwrap();

        println!(
            "{}, {}",
            x.value().attr("name").unwrap(),
            y.text().skip(1).next().unwrap()
        );
    }
}
