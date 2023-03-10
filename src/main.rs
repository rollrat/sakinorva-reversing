use std::collections::HashMap;

use scraper::{Html, Selector};

#[derive(Debug)]
struct Question {
    code: String,
    description: String,
}

#[tokio::main]
async fn main() {
    let qs = load_questions().await;

    for q in qs.into_iter() {
        println!("{} {}", q.code, q.description);
    }

    post_functions([("q1", 5)].iter().cloned().collect::<HashMap<&str, i32>>()).await;
}

async fn load_questions() -> Vec<Question> {
    let mut result = Vec::new();

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

        result.push(Question {
            code: String::from(x.value().attr("name").unwrap()),
            description: String::from(y.text().skip(1).next().unwrap()),
        })
    }

    result.sort_by(|a, b| a.code.cmp(&b.code));

    result
}

async fn post_functions(query: HashMap<&str, i32>) {
    let client = reqwest::Client::new();
    let res = client
        .post("https://sakinorva.net/functions?lang=kr")
        .body(format!(
            "{}&{}",
            query
                .iter()
                .map(|x| format!("{}={}", x.0, x.1))
                .collect::<Vec<String>>()
                .join("&"),
            "cons=0&age=&idmbti=&enneagram=&comments=&submit=%28%EA%B2%B0%EA%B3%BC+%EC%A0%9C%EC%B6%9C%29"
        ))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await;

    println!("{}", res.unwrap().text().await.unwrap());
}
