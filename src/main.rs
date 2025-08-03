use reqwest::blocking::Client;
use serde_json::json;

#[derive(Debug)]
pub struct ProblemInfo {
    pub name: String,
    pub url: String,
    pub difficulty: Difficulty,
}

#[derive(Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    fn from_str(s: &str) -> Self {
        match s {
            "Easy" | "EASY" => Difficulty::Easy,
            "Medium" | "MEDIUM" => Difficulty::Medium,
            "Hard" | "HARD" => Difficulty::Hard,
            _ => panic!("Unknown difficulty: {}", s),
        }
    }
}

fn main() {
    let client = Client::new();
    let url = "https://leetcode.com/graphql";

    let body = json!({
        "operationName": "problemsetQuestionListV2",
        "variables": {
            "categorySlug": "",
            "skip": 0,
            "limit": 50
        },
        "query": "query problemsetQuestionListV2($categorySlug: String, $skip: Int, $limit: Int) {
            problemsetQuestionListV2(categorySlug: $categorySlug, skip: $skip, limit: $limit) {
                questions {
                    title
                    titleSlug
                    difficulty
                }
            }
        }"
    });

    let res = client
        .post(url)
        .json(&body)
        .send()
        .expect("Failed to send request");

    let json: serde_json::Value = res.json().expect("Failed to parse JSON from response");

    let questions = json["data"]["problemsetQuestionListV2"]["questions"]
        .as_array()
        .expect("Missing questions array");

    let problems: Vec<ProblemInfo> = questions
        .iter()
        .map(|q| ProblemInfo {
            name: q["title"].as_str().unwrap().to_string(),
            url: format!(
                "https://leetcode.com/problems/{}/",
                q["titleSlug"].as_str().unwrap()
            ),
            difficulty: Difficulty::from_str(q["difficulty"].as_str().unwrap()),
        })
        .collect();

    for p in problems {
        println!("{:?}", p);
    }
}
