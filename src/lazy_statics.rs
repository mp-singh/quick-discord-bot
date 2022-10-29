use std::{
    collections::{HashMap, HashSet},
    env,
};

use lazy_static::lazy_static;
use maplit::{hashmap, hashset};
use regex::Regex;
use reqwest::redirect;

use crate::utils::read_dir;

lazy_static! {
    pub static ref REQESUT: reqwest::Client = reqwest::Client::builder()
        .redirect(redirect::Policy::none())
        .build()
        .unwrap();
    pub static ref DO_CONFIGURATION: Configuration {
        bearer_access_token: env::var("DO_TOKEN").expect("token"),
        client: REQESUT
        ..Default::default()
    };
    pub static ref NASA_API_KEY: String = env::var("NASA_API_KEY").unwrap();
    pub static ref REGEX_DICE: Regex = Regex::new(r"^([1-9][0-9]?|100)[Dd]([1-9]\d*)$").unwrap();
    pub static ref HARDLY: Regex = Regex::new(r"(\w{2,}(?:[aeiou]r|re))(?:\W|$)").unwrap();
    pub static ref MOVIE1: Regex = Regex::new(r"<([a-zA-Z]+)([0-9]+)?>").unwrap();
    pub static ref MOVIE_SYNOPSIS1: Regex = Regex::new(r"a\s([aeiou])").unwrap();
    pub static ref MOVIE_SYNOPSIS2: Regex = Regex::new(r"([.?!]\s+)([a-z])").unwrap();
    pub static ref MOVIE_CONTENTS: HashMap<String, Vec<String>> = read_dir(String::from("lists"));
    pub static ref TRANSFORMATION_TYPES: HashMap<&'static str, Vec<f32>> = hashmap! {
        "laplacian" => vec![0.0, 1.0, 0.0, 1.0, -4.0, 1.0, 0.0, 1.0, 0.0],
        "sobel" => vec![-1.0, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0],
        "prewitt" => vec![-1.0, 0.0, 1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0],
        "roberts" => vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0],
        "scharr" => vec![-3.0, 0.0, 3.0, -10.0, 0.0, 10.0, -3.0, 0.0, 3.0],
        "laplacian_of_gaussian" => vec![0.0, 0.0, 1.0, 2.0, 1.0, 0.0, 0.0, 0.0, 0.0],
        "gaussian" => vec![1.0, 2.0, 1.0, 2.0, 4.0, 2.0, 1.0, 2.0, 1.0],
        "unsharp_mask" => vec![0.0, -1.0, 0.0, -1.0, 5.0, -1.0, 0.0, -1.0, 0.0],
    };
    pub static ref BLACK_LIST: HashSet<&'static str> = hashset![
        "another",
        "other",
        "tenor",
        "before",
        "never",
        "over",
        "youre",
        "fairer",
        "after",
        "everywhere",
        "ever",
        "hardware",
        "software",
        "anywhere",
        "super",
        "order"
    ];
}
