mod sakinorva {
    use rand::Rng;
    use serde::{Deserialize, Serialize};
    use std::{
        collections::HashMap,
        fmt::{self, format},
        fs, path,
    };

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

    // E N F J
    #[derive(Debug)]
    pub struct MbtiFitness(f32, f32, f32, f32);

    #[derive(Debug)]
    pub struct QuestionCode {
        code: Vec<u8>,
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

        pub fn parse_myers_letter_type_with_fitness(&self) -> MbtiFitness {
            let document = Html::parse_document(&self.raw_html);

            let binding = Selector::parse("#my_results td").unwrap();
            let myers_letter_type = document.select(&binding).last().unwrap();

            let mbti = myers_letter_type
                .text()
                .map(|x| String::from(x))
                .collect::<Vec<String>>();

            let fitness_selector = Selector::parse("font > font").unwrap();
            let letters = myers_letter_type
                .select(&fitness_selector)
                .map(|x| {
                    let fitness_raw =
                        &x.value().attr("style").unwrap()["color: rgba(255, 255, 255, ".len()..];

                    fitness_raw[..fitness_raw.len() - 1].parse().unwrap()
                })
                .collect::<Vec<f32>>();

            MbtiFitness(
                if mbti[0] == "E" {
                    letters[0]
                } else {
                    -letters[0]
                },
                if mbti[1] == "N" {
                    letters[1]
                } else {
                    -letters[1]
                },
                if mbti[2] == "F" {
                    letters[2]
                } else {
                    -letters[2]
                },
                if mbti[3] == "J" {
                    letters[3]
                } else {
                    -letters[3]
                },
            )
        }

        pub fn save_as_html(&self, file_name: String) {
            fs::write(format!("{}.html", file_name), &self.raw_html).unwrap();
        }
    }

    impl MbtiFitness {
        pub fn new(e: f32, n: f32, f: f32, j: f32) -> MbtiFitness {
            MbtiFitness(e, n, f, j)
        }

        pub fn diff_with(&self, other: &MbtiFitness) -> f32 {
            let p = self.0 * other.0 + self.1 * other.1 + self.2 * other.2 + self.3 * other.3;

            let cl = self.0 * self.0 + self.1 * self.1 + self.2 * self.2 + self.3 * self.3;
            let cr = other.0 * other.0 + other.1 * other.1 + other.2 * other.2 + other.3 * other.3;

            p / (cl.sqrt() * cr.sqrt())
        }

        pub fn to_string(&self) -> String {
            format!(
                "{}{}{}{}",
                if self.0 >= 0.0 { "E" } else { "I" },
                if self.1 >= 0.0 { "N" } else { "S" },
                if self.2 >= 0.0 { "F" } else { "T" },
                if self.3 >= 0.0 { "J" } else { "P" }
            )
        }
    }

    impl fmt::Display for MbtiFitness {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.to_string())
        }
    }

    impl QuestionCode {
        fn new() -> QuestionCode {
            QuestionCode { code: vec![0; 96] }
        }

        pub fn create_random_code() -> QuestionCode {
            let mut rng = rand::thread_rng();
            let mut result = QuestionCode::new();

            for i in 0..96 {
                result.code[i] = rng.gen_range(1..=5);
            }

            result
        }

        pub fn crossover(&self, other: &QuestionCode) -> QuestionCode {
            let mut rng = rand::thread_rng();
            let mut result = QuestionCode::new();

            let replacement_pos = rng.gen_range(0..96);

            for i in 0..replacement_pos {
                result.code[i] = self.code[i];
            }

            for i in replacement_pos..96 {
                result.code[i] = other.code[i];
            }

            result
        }

        pub fn mutate(&mut self, mutation_rate: f64) -> &QuestionCode {
            let mut rng = rand::thread_rng();

            for i in 0..96 {
                if rng.gen_bool(mutation_rate) {
                    self.code[i] = rng.gen_range(1..=5);
                }
            }

            self
        }

        pub fn to_query(&self) -> HashMap<String, i32> {
            let mut result = HashMap::new();

            for (pos, e) in self.code.iter().enumerate() {
                result.insert(format!("q{}", pos), *e as i32);
            }

            result
        }
    }

    #[derive(Debug)]
    pub enum GeneticFieldStrategy {
        Exp,
        Tangent,
    }

    #[derive(Debug)]
    pub struct GeneticField {
        target: MbtiFitness,
        strategy: GeneticFieldStrategy,
        codes: Vec<QuestionCode>,
        mutation_rate: f64,
        current_max_rate: f32,
        max_rate_change_count: usize,
    }

    impl GeneticField {
        pub fn new(
            target: MbtiFitness,
            strategy: GeneticFieldStrategy,
            mutation_rate: f64,
            population: i32,
        ) -> GeneticField {
            GeneticField {
                target,
                strategy,
                codes: (0..population)
                    .map(|_| QuestionCode::create_random_code())
                    .collect(),
                mutation_rate,
                current_max_rate: -1.0,
                max_rate_change_count: 0,
            }
        }

        pub async fn evolution(&mut self) {
            let mut fitnesses = Vec::new();

            for (pos, code) in self.codes.iter().enumerate() {
                let functions = post_functions(
                    code.to_query()
                        .iter()
                        .map(|(x, y)| (&x[..], *y))
                        .collect::<HashMap<&str, i32>>(),
                )
                .await;
                fitnesses.push(functions.parse_myers_letter_type_with_fitness());

                let rate = self.target.diff_with(fitnesses.last().unwrap());

                print!("{} ({: >8.5}) ", fitnesses.last().unwrap(), rate);

                if rate > self.current_max_rate {
                    functions.save_as_html(format!("{} ({})", self.max_rate_change_count, rate));

                    self.max_rate_change_count += 1;
                    self.current_max_rate = rate;
                }

                if pos % 10 == 9 {
                    println!("");
                }
            }

            let mut hunting_pool: Vec<usize> = Vec::new();

            for (pos, e) in fitnesses.iter().enumerate() {
                let diff = self.target.diff_with(e);
                let putup = match &self.strategy {
                    GeneticFieldStrategy::Exp => ((diff + 1.0) * 6.0).exp() as i32,
                    GeneticFieldStrategy::Tangent => {
                        ((diff * std::f32::consts::PI / 2.0).tan()) as i32
                    }
                };

                if putup >= 0 {
                    (0..putup).for_each(|_| hunting_pool.push(pos));
                }
            }

            let mut result_codes: Vec<QuestionCode> = Vec::new();
            let mut rng = rand::thread_rng();

            for _ in 0..self.codes.len() {
                let p1 = hunting_pool[rng.gen_range(0..hunting_pool.len())];
                let p2 = hunting_pool[rng.gen_range(0..hunting_pool.len())];
                let mut n_code = self.codes[p1].crossover(&self.codes[p2]);
                n_code.mutate(self.mutation_rate);
                result_codes.push(n_code);
            }

            self.codes = result_codes;
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

                // if qs == "q47" || qs == "q69" {
                //     continue;
                // }

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
    let mut pool = GeneticField::new(
        // ENFJ
        MbtiFitness::new(1.0, 1.0, 1.0, 1.0),
        GeneticFieldStrategy::Tangent,
        0.01,
        100,
    );

    for i in 0..100 {
        println!("Loop {}", i + 1);
        pool.evolution().await;
    }
}
