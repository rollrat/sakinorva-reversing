use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path};

use scraper::{Html, Selector};

#[derive(Debug, Serialize, Deserialize)]
struct Question {
    code: String,
    description: String,
    feature: Option<String>,
}

mod sakinorva {
    use std::collections::HashMap;

    use scraper::{Html, Selector};

    #[derive(Debug)]
    pub struct FunctionsInfo {
        raw_html: String,
    }

    impl FunctionsInfo {
        pub fn new(raw_html: String) -> FunctionsInfo {
            FunctionsInfo { raw_html }
        }

        pub fn parse_features(&self) -> HashMap<String, f32> {
            let mut result = HashMap::new();

            let document = Html::parse_document(&self.raw_html);

            let binding = Selector::parse("#my_results td").unwrap();
            let mut iter = document.select(&binding).take(16);

            while let Some(x) = iter.next() {
                if let Some(y) = iter.next() {
                    result.insert(
                        String::from(&x.text().next().unwrap()[0..2]),
                        y.text().next().unwrap().parse::<f32>().unwrap(),
                    );
                }
            }

            result
        }

        pub fn parse_myers_letter_type(&self) -> String {
            let document = Html::parse_document(&self.raw_html);

            let binding = Selector::parse("#my_results td").unwrap();

            String::from(
                document
                    .select(&binding)
                    .last()
                    .unwrap()
                    .text()
                    .map(|x| String::from(x))
                    .collect::<Vec<String>>()
                    .join(""),
            )
        }
    }
}

use sakinorva::*;

#[tokio::main]
async fn main() {
    let qf = load_questions_with_feature().await;
    let mut m: HashMap<&str, i32> = HashMap::new();

    for q in &qf {
        let f = q.feature.as_ref().unwrap();
        *m.entry(&f[..]).or_default() += 1;
    }

    println!("{:#?}", m);

    let f = post_functions([("q1", 5)].iter().cloned().collect::<HashMap<&str, i32>>()).await;

    println!("{}", f.parse_myers_letter_type());
}

async fn load_questions() -> Vec<Question> {
    if path::Path::new("questions.json").exists() {
        let q = fs::read_to_string("questions.json").unwrap();

        return serde_json::from_str(&q).unwrap();
    }

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
            feature: None,
        })
    }

    assert_eq!(result.len(), 96);

    result.sort_by(|a, b| a.code.cmp(&b.code));

    let questions = serde_json::to_string_pretty(&result).unwrap();
    fs::write("questions.json", questions).unwrap();

    result
}

async fn load_questions_with_feature() -> Vec<Question> {
    if path::Path::new("questions-features.json").exists() {
        let q = fs::read_to_string("questions-features.json").unwrap();

        return serde_json::from_str(&q).unwrap();
    }

    let mut qs = load_questions().await;

    for q in &mut qs {
        let f = post_functions(
            [(&q.code[..], 5)]
                .iter()
                .cloned()
                .collect::<HashMap<&str, i32>>(),
        )
        .await;

        let parse_result = f.parse_features();
        let feature = parse_result
            .iter()
            .max_by(|x, y| x.1.total_cmp(y.1))
            .unwrap()
            .0;

        println!("{}, {}", q.code, &feature);

        q.feature = Some(feature.clone());
    }

    let questions = serde_json::to_string_pretty(&qs).unwrap();
    fs::write("questions-features.json", questions).unwrap();

    qs
}

async fn post_functions(query: HashMap<&str, i32>) -> FunctionsInfo {
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
        .await
        .unwrap();

    FunctionsInfo::new(res.text().await.unwrap())
}
