use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Joke {
    pub joke: String,
}

#[derive(Serialize, Deserialize)]
pub struct TriviaQuestions {
    response_code: u32,
    pub results: Vec<Trivia>,
}

#[derive(Serialize, Deserialize)]
pub struct Trivia {
    category: String,
    #[serde(rename = "type")]
    typ: String,
    pub question: String,
    pub correct_answer: String,
    pub incorrect_answers: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Excuse {
    id: String,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChuckNorris {
    categories: Vec<String>,
    id: String,
    icon_url: String,
    updated_at: String,
    url: String,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct Trump {
    pub message: String,
}

pub struct Movie {
    pub title: String,
    pub synopsis: String
}
