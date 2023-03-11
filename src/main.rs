mod sakinorva {
    use serde::{Deserialize, Serialize};
    use std::{collections::HashMap, fs, path};

    use scraper::{Html, Selector};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Question {
        pub code: String,
        pub description: String,
        pub feature: Option<String>,
    }

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

    pub async fn load_questions() -> Vec<Question> {
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

    pub async fn load_questions_with_feature() -> Vec<Question> {
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

    pub async fn post_functions(query: HashMap<&str, i32>) -> FunctionsInfo {
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

        let txt = res.text().await.unwrap().replace(
            "/imgarchive/mute_typology.png",
            "https://sakinorva.net/imgarchive/mute_typology.png",
        );

        fs::write("last.html", &txt).unwrap();

        FunctionsInfo::new(txt)
    }

    #[derive(Debug)]
    pub struct Features {
        pub ti: i32,
        pub te: i32,
        pub si: i32,
        pub se: i32,
        pub ni: i32,
        pub ne: i32,
        pub fi: i32,
        pub fe: i32,
    }

    pub async fn get_question_feature_inverse_table<'a>() -> HashMap<String, Vec<String>> {
        let qf = load_questions_with_feature().await;
        let mut m: HashMap<String, Vec<String>> = HashMap::new();

        for q in qf {
            m.entry(q.feature.unwrap()).or_default().push(q.code);
        }

        m
    }

    pub async fn create_query_from_features<'a>(feat: Features) -> HashMap<String, i32> {
        let inv_table = get_question_feature_inverse_table().await;
        let mut result: HashMap<String, i32> = HashMap::new();

        let mut insert_questions = |feature: &str, score: i32| {
            if score < -12 || score > 12 {
                panic!("Adjust specific feature to [-12, 12]!");
            }

            let apply_score = if score > 0 { 5 } else { 1 };

            let mut remain_score = score.abs();

            for qs in inv_table.get(feature).unwrap().iter() {
                if remain_score == 0 {
                    break;
                }

                if qs == "q47" || qs == "q69" {
                    continue;
                }

                result.insert(qs.clone(), apply_score);

                remain_score -= 1
            }
        };

        insert_questions("Ti", feat.ti);
        insert_questions("Te", feat.te);
        insert_questions("Si", feat.si);
        insert_questions("Se", feat.se);
        insert_questions("Ni", feat.ni);
        insert_questions("Ne", feat.ne);
        insert_questions("Fi", feat.fi);
        insert_questions("Fe", feat.fe);

        result
    }

    pub async fn get_functions_from_features(feat: Features) -> FunctionsInfo {
        post_functions(
            create_query_from_features(feat)
                .await
                .iter()
                .map(|(x, y)| (&x[..], *y))
                .collect::<HashMap<&str, i32>>(),
        )
        .await
    }

    pub async fn print_irregular_questions() {
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
            let feature: Vec<(&String, &f32)> =
                parse_result.iter().filter(|x| x.1.ne(&23.0)).collect();

            if feature.len() > 1 {
                println!("{}, {:#?}", q.code, &parse_result);
            }
        }
    }
}

use sakinorva::*;

#[tokio::main]
async fn main() {
    let mbti = get_functions_from_features(Features {
        ti: 0,
        te: 0,
        si: -12,
        se: 0,
        ni: 0,
        ne: 12,
        fi: 12,
        fe: 12,
    })
    .await;

    println!("{:#?}", mbti.parse_features());
    println!("{}", mbti.parse_myers_letter_type());
}
