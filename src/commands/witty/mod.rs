use serenity::{model::channel::Message, prelude::Mentionable};

use crate::{
    lazy_statics::{BLACK_LIST, HARDLY},
    utils::syllables::count_syllables,
};

#[allow(dead_code)]
pub fn hardly(msg: &str) -> Option<String> {
    for cap in HARDLY.captures_iter(msg) {
        let word = cap.get(1).unwrap().as_str();
        if count_syllables(word) > 1 && !BLACK_LIST.contains(word.to_lowercase().as_str()) {
            return Some(format!(
                "{}{}? I hardly know her!",
                word[0..1].to_uppercase(),
                &word[1..]
            ));
        }
    }
    None
}

pub fn shirley(msg: &str) -> Option<String> {
    match msg.to_lowercase().contains("surely") {
        true => Some("Don't call me Shirley!".to_string()),
        false => None,
    }
}

pub fn thanks(msg: &Message) -> Option<String> {
    let content = msg.content.to_lowercase();
    match content.contains("thank") || content.contains("thx") {
        true => Some(format!("No, thank you {}!", msg.author.mention())),
        false => None,
    }
}
